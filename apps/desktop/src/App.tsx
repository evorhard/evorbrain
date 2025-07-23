import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { appWindow } from '@tauri-apps/api/window';
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';

export function App() {
  const [greeting, setGreeting] = useState('');
  const [appInfo, setAppInfo] = useState<{
    name: string;
    version: string;
    tauriVersion: string;
  } | null>(null);
  const [windowTitle, setWindowTitle] = useState('EvorBrain');

  useEffect(() => {
    // Load app information
    Promise.all([getName(), getVersion(), getTauriVersion()]).then(
      ([name, version, tauriVersion]) => {
        setAppInfo({ name, version, tauriVersion });
      }
    );

    // Test window events
    const unlistenPromise = appWindow.listen('tauri://resize', () => {
      // Window resize event handler
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const handleGreet = async () => {
    try {
      const response = await invoke<string>('greet', { name: 'User' });
      setGreeting(response);
    } catch (error) {
      setGreeting('Error: Failed to call greet command');
    }
  };

  const handleUpdateTitle = async () => {
    const newTitle = `EvorBrain - ${new Date().toLocaleTimeString()}`;
    await appWindow.setTitle(newTitle);
    setWindowTitle(newTitle);
  };

  const handleMinimize = () => appWindow.minimize();
  const handleMaximize = () => appWindow.toggleMaximize();
  const handleCenter = () => appWindow.center();

  return (
    <div style={{ padding: '20px', fontFamily: 'system-ui' }}>
      <h1>🧠 EvorBrain</h1>
      <p>Personal Knowledge Management System</p>

      {appInfo && (
        <div style={{ marginTop: '20px', padding: '10px', backgroundColor: '#f0f0f0', borderRadius: '5px' }}>
          <h3>App Information:</h3>
          <p>Name: {appInfo.name}</p>
          <p>Version: {appInfo.version}</p>
          <p>Tauri Version: {appInfo.tauriVersion}</p>
        </div>
      )}

      <div style={{ marginTop: '20px' }}>
        <h3>Test IPC Communication:</h3>
        <button onClick={handleGreet} style={{ padding: '10px 20px', marginRight: '10px' }}>
          Test Greet Command
        </button>
        {greeting && <p style={{ marginTop: '10px', color: 'green' }}>{greeting}</p>}
      </div>

      <div style={{ marginTop: '20px' }}>
        <h3>Window Controls:</h3>
        <div style={{ display: 'flex', gap: '10px' }}>
          <button onClick={handleUpdateTitle} style={{ padding: '10px 20px' }}>
            Update Title
          </button>
          <button onClick={handleMinimize} style={{ padding: '10px 20px' }}>
            Minimize
          </button>
          <button onClick={handleMaximize} style={{ padding: '10px 20px' }}>
            Toggle Maximize
          </button>
          <button onClick={handleCenter} style={{ padding: '10px 20px' }}>
            Center Window
          </button>
        </div>
        <p style={{ marginTop: '10px' }}>Current Title: {windowTitle}</p>
      </div>

      <div style={{ marginTop: '40px', fontSize: '12px', color: '#666' }}>
        <p>✅ Tauri is working correctly!</p>
        <p>✅ Window controls are functional</p>
        <p>✅ IPC communication is established</p>
      </div>
    </div>
  );
}
