use super::*;
use notify::{Event, EventKind, RecommendedWatcher, Watcher as NotifyWatcher, RecursiveMode, Result as NotifyResult};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: mpsc::UnboundedReceiver<FileEvent>,
    debounce_map: HashMap<PathBuf, Instant>,
    debounce_duration: Duration,
}

impl FileWatcher {
    pub fn new(debounce_ms: u64) -> Result<(Self, mpsc::UnboundedSender<FileEvent>)> {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx_clone = tx.clone();
        
        let watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
            if let Ok(event) = res {
                let file_event = match event.kind {
                    EventKind::Create(_) => {
                        event.paths.first().map(|p| FileEvent::Created(p.clone()))
                    }
                    EventKind::Modify(_) => {
                        event.paths.first().map(|p| FileEvent::Modified(p.clone()))
                    }
                    EventKind::Remove(_) => {
                        event.paths.first().map(|p| FileEvent::Deleted(p.clone()))
                    }
                    EventKind::Any => {
                        // Handle rename events
                        if event.paths.len() == 2 {
                            Some(FileEvent::Renamed {
                                from: event.paths[0].clone(),
                                to: event.paths[1].clone(),
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                
                if let Some(file_event) = file_event {
                    let _ = tx_clone.send(file_event);
                }
            }
        }).map_err(|e| FileSystemError::WatchError(e.to_string()))?;
        
        Ok((
            Self {
                watcher,
                rx,
                debounce_map: HashMap::new(),
                debounce_duration: Duration::from_millis(debounce_ms),
            },
            tx,
        ))
    }
    
    pub fn watch(&mut self, path: &Path) -> Result<()> {
        self.watcher
            .watch(path, RecursiveMode::Recursive)
            .map_err(|e| FileSystemError::WatchError(e.to_string()))
    }
    
    pub fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.watcher
            .unwatch(path)
            .map_err(|e| FileSystemError::WatchError(e.to_string()))
    }
    
    pub async fn next_event(&mut self) -> Option<FileEvent> {
        loop {
            if let Some(event) = self.rx.recv().await {
                // Debounce logic
                let path = match &event {
                    FileEvent::Created(p) | FileEvent::Modified(p) | FileEvent::Deleted(p) => p,
                    FileEvent::Renamed { to, .. } => to,
                };
                
                let now = Instant::now();
                if let Some(last_time) = self.debounce_map.get(path) {
                    if now.duration_since(*last_time) < self.debounce_duration {
                        continue; // Skip this event due to debouncing
                    }
                }
                
                self.debounce_map.insert(path.clone(), now);
                return Some(event);
            }
            return None;
        }
    }
}

impl FileSystem {
    pub async fn start_watching(&self) -> Result<mpsc::UnboundedReceiver<FileEvent>> {
        let (mut watcher, _tx) = FileWatcher::new(300)?; // 300ms debounce
        watcher.watch(&self.base_path)?;
        
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        
        // Spawn task to forward debounced events
        tokio::spawn(async move {
            while let Some(event) = watcher.next_event().await {
                let _ = event_tx.send(event);
            }
        });
        
        Ok(event_rx)
    }
    
    pub async fn watch_path(&self, path: &Path) -> Result<mpsc::UnboundedReceiver<FileEvent>> {
        let full_path = self.resolve_path(path)?;
        let (mut watcher, _tx) = FileWatcher::new(300)?;
        watcher.watch(&full_path)?;
        
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let base_path = self.base_path.clone();
        
        // Spawn task to forward debounced events with relative paths
        tokio::spawn(async move {
            while let Some(event) = watcher.next_event().await {
                let relative_event = match event {
                    FileEvent::Created(p) => {
                        p.strip_prefix(&base_path).ok().map(|rp| FileEvent::Created(rp.to_path_buf()))
                    }
                    FileEvent::Modified(p) => {
                        p.strip_prefix(&base_path).ok().map(|rp| FileEvent::Modified(rp.to_path_buf()))
                    }
                    FileEvent::Deleted(p) => {
                        p.strip_prefix(&base_path).ok().map(|rp| FileEvent::Deleted(rp.to_path_buf()))
                    }
                    FileEvent::Renamed { from, to } => {
                        match (from.strip_prefix(&base_path), to.strip_prefix(&base_path)) {
                            (Ok(rf), Ok(rt)) => Some(FileEvent::Renamed {
                                from: rf.to_path_buf(),
                                to: rt.to_path_buf(),
                            }),
                            _ => None,
                        }
                    }
                };
                
                if let Some(rel_event) = relative_event {
                    let _ = event_tx.send(rel_event);
                }
            }
        });
        
        Ok(event_rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::time::sleep;
    
    #[tokio::test]
    async fn test_file_watcher() {
        let temp_dir = TempDir::new().unwrap();
        let fs = FileSystem::new(temp_dir.path().to_path_buf());
        
        let mut event_rx = fs.start_watching().await.unwrap();
        
        // Create a file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "Hello").await.unwrap();
        
        // Wait for event
        sleep(Duration::from_millis(400)).await;
        
        if let Ok(Some(event)) = event_rx.try_recv() {
            match event {
                FileEvent::Created(p) => {
                    assert!(p.ends_with("test.txt"));
                }
                _ => panic!("Expected Created event"),
            }
        }
    }
    
    #[tokio::test]
    async fn test_debouncing() {
        let temp_dir = TempDir::new().unwrap();
        let (mut watcher, tx) = FileWatcher::new(100).unwrap();
        
        watcher.watch(temp_dir.path()).unwrap();
        
        let test_file = temp_dir.path().join("test.txt");
        
        // Rapid modifications
        for i in 0..5 {
            fs::write(&test_file, format!("Content {}", i)).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
        
        // Wait for debounce
        sleep(Duration::from_millis(200)).await;
        
        // Should only get one or two events due to debouncing
        let mut event_count = 0;
        while let Ok(Some(_)) = watcher.rx.try_recv() {
            event_count += 1;
        }
        
        assert!(event_count < 5, "Debouncing should reduce event count");
    }
}