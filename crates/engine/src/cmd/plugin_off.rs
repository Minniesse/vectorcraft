//! The `plugin.*` commands in a build without the `plugins` feature: no WebAssembly plug-in host.
//! None is registered, so every `plugin.*` id is an unknown command, plug-in command ids
//! (`plugin.<id>.<command>`) included; the Plug-ins folder is never read and no plug-in loads at
//! startup. The names the rest of the engine and the interface use are kept, inert.

use std::sync::Arc;

use serde_json::Value;

use super::{CommandSpec, Result};
use crate::Session;
use crate::plugin_host::{Command, Plugin};

/// No `plugin.*` commands.
pub fn specs() -> Vec<CommandSpec> {
    Vec::new()
}

/// The command id of plug-in `plugin`'s command `command`: `plugin.<plugin>.<command>`.
pub fn command_id(plugin: &str, command: &str) -> String {
    format!("plugin.{plugin}.{command}")
}

/// No id is a plug-in command's: none can be installed.
pub fn is_command_id(_: &str) -> bool {
    false
}

/// No plug-in is installed.
pub fn command_of(_: &str) -> Option<(Arc<Plugin>, Command)> {
    None
}

/// No plug-in commands.
pub fn command_specs() -> Vec<&'static CommandSpec> {
    Vec::new()
}

/// Never changes: there are no plug-in commands.
pub fn commands_generation() -> u64 {
    0
}

/// No plug-in command has a spec.
pub fn find_spec(_: &str) -> Option<&'static CommandSpec> {
    None
}

/// Never called: [`is_command_id`] is false for every id.
pub(crate) fn run_command(_: &mut Session, id: &str, _: &Value) -> Result<Value> {
    Err(crate::EngineError::UnknownCommand(id.into()))
}

/// The label of `label`'s other state, for a menu item that switches between two labels (as the
/// `plugins` build has it; the desktop app's dynamic labels use it).
pub fn other_state(label: &str) -> Option<String> {
    const PAIRS: &[(&str, &str)] = &[
        ("Preview", "Outline"),
        ("Change to Global Rulers", "Change to Artboard Rulers"),
        ("Edit Envelope", "Edit Contents"),
        ("Start Global Edit", "Stop Global Edit"),
    ];
    let paired = |&(a, b): &(&'static str, &'static str)| {
        if label == a {
            Some(b)
        } else if label == b {
            Some(a)
        } else {
            None
        }
    };
    if let Some(other) = PAIRS.iter().find_map(paired) {
        return Some(other.to_string());
    }
    [("Hide ", "Show "), ("Show ", "Hide "), ("Lock ", "Unlock "), ("Unlock ", "Lock ")]
        .iter()
        .find_map(|(from, to)| label.strip_prefix(from).map(|rest| format!("{to}{rest}")))
}

/// No plug-in command is placed in another menu: none is installed.
pub fn placed_menu(p: &Plugin, _: &Command) -> Option<&'static [&'static str]> {
    match *p {}
}

/// The extensions File › Open installs as plug-ins: none in this build.
pub const EXTS: &[&str] = &[];

/// A filter run of a plug-in; none can be made.
pub enum FilterRun {}

impl FilterRun {
    /// Refused: plug-ins are not available in this build.
    pub fn prepare(_: &Session, _: &Value) -> Result<FilterRun> {
        Err(crate::EngineError::UnknownCommand("plugin.run".into()))
    }

    pub fn plugin(&self) -> &Arc<Plugin> {
        match *self {}
    }

    pub fn inputs(&self) -> &[vectorcraft_doc::NodeId] {
        match *self {}
    }

    pub fn document(&self) -> u64 {
        match *self {}
    }

    pub fn current(&self, _: &Session) -> bool {
        match *self {}
    }

    pub fn run(&self) -> Result<Vec<u8>> {
        match *self {}
    }

    pub fn calls(&self) -> bool {
        match *self {}
    }

    pub fn hand_in(self, _: &mut Session, _: Vec<u8>) {
        match self {}
    }
}

/// What a person approved for one install; no install can use it in this build.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstallApproval {
    pub id: String,
    pub hash: String,
    pub permissions: Vec<String>,
    pub heavy: bool,
}

impl Session {
    /// Nothing to approve: `plugin.install` is unknown in this build.
    pub fn approve_plugin_install(&mut self, approval: InstallApproval) {
        drop(approval);
    }

    /// Nothing to clear.
    pub fn clear_plugin_install_approval(&mut self) {}

    /// Nothing to allow: `plugin.install` is unknown in this build.
    pub fn allow_plugin_grants_from_requests(&mut self) {}
}

/// The Plug-ins folder preference is not read.
pub(crate) fn sync_prefs(_: &Session) {}

/// Nothing was loaded from a folder.
pub(crate) fn forget_prefs_folder(_: &str) {}

/// Nothing to turn off: no plug-in loads or runs in this build.
pub fn start_in_safe_mode() {}

/// No plug-in is stored or loaded in this build.
pub fn load_prefs_folder(_: &mut Session) {}
