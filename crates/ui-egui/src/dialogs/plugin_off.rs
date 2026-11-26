//! The plug-in dialogs and menus in a build without the `plugins` feature: no plug-in is installed
//! and every `plugin.*` command is unknown, so the menus hold no plug-in row (Install Plug-in…,
//! File › Open With and Safe Mode… are left out, [`crate::left_out`]), no plug-in dialog opens,
//! and an install never waits for an answer.

use serde_json::Value;

use crate::VectorcraftApp;
use crate::plugin_host::Plugin;

/// The key of the hosted dialog of plug-in filters.
pub const KIND: &str = "plugin";

/// The key of an exporter's parameters dialog.
pub const EXPORT_KIND: &str = "pluginExport";

/// The key of an importer's parameters dialog.
pub const IMPORT_KIND: &str = "pluginImport";

/// The key of the approval of a plug-in install.
pub const INSTALL_KIND: &str = "pluginInstall";

/// No plug-in command is placed in a menu: none is installed.
pub fn place_commands(_: &mut [(&'static str, Vec<craft::menu::Node>)]) {}

/// Object › Plug-ins: only the built-in add-ons, no plug-in row ([`crate::left_out`]).
pub fn object_menu() -> Vec<craft::menu::Node> {
    Vec::new()
}

/// File › Open With: left out of this build ([`crate::left_out`]).
pub fn open_with_menu() -> Vec<craft::menu::Node> {
    Vec::new()
}

/// Effect › Plug-ins: none (no live-effect plug-in is installed).
pub fn effect_menu() -> Option<craft::menu::Node> {
    None
}

/// `plugin.dialog`: no plug-in is installed.
pub fn open(_: &mut VectorcraftApp, p: &Value) -> Result<Value, String> {
    let id = p.get("id").and_then(Value::as_str).unwrap_or_default();
    Err(format!("no plug-in {id:?} is installed: plug-ins are not available in this build"))
}

/// No plug-in command has a dialog.
pub fn open_command(_: &mut VectorcraftApp, _: &str) -> bool {
    false
}

/// No exporter plug-in is installed.
pub fn open_export(_: &mut VectorcraftApp, plugin: &Plugin, _: Value) {
    match *plugin {}
}

/// No importer plug-in is installed.
pub fn open_import(_: &mut VectorcraftApp, plugin: &Plugin, _: Value) {
    match *plugin {}
}

/// Nothing to pump: no plug-in dialog or install question is ever open.
pub(crate) fn pump(_: &mut VectorcraftApp, _: &egui::Context) {}
