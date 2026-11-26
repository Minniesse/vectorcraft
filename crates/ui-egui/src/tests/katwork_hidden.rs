//! The KatWork build: without `plugins` (no WebAssembly plug-in host) and without `scripting` (no
//! ECMAScript interpreter) the menus, the command list and the command palette show none of the
//! items and commands those features had. Object › Plug-ins stays: it holds the built-in add-ons.

use serde_json::Value;
use vectorcraft_engine::Session;

use crate::{VectorcraftApp, menus};

/// Every command id the menus, the UI command list and the palette offer, with the menu path and
/// label of each menu item.
fn offered() -> (Vec<(Vec<String>, String, String, Value)>, Vec<&'static str>, Vec<String>) {
    let app = VectorcraftApp::new(Session::new(), crate::Services::default());
    let entries = menus::menu_entries(&app).into_iter().map(|e| (e.path, e.label, e.command.unwrap_or_default(), e.params)).collect();
    let ui = menus::UI_COMMANDS.iter().map(|c| c.0).collect();
    let palette = crate::palette::items().into_iter().map(|(_, id)| id).collect();
    (entries, ui, palette)
}

/// The ids offered anywhere that `hidden` matches, each with where it was offered.
fn found(hidden: impl Fn(&str, &Value) -> bool) -> Vec<String> {
    let (entries, ui, palette) = offered();
    let mut out = vec![];
    for (path, label, id, params) in &entries {
        if hidden(id, params) {
            out.push(format!("menu {} › {label} ({id})", path.join(" › ")));
        }
    }
    out.extend(ui.into_iter().filter(|id| hidden(id, &Value::Null)).map(|id| format!("UI command {id}")));
    out.extend(palette.into_iter().filter(|id| hidden(id, &Value::Null)).map(|id| format!("palette {id}")));
    out
}

#[cfg(not(feature = "plugins"))]
#[test]
fn katwork_hidden_plugin_items() {
    let plugin = |id: &str, _: &Value| id.starts_with("plugin.") || matches!(id, "ui.installPlugin" | "help.safeMode" | "file.openWith");
    let shown = found(plugin);
    assert!(shown.is_empty(), "plug-in items and commands in a build without plug-ins: {shown:#?}");
    let (entries, ..) = offered();
    let named: Vec<_> = entries
        .iter()
        .filter(|(path, label, ..)| path.iter().any(|p| p == "Open With") || label.contains("Safe Mode") || label.to_lowercase().contains("plug-in"))
        .map(|(path, label, ..)| format!("{} › {label}", path.join(" › ")))
        .collect();
    assert!(named.is_empty(), "plug-in menu items in a build without plug-ins: {named:#?}");
}

#[cfg(not(feature = "scripting"))]
#[test]
fn katwork_hidden_script_items() {
    let script = |id: &str, params: &Value| {
        id.starts_with("script.")
            || matches!(id, "ui.runScript" | "ui.runInstalledScript" | "ui.checkScript" | "ui.copyScriptConsole")
            || (id == "window.panel" && params["panel"] == "scriptConsole")
    };
    let shown = found(script);
    assert!(shown.is_empty(), "script items and commands in a build without scripting: {shown:#?}");
    let (entries, ..) = offered();
    let named: Vec<_> = entries
        .iter()
        .filter(|(path, label, ..)| path.iter().any(|p| p == "Scripts") || label.contains("Script"))
        .map(|(path, label, ..)| format!("{} › {label}", path.join(" › ")))
        .collect();
    assert!(named.is_empty(), "script menu items in a build without scripting: {named:#?}");
}
