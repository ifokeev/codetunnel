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
    
    // Debug: print the resource path
    println!("Resource path: {:?}", resource_path);
    println!("Looking for binary at: {:?}", binary_path);
    
    // Check if binary exists
    if !binary_path.exists() {
        // Try to list what's in the directory
        let parent = binary_path.parent();
        if let Some(p) = parent {
            println!("Directory contents of {:?}:", p);
            if let Ok(entries) = fs::read_dir(p) {
                for entry in entries {
                    if let Ok(e) = entry {
                        println!("  - {:?}", e.path());
                    }
                }
            }
        }
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
    
    // Generate a random base path that acts as a secret token
    let secret_token = generate_secret_token();
    
    // Start ttyd process
    let shell = if cfg!(target_os = "windows") {
        "cmd.exe"
    } else if cfg!(target_os = "macos") {
        "/bin/zsh"
    } else {
        "/bin/bash"
    };
    
    // Run ttyd with a secret base path for security
    // This acts as authentication since only those who know the path can connect
    let ttyd_cmd = Command::new(&ttyd_path)
        .args(&[
            "--writable",
            "--port", &port.to_string(),
            "--base-path", &format!("/{}", secret_token),
            shell
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();
    
    let mut ttyd_process = ttyd_cmd.map_err(|e| format!("Failed to start ttyd: {}", e))?;
    
    // Spawn threads to capture and print ttyd output
    if let Some(stdout) = ttyd_process.stdout.take() {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("ttyd stdout: {}", line);
                }
            }
        });
    }
    
    if let Some(stderr) = ttyd_process.stderr.take() {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("ttyd stderr: {}", line);
                }
            }
        });
    }
    
    println!("Started ttyd on port {}", port);
    
    // Give ttyd time to start and bind to the port
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Check if ttyd is still running
    if let Ok(Some(status)) = ttyd_process.try_wait() {
        return Err(format!("ttyd exited immediately with status: {:?}", status));
    }
    
    // Start cloudflared tunnel with output capture
    println!("Starting cloudflared tunnel for http://localhost:{}", port);
    let cloudflared_cmd = Command::new(&cloudflared_path)
        .args(&[
            "tunnel",
            "--loglevel", "debug",
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
    let (url_sender, url_receiver) = std::sync::mpsc::channel();
    
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        let mut tunnel_url_found = false;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("Cloudflared: {}", line);
                
                // Look for the tunnel URL in the output
                if !tunnel_url_found && line.contains("https://") && line.contains(".trycloudflare.com") {
                    if let Some(start) = line.find("https://") {
                        let tunnel_url = if let Some(end) = line[start..].find(' ') {
                            line[start..start + end].to_string()
                        } else {
                            line[start..].to_string()
                        };
                        println!("Found tunnel URL: {}", tunnel_url);
                        tunnel_url_found = true;
                        // Send URL but continue reading to prevent SIGPIPE
                        let _ = url_sender.send(tunnel_url);
                    }
                }
            }
        }
        println!("Cloudflared stderr reader thread ending");
    });
    
    // Wait for tunnel URL (with timeout)
    let url = match url_receiver.recv_timeout(std::time::Duration::from_secs(30)) {
        Ok(url) => {
            // Wait a bit for DNS propagation
            std::thread::sleep(std::time::Duration::from_secs(2));
            url
        }
        Err(_) => {
            // Clean up processes
            let _ = ttyd_process.kill();
            let _ = cloudflared_process.kill();
            return Err("Failed to get tunnel URL from cloudflared".to_string());
        }
    };
    
    // Append the secret path to the URL
    let full_url = format!("{}/{}", url, secret_token);
    
    let terminal_info = TerminalInfo {
        url: full_url.clone(),
        username: username.clone(),
        password: password.clone(),
        port,
    };
    
    // Update state
    state.ttyd_process = Some(ttyd_process);
    state.cloudflared_process = Some(cloudflared_process);
    state.terminal_info = Some(terminal_info.clone());
    
    // Spawn a thread to monitor process health
    let app_handle_clone = app_handle.clone();
    thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            
            // Check if processes are still running
            if let Ok(mut state) = app_handle_clone.state::<Mutex<AppState>>().lock() {
                let mut processes_failed = false;
                
                if let Some(ref mut ttyd) = state.ttyd_process {
                    if let Ok(Some(status)) = ttyd.try_wait() {
                        println!("ERROR: ttyd process exited with status: {:?}", status);
                        processes_failed = true;
                    }
                }
                
                if let Some(ref mut cloudflared) = state.cloudflared_process {
                    if let Ok(Some(status)) = cloudflared.try_wait() {
                        println!("ERROR: cloudflared process exited with status: {:?}", status);
                        processes_failed = true;
                    }
                }
                
                if processes_failed {
                    println!("CRITICAL: One or more processes have failed!");
                    break;
                }
            }
        }
    });
    
    // Emit status update
    app_handle.emit("terminal-status", TerminalStatus {
        running: true,
        url: Some(full_url),
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

fn generate_secret_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Generate a 32-character random token
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
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