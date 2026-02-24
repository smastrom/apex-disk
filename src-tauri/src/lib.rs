// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub icon: Option<String>,
    /// Child folders, sorted by size descending. Enables recursive navigation.
    pub children: Vec<FolderInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct FolderScanProgress {
    current: usize,
    total: usize,
    folder: String,
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scanning: Option<String>,
}

/// Builds the full folder tree in a single filesystem pass to avoid redundant I/O.
/// Uses a stack to accumulate sizes and construct the tree as we walk depth-first.
fn get_user_folders_sync_with_progress(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    let user_dir = dirs::home_dir().ok_or("Unable to determine user directory")?;

    // Count top-level dirs for progress
    let total = std::fs::read_dir(&user_dir)
        .map_err(|e| format!("Failed to read user directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let ft = e.file_type().ok();
            e.path().is_dir() && ft.map(|f| !f.is_symlink()).unwrap_or(false)
        })
        .count();

    #[derive(Clone)]
    struct StackEntry {
        depth: usize,
        info: FolderInfo,
    }

    let mut stack: Vec<StackEntry> = Vec::new();
    let mut roots: Vec<FolderInfo> = Vec::new();
    let mut roots_completed = 0usize;

    for entry in walkdir::WalkDir::new(&user_dir)
        .min_depth(1)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !e.file_type().is_symlink())
        .filter_map(|e| e.ok())
    {
        let depth = entry.depth();
        let path = entry.path().to_path_buf();
        let is_dir = entry.file_type().is_dir();
        let is_symlink = entry.file_type().is_symlink();

        if is_dir && !is_symlink {
            // Pop finished directories (we've exited them — depth decreased or sibling)
            while stack.last().is_some_and(|e| e.depth >= depth) {
                let StackEntry { info, .. } = stack.pop().unwrap();
                if let Some(parent) = stack.last_mut() {
                    parent.info.size += info.size;
                    parent.info.children.push(info);
                    parent
                        .info
                        .children
                        .sort_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));
                } else {
                    let (name, size) = (info.name.clone(), info.size);
                    roots.push(info);
                    roots_completed += 1;
                    let _ = app.emit(
                        "folder-scan-progress",
                        &FolderScanProgress {
                            current: roots_completed,
                            total,
                            folder: name,
                            size,
                            scanning: None,
                        },
                    );
                }
            }

            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            stack.push(StackEntry {
                depth,
                info: FolderInfo {
                    name,
                    path: path.to_string_lossy().to_string(),
                    size: 0,
                    icon: None,
                    children: Vec::new(),
                },
            });
        } else if entry.file_type().is_file() && !is_symlink {
            if let Some(parent) = stack.last_mut() {
                parent.info.size += entry.metadata().map(|m| m.len()).unwrap_or(0);
            }
        }
    }

    // Pop remaining stack (in case walk ended inside a dir)
    while let Some(StackEntry { info, .. }) = stack.pop() {
        if let Some(parent) = stack.last_mut() {
            parent.info.size += info.size;
            parent.info.children.push(info);
            parent
                .info
                .children
                .sort_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));
        } else {
            let (name, size) = (info.name.clone(), info.size);
            roots.push(info);
            roots_completed += 1;
            let _ = app.emit(
                "folder-scan-progress",
                &FolderScanProgress {
                    current: roots_completed,
                    total,
                    folder: name,
                    size,
                    scanning: None,
                },
            );
        }
    }

    roots.sort_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name)));

    Ok(roots)
}

/// Runs the heavy disk I/O in a background thread so the UI stays responsive.
/// Emits `folder-scan-progress` events as each folder is scanned for real-time progress.
#[tauri::command]
async fn get_user_folders(app: tauri::AppHandle) -> Result<Vec<FolderInfo>, String> {
    tauri::async_runtime::spawn_blocking(move || get_user_folders_sync_with_progress(app))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_user_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
