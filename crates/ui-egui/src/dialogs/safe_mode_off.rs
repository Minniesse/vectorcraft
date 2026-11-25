//! Help › Safe Mode in a build without the `plugins` feature: Safe Mode only ever kept plug-ins
//! from loading, and this build has none, so it is never on and has nothing to list.

use serde_json::Value;

use crate::VectorcraftApp;

/// Help › Safe Mode: says plug-ins are not available in this build.
pub fn open(_: &mut VectorcraftApp) -> Result<Value, String> {
    Err("Safe Mode turns plug-ins off; plug-ins are not available in this build".into())
}

/// No choice about a stopped plug-in is ever waiting.
pub fn choosing(_: &VectorcraftApp) -> bool {
    false
}

/// Safe Mode is never on.
pub fn staying_on(_: &VectorcraftApp) -> bool {
    false
}

/// Nothing to pump.
pub(crate) fn pump(_: &mut VectorcraftApp, _: &egui::Context) {}
