import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import './App.css';

const LoadingOverlay = ({ message }) => (
  <div className="loading-overlay">
    <div className="spinner"></div>
    <p>{message}</p>
  </div>
);

const ConnectionDetails = ({ status, onCopy }) => (
  <div className="connection-panel">
    <h2>Connection Details</h2>
    <div className="connection-item">
      <label>URL</label>
      <div className="input-group">
        <input type="text" value={status.url} readOnly />
        <button className="copy-btn" onClick={() => onCopy(status.url)}>Copy</button>
      </div>
    </div>
    <div className="connection-item" style={{ 
      backgroundColor: 'rgba(255, 255, 255, 0.08)',
      padding: '12px',
      borderRadius: '6px',
      marginTop: '12px'
    }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '4px' }}>
        <span style={{ fontSize: '14px' }}>🔒</span>
        <label style={{ fontSize: '12px', margin: 0, fontWeight: '500', color: 'white' }}>Security</label>
      </div>
      <p style={{ 
        margin: 0, 
        color: 'rgba(255, 255, 255, 0.9)', 
        fontSize: '13px',
        lineHeight: '1.4'
      }}>
        Protected by 32-character token in URL. Share only with trusted users.
      </p>
    </div>
  </div>
);

function App() {
  const [status, setStatus] = useState({
    running: false,
    url: '',
    username: '',
    password: '',
    port: null
  });
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [loadingMessage, setLoadingMessage] = useState('');

  useEffect(() => {
    updateStatus();
    const unlisten = listen('terminal-status', (event) => {
      setStatus(event.payload);
    });
    return () => { unlisten.then(fn => fn()); };
  }, []);

  const updateStatus = async () => {
    try {
      const currentStatus = await invoke('get_status');
      setStatus(currentStatus);
    } catch (err) {
      showError(err);
    }
  };

  const startTerminal = async () => {
    try {
      setLoading(true);
      setLoadingMessage('Starting terminal...');
      setError('');
      
      const result = await invoke('start_terminal');
      
      setLoadingMessage('Creating secure tunnel...');
      setStatus(prev => ({ ...prev, ...result, running: true }));

    } catch (err) {
      showError(err);
      setStatus({ running: false, url: '', username: '', password: '', port: null });
    } finally {
      setLoading(false);
      setLoadingMessage('');
    }
  };

  const stopTerminal = async () => {
    try {
      setLoading(true);
      setLoadingMessage('Stopping terminal...');
      await invoke('stop_terminal');
      setStatus({ running: false, url: '', username: '', password: '', port: null });
    } catch (err) {
      showError(err);
    } finally {
      setLoading(false);
      setLoadingMessage('');
    }
  };

  const showError = (err) => {
    let errorMessage = err.toString();
    if (errorMessage.includes('Binary') && errorMessage.includes('not found')) {
      errorMessage += '\n\nPlease run: ./scripts/download-binaries.sh';
    }
    setError(errorMessage);
    setTimeout(() => setError(''), 8000);
  };

  const copyToClipboard = async (text) => {
    try {
      await navigator.clipboard.writeText(text);
    } catch (err) {
      showError('Failed to copy to clipboard');
    }
  };

  return (
    <div className="app-container">
      {loading && <LoadingOverlay message={loadingMessage} />}
      
      <header className="header">
        <img src="/src/assets/a1-shell-black.png" alt="A1 Shell" className="logo" />
        <div className="title-container">
          <h1>A1 Shell</h1>
          <p>Instant, secure web terminals</p>
        </div>
      </header>
      
      <main className="main-content">
        <div className="status-panel">
          <h2>Terminal Status</h2>
          <div className="status-indicator">
            <div className={`status-light ${status.running ? 'running' : 'stopped'}`}></div>
            <span>{status.running ? 'Running' : 'Not Running'}</span>
          </div>
        </div>
        
        {status.running && status.url && (
          <ConnectionDetails status={status} onCopy={copyToClipboard} />
        )}

        {error && (
          <div className="error-message">
            {error}
          </div>
        )}
      </main>
      
      <footer className="footer">
        <div className="controls">
          <button 
            className="btn btn-start"
            onClick={startTerminal}
            disabled={status.running || loading}
          >
            Start Terminal
          </button>
          <button 
            className="btn btn-stop"
            onClick={stopTerminal}
            disabled={!status.running || loading}
          >
            Stop Terminal
          </button>
        </div>
      </footer>
    </div>
  );
}

export default App;
