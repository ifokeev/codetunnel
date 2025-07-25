use serde::{Deserialize, Serialize};
use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::{Emitter, State, Manager};
use rand::Rng;
use std::net::TcpListener;
use std::io::{BufRead, BufReader};
use std::thread;
use std::path::PathBuf;
use std::fs;

#[derive(Default)]
struct AppState {
    ttyd_process: Option<Child>,
    cloudflared_process: Option<Child>,
    terminal_info: Option<TerminalInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TerminalInfo {
    url: String,
    username: String,
    password: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
struct TerminalStatus {
    running: bool,
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
    port: Option<u16>,
}

fn get_binary_path(app_handle: &tauri::AppHandle, binary_name: &str) -> Result<PathBuf, String> {
    let resource_path = app_handle
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;
    
    let platform_dir = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    
    let binary_filename = if cfg!(target_os = "windows") {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    };
    
    let binary_path = resource_path
        .join("resources")
        .join(platform_dir)
        .join(binary_filename);
    
    // Check if binary exists
    if !binary_path.exists() {
        return Err(format!("Binary {} not found at {:?}. Please ensure the binary is included in the resources/{} directory.", 
            binary_name, binary_path, platform_dir));
    }
    
    // On Unix systems, ensure the binary is executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&binary_path)
            .map_err(|e| format!("Failed to get metadata for binary: {}", e))?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&binary_path, permissions)
            .map_err(|e| format!("Failed to set executable permissions: {}", e))?;
    }
    
    Ok(binary_path)
}

#[tauri::command]
async fn start_terminal(
    state: State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) -> Result<TerminalInfo, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    
    // Check if already running
    if state.ttyd_process.is_some() {
        return Err("Terminal is already running".to_string());
    }
    
    // Get binary paths
    let ttyd_path = get_binary_path(&app_handle, "ttyd")?;
    let cloudflared_path = get_binary_path(&app_handle, "cloudflared")?;
    
    // Generate random port
    let port = get_random_port()?;
    
    // Generate random 6-digit password and username
    let password = generate_password();
    let username = generate_username();
    
    // Start ttyd process
    let shell = if cfg!(target_os = "windows") {
        "cmd.exe"
    } else if cfg!(target_os = "macos") {
        "/bin/zsh"
    } else {
        "/bin/bash"
    };
    
    let ttyd_cmd = Command::new(&ttyd_path)
        .args(&[
            "-p", &port.to_string(),
            "-i", "0.0.0.0",  // Bind to all interfaces
            "-c", &format!("{}:{}", username, password),
            "-t", "theme={\"background\": \"#000\"}",
            "-W",  // Writable
            shell
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();
    
    let mut ttyd_process = ttyd_cmd.map_err(|e| format!("Failed to start ttyd: {}", e))?;
    
    println!("Started ttyd on port {}", port);
    
    // Give ttyd more time to start and bind to the port
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Check if ttyd is still running
    if let Ok(Some(status)) = ttyd_process.try_wait() {
        return Err(format!("ttyd exited immediately with status: {:?}", status));
    }
    
    // Verify ttyd is listening on the port by trying to connect
    let max_attempts = 10;
    let mut ttyd_ready = false;
    for i in 0..max_attempts {
        match std::net::TcpStream::connect(format!("127.0.0.1:{}", port)) {
            Ok(_) => {
                // Successfully connected, ttyd is listening
                println!("Verified ttyd is listening on port {} (attempt {})", port, i + 1);
                ttyd_ready = true;
                break;
            }
            Err(_) => {
                // Can't connect yet, wait a bit more
                if i == max_attempts - 1 {
                    let _ = ttyd_process.kill();
                    return Err("ttyd failed to start listening on port".to_string());
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
    
    if !ttyd_ready {
        let _ = ttyd_process.kill();
        return Err("ttyd is not responding on the expected port".to_string());
    }
    
    // Start cloudflared tunnel with output capture
    println!("Starting cloudflared tunnel for http://localhost:{}", port);
    let cloudflared_cmd = Command::new(&cloudflared_path)
        .args(&[
            "tunnel",
            "--no-autoupdate",
            "--url",
            &format!("http://localhost:{}", port)
        ])
        .stderr(std::process::Stdio::piped())
        .spawn();
    
    let mut cloudflared_process = cloudflared_cmd
        .map_err(|e| format!("Failed to start cloudflared: {}", e))?;
    
    // Capture cloudflared output to get the tunnel URL
    let stderr = cloudflared_process.stderr.take().ok_or("Failed to capture cloudflared output")?;
    
    // Parse cloudflared output in a separate thread
    let url_receiver = thread::spawn(move || -> Result<String, String> {
        let reader = BufReader::new(stderr);
        let mut tunnel_url = String::new();
        let mut connection_ready = false;
        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(30);
        
        for line in reader.lines() {
            if start_time.elapsed() > timeout {
                return Err("Timeout waiting for tunnel URL".to_string());
            }
            if let Ok(line) = line {
                println!("Cloudflared: {}", line);
                
                // Look for the tunnel URL in the output
                if line.contains("https://") && line.contains(".trycloudflare.com") {
                    if let Some(start) = line.find("https://") {
                        if let Some(end) = line[start..].find(' ') {
                            tunnel_url = line[start..start + end].to_string();
                        } else {
                            tunnel_url = line[start..].to_string();
                        }
                        println!("Found tunnel URL: {}", tunnel_url);
                    }
                }
                
                // Check if tunnel connection is registered
                if line.contains("Connection") && line.contains("registered") {
                    println!("Connection registered!");
                    connection_ready = true;
                    // Wait a bit more to ensure tunnel is fully established
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    if !tunnel_url.is_empty() {
                        break;
                    }
                }
                
                // Also check for "Starting metrics server" as an indicator the tunnel is ready
                if line.contains("Starting metrics server") && !tunnel_url.is_empty() {
                    println!("Tunnel is ready (metrics server started)!");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    break;
                }
            }
        }
        
        if tunnel_url.is_empty() {
            Err("Failed to get tunnel URL from cloudflared".to_string())
        } else {
            Ok(tunnel_url)
        }
    });
    
    // Wait for tunnel URL (with timeout)
    let url = match url_receiver.join() {
        Ok(Ok(url)) => url,
        Ok(Err(e)) => {
            // Clean up processes
            let _ = ttyd_process.kill();
            let _ = cloudflared_process.kill();
            return Err(e);
        }
        Err(_) => {
            // Clean up processes
            let _ = ttyd_process.kill();
            let _ = cloudflared_process.kill();
            return Err("Failed to parse cloudflared output".to_string());
        }
    };
    
    let terminal_info = TerminalInfo {
        url: url.clone(),
        username: username.clone(),
        password: password.clone(),
        port,
    };
    
    // Update state
    state.ttyd_process = Some(ttyd_process);
    state.cloudflared_process = Some(cloudflared_process);
    state.terminal_info = Some(terminal_info.clone());
    
    // Emit status update
    app_handle.emit("terminal-status", TerminalStatus {
        running: true,
        url: Some(url),
        username: Some(username),
        password: Some(password),
        port: Some(port),
    }).map_err(|e| e.to_string())?;
    
    Ok(terminal_info)
}

#[tauri::command]
async fn stop_terminal(
    state: State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    
    // Kill ttyd process
    if let Some(mut process) = state.ttyd_process.take() {
        let _ = process.kill();
        let _ = process.wait();
    }
    
    // Kill cloudflared process
    if let Some(mut process) = state.cloudflared_process.take() {
        let _ = process.kill();
        let _ = process.wait();
    }
    
    // Clear terminal info
    state.terminal_info = None;
    
    // Emit status update
    app_handle.emit("terminal-status", TerminalStatus {
        running: false,
        url: None,
        username: None,
        password: None,
        port: None,
    }).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn get_status(state: State<'_, Mutex<AppState>>) -> Result<TerminalStatus, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    
    let running = state.ttyd_process.is_some();
    let status = if let Some(ref info) = state.terminal_info {
        TerminalStatus {
            running,
            url: Some(info.url.clone()),
            username: Some(info.username.clone()),
            password: Some(info.password.clone()),
            port: Some(info.port),
        }
    } else {
        TerminalStatus {
            running: false,
            url: None,
            username: None,
            password: None,
            port: None,
        }
    };
    
    Ok(status)
}

fn get_random_port() -> Result<u16, String> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to bind to random port: {}", e))?;
    let port = listener.local_addr()
        .map_err(|e| format!("Failed to get local address: {}", e))?
        .port();
    drop(listener);
    Ok(port)
}

fn generate_password() -> String {
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect::<String>()
}

fn generate_username() -> String {
    let mut rng = rand::thread_rng();
    let adjectives = vec![
        "quick", "bright", "calm", "brave", "cool", "smart", "swift", "bold", "keen", "wise",
        "fair", "kind", "warm", "glad", "neat", "pure", "safe", "clear", "fresh", "light"
    ];
    let nouns = vec![
        "fox", "wolf", "bear", "hawk", "deer", "owl", "lynx", "seal", "crow", "dove",
        "lion", "tiger", "eagle", "shark", "whale", "otter", "raven", "heron", "finch", "swan"
    ];
    
    let adjective = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];
    let number = rng.gen_range(10..99);
    
    format!("{}{}{}", adjective, noun, number)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            start_terminal,
            stop_terminal,
            get_status
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                // Clean up processes when window is closed
                if let Some(app_handle) = window.app_handle().try_state::<Mutex<AppState>>() {
                    if let Ok(mut state) = app_handle.lock() {
                        if let Some(mut process) = state.ttyd_process.take() {
                            let _ = process.kill();
                        }
                        if let Some(mut process) = state.cloudflared_process.take() {
                            let _ = process.kill();
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}