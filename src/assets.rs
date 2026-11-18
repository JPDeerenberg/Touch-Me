//! Compile-time embedded game assets.
//!
//! Every sprite the game needs is baked into the executable with
//! `include_bytes!`. This means the shipped binaries (Windows `.exe`,
//! macOS `.app`, Linux tarball/`.deb`) run from any working directory and
//! never fail because a resource folder was not copied next to them.
//!
//! Loose files still win: `asset_candidates()` returns the on-disk locations
//! that are checked first, so players can drop a modified sprite next to the
//! executable (or in the repo root when running from source) to override the
//! built-in art.

use std::path::PathBuf;

/// Logical asset path -> embedded bytes.
///
/// Returns `None` for paths that are not compiled into the binary.
pub fn embedded(path: &str) -> Option<&'static [u8]> {
    let normalized = path.replace('\\', "/");
    Some(match normalized.as_str() {
        "players/red.png" => include_bytes!("../players/red.png"),
        "players/blue.png" => include_bytes!("../players/blue.png"),
        "players/green.png" => include_bytes!("../players/green.png"),
        "players/yellow.png" => include_bytes!("../players/yellow.png"),
        "images/sneakers.png" => include_bytes!("../images/sneakers.png"),
        "images/cloud.png" => include_bytes!("../images/cloud.png"),
        "images/ghost.png" => include_bytes!("../images/ghost.png"),
        "images/ice.png" => include_bytes!("../images/ice.png"),
        "images/shield.png" => include_bytes!("../images/shield.png"),
        "images/portal blue.png" => include_bytes!("../images/portal blue.png"),
        "images/portal green.png" => include_bytes!("../images/portal green.png"),
        _ => return None,
    })
}

/// Directory the running executable lives in.
pub fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
}

/// Directories that may contain loose asset overrides, most specific first.
///
/// * working directory (running from source / a copied folder)
/// * next to the executable (portable installs, Windows `resources/`)
/// * `../Resources` of the executable (macOS `.app` bundle layout)
pub fn asset_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd);
    }

    if let Some(dir) = exe_dir() {
        roots.push(dir.join("resources"));
        roots.push(dir.clone());
        // macOS: MyGame.app/Contents/MacOS/game -> MyGame.app/Contents/Resources
        if let Some(contents) = dir.parent() {
            roots.push(contents.join("Resources"));
        }
    }

    // Linux `.deb` install location.
    #[cfg(target_os = "linux")]
    roots.push(PathBuf::from("/usr/share/touch_me"));

    roots
}

/// All candidate on-disk locations for `path`, in lookup order.
pub fn asset_candidates(path: &str) -> Vec<PathBuf> {
    asset_roots()
        .into_iter()
        .map(|root| root.join(path))
        .collect()
}

/// Directory where custom levels are stored (and created on demand).
///
/// Uses `TOUCH_ME_LEVELS_DIR` when set, otherwise the platform user-data
/// directory, falling back to a folder next to the executable and finally
/// the current working directory.
pub fn levels_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("TOUCH_ME_LEVELS_DIR") {
        let dir = PathBuf::from(dir);
        if std::fs::create_dir_all(&dir).is_ok() {
            return dir;
        }
    }

    let mut candidates = Vec::new();
    if let Some(base) = user_data_dir() {
        candidates.push(base.join("touch_me").join("custom_levels"));
    }
    if let Some(dir) = exe_dir() {
        candidates.push(dir.join("custom_levels"));
    }
    candidates.push(PathBuf::from("custom_levels"));

    for dir in candidates {
        if std::fs::create_dir_all(&dir).is_ok() {
            return dir;
        }
    }

    PathBuf::from("custom_levels")
}

#[cfg(target_os = "windows")]
fn user_data_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(PathBuf::from)
}

#[cfg(target_os = "macos")]
fn user_data_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
    })
}

#[cfg(all(unix, not(target_os = "macos")))]
fn user_data_dir() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        return Some(PathBuf::from(xdg));
    }
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local").join("share"))
}

#[cfg(not(any(unix, target_os = "windows")))]
fn user_data_dir() -> Option<PathBuf> {
    None
}
