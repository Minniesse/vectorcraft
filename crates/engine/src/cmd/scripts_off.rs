//! The `script.*` commands in a build without the `scripting` feature: none is registered, so every
//! `script.*` id is an unknown command, and no Scripts folder or script data folder is attached.

use super::CommandSpec;
use crate::Session;

/// No extension opens as a script.
pub const EXTS: &[&str] = &[];

/// No extension runs as a script.
pub const RUN_EXTS: &[&str] = &[];

/// No file runs as a script.
pub fn runs(_: &str) -> bool {
    false
}

/// No `script.*` commands.
pub fn specs() -> Vec<CommandSpec> {
    Vec::new()
}

/// The bytes of the first `max` bytes `b64` encodes, decoding no more than that, as the
/// `scripting` build has it (`actionSet.read` uses it too).
pub(super) fn decode_head(b64: &str, max: usize) -> Option<Vec<u8>> {
    let chars = max.div_ceil(3).saturating_mul(4);
    let mut kept = 0usize;
    let end = b64
        .bytes()
        .position(|b| {
            kept += usize::from(!b.is_ascii_whitespace());
            kept > chars
        })
        .unwrap_or(b64.len());
    vectorcraft_format::base64_decode(b64.get(..end)?)
}

impl Session {
    /// No Scripts folder: scripts are not available in this build.
    pub fn attach_scripts_dir(&mut self, dir: std::path::PathBuf) {
        drop(dir);
    }

    /// No script data folder: scripts are not available in this build.
    pub fn attach_script_data(&mut self, dir: std::path::PathBuf) {
        drop(dir);
    }

    /// None: no script data folder.
    pub fn script_data_dir(&self) -> Option<&std::path::Path> {
        None
    }

    /// None: no Scripts folder.
    pub fn scripts_dir(&self) -> Option<&std::path::Path> {
        None
    }
}
