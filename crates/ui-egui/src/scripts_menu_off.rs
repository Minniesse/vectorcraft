//! File › Scripts in a build without the `scripting` feature: no script runs, so there is no
//! Scripts menu ([`crate::left_out`] leaves it out) and no Scripts folder is read.

use craft::menu::Node;
use serde_json::Value;

use crate::VectorcraftApp;

/// The command a listed script's menu item ran; this build has none.
pub const COMMAND: &str = "ui.runInstalledScript";

/// The notes the Scripts menu showed; this build shows none.
pub const LABELS: &[&str] = &[];

/// The most different names [`intern`] keeps in one session, as in a build with scripts.
pub const MAX_NAMES: usize = 16_384;

/// Leak-once interning of names menus show (Proof Setup's saved setups use it too); `None` for a
/// new name once [`MAX_NAMES`] are kept.
pub(crate) fn intern(s: &str) -> Option<&'static str> {
    use std::sync::Mutex;
    #[cfg(not(test))]
    let set: &'static Mutex<std::collections::HashSet<&'static str>> = {
        static SET: std::sync::OnceLock<Mutex<std::collections::HashSet<&'static str>>> = std::sync::OnceLock::new();
        SET.get_or_init(Default::default)
    };
    #[cfg(test)]
    let set: &'static Mutex<std::collections::HashSet<&'static str>> = {
        thread_local!(static SET: &'static Mutex<std::collections::HashSet<&'static str>> = Box::leak(Box::default()));
        SET.with(|s| *s)
    };
    let mut set = set.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(v) = set.get(s) {
        return Some(v);
    }
    if set.len() >= MAX_NAMES {
        return None;
    }
    let v: &'static str = Box::leak(s.to_string().into_boxed_str());
    set.insert(v);
    Some(v)
}

/// The menus' listing never changes.
pub fn revision() -> u64 {
    0
}

/// No Scripts folder is read.
pub fn refresh(_app: &VectorcraftApp, _ctx: &egui::Context) {}

/// No Scripts folder is read.
pub fn poll(_app: &VectorcraftApp, _ctx: &egui::Context) {}

/// File › Scripts' items: none ([`crate::left_out`] leaves the submenu out).
pub fn items() -> Vec<Node> {
    Vec::new()
}

/// No submenu is a Scripts subfolder.
pub fn is_folder(_children: &[Node]) -> bool {
    false
}

/// May the request being served read the file `p` names (Proof Setup's saved setups ask it too)?
pub fn readable(p: &Value) -> bool {
    p.get("path").and_then(Value::as_str).is_some_and(|path| vectorcraft_engine::file_access::check_read(path).is_ok())
}
