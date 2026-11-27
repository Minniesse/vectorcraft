//! The menu items and commands of features this build leaves out: without `plugins` (no
//! WebAssembly plug-in host) Install Plug-in…, Safe Mode… and File › Open With; without `scripting`
//! (no script interpreter) File › Scripts, Check Script…, Other Script… and the Script Console. The
//! menus, the UI command list and the command palette show none of them.

use craft::menu::Node;
use serde_json::Value;

/// Whether this build leaves out the command `id` with `params`.
pub(crate) fn hidden(id: &str, params: &Value) -> bool {
    let plugin = id.starts_with("plugin.") || matches!(id, "ui.installPlugin" | "help.safeMode" | "file.openWith");
    let script = id.starts_with("script.")
        || matches!(id, "ui.runScript" | "ui.runInstalledScript" | "ui.checkScript" | "ui.copyScriptConsole")
        || (id == "window.panel" && params["panel"] == "scriptConsole");
    (!cfg!(feature = "plugins") && plugin) || (!cfg!(feature = "scripting") && script)
}

/// Whether Preferences leaves out the setting `key`: the plug-in folder and budget without
/// `plugins`, the external script warning without `scripting`.
pub(crate) fn hidden_pref(key: &str) -> bool {
    (!cfg!(feature = "plugins") && matches!(key, "pluginsFolder" | "pluginInstructionBudget"))
        || (!cfg!(feature = "scripting") && key == "showExternalScriptWarning")
}

/// Whether this build leaves out the submenu named `english`, whatever it holds.
fn hidden_submenu(english: &str) -> bool {
    (!cfg!(feature = "plugins") && english == "Open With") || (!cfg!(feature = "scripting") && english == "Scripts")
}

/// Drops the items of [`hidden`] commands and the [`hidden_submenu`]s from `nodes`, and each
/// submenu they leave empty, at every depth.
pub(crate) fn prune(nodes: &mut Vec<Node>) {
    nodes.retain_mut(|n| match n {
        Node::Item(item) => !hidden(&item.command.id, &item.command.params),
        Node::Submenu { label, english, children, .. } => {
            if hidden_submenu(english.as_deref().unwrap_or(label)) {
                return false;
            }
            let had = !children.is_empty();
            prune(children);
            !had || !children.is_empty()
        }
        _ => true,
    });
}
