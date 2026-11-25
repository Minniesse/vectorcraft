//! The plug-in dialogs and menus in a build without the `plugins` feature: no plug-in is installed
//! and every `plugin.*` command is unknown, so the plug-in menus hold their "none installed" rows
//! and Install Plug-in… alone, no plug-in dialog opens, and an install never waits for an answer.

use serde_json::Value;

use crate::VectorcraftApp;
use crate::menus::{Sep, c, placeholder};
use crate::plugin_host::Plugin;

/// The key of the hosted dialog of plug-in filters.
pub const KIND: &str = "plugin";

/// Object › Plug-ins' disabled row while no filter plug-in is installed.
pub const NO_FILTERS: &str = "No filter plug-ins installed";

/// File › Open With's disabled row while no importer plug-in is installed.
pub const NO_IMPORTERS: &str = "No importer plug-ins installed";

/// The key of an exporter's parameters dialog.
pub const EXPORT_KIND: &str = "pluginExport";

/// The key of an importer's parameters dialog.
pub const IMPORT_KIND: &str = "pluginImport";

/// The key of the approval of a plug-in install.
pub const INSTALL_KIND: &str = "pluginInstall";

/// No plug-in command is placed in a menu: none is installed.
pub fn place_commands(_: &mut [(&'static str, Vec<craft::menu::Node>)]) {}

/// Object › Plug-ins: no filter is installed; Install Plug-in… and Reload Plug-ins stay, and say
/// that plug-ins are not available when chosen.
pub fn object_menu() -> Vec<craft::menu::Node> {
    vec![placeholder(NO_FILTERS), Sep, c("Install Plug-in…", "ui.installPlugin"), c("Reload Plug-ins", "plugin.reload")]
}

/// File › Open With: no importer is installed.
pub fn open_with_menu() -> Vec<craft::menu::Node> {
    vec![placeholder(NO_IMPORTERS)]
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
