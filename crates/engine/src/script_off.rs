//! Scripts in a build without the `scripting` feature: no ECMAScript interpreter. No script ever
//! runs, so there is never a running script's job; the names the rest of the engine uses are kept.

/// A running script's job; there is never one in this build.
#[derive(Debug)]
pub enum JobKey {}

/// No command is refused to a script: none runs.
pub fn refused_to_scripts(_: &str) -> bool {
    false
}

/// The error for a script's command that [`refused_to_scripts`] names (never, in this build).
pub(crate) fn refused_text(id: &str) -> String {
    format!("`{id}` can't be used from a script")
}

impl crate::Session {
    /// Is a script running? Never, in this build.
    pub fn script_running(&self) -> bool {
        false
    }

    /// The report of the last script run, as automation shows it: none, in this build.
    pub fn shown_script_report(&self) -> Option<serde_json::Value> {
        None
    }
}
