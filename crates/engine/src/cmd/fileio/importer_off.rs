//! `document.open` in a build without the `plugins` feature: no importer plug-in is installed, so
//! a file opens with a built-in format or not at all, and `importer` names nothing that can open it.

use std::sync::Arc;

use serde_json::Value;

use super::super::{Result, bad};
use super::load::Loaded;
use crate::plugin_host::Plugin;

const C: &str = "document.open";

/// No importer opens a file; one `importer` names is not installed.
pub(super) fn pick(_: &str, _: &[u8], p: &Value) -> Result<Option<Arc<Plugin>>> {
    match p.get("importer").filter(|v| !v.is_null()) {
        None => Ok(None),
        Some(_) => Err(bad(C, "importer plug-ins are not available in this build")),
    }
}

/// Nothing to refuse early: no file is read by an importer.
pub(super) fn check_data(_: &Value) -> Result<()> {
    Ok(())
}

/// The file at `path`, read whole as a built-in format reads it.
pub(super) fn read_path(path: &str, _: &Value) -> Result<Vec<u8>> {
    super::read_file(path)
}

/// Never called: [`pick`] gives no plug-in.
pub(super) fn load(_: &str, _: &[u8], plugin: &Plugin, _: &Value) -> Result<Loaded> {
    match *plugin {}
}
