//! The WebAssembly plug-in host in a build without the `plugins` feature: the names the engine
//! takes from vectorcraft-plugins, with nothing installed and nothing that runs a module. A plug-in
//! effect a document names takes the not-installed path; exporter and importer plug-ins add no
//! formats. The text helpers scripts share (`permissions::display_text`, `sha256_hex`) and the id
//! rules are the same as vectorcraft-plugins'.

/// The prefix of every plug-in id in command, effect and format ids.
pub const PREFIX: &str = "plugin.";

/// An installed plug-in; there are none.
#[derive(Debug)]
pub enum Plugin {}

impl Plugin {
    pub fn manifest(&self) -> &Manifest {
        match *self {}
    }
}

/// A plug-in's manifest; there are none.
#[derive(Debug)]
pub enum Manifest {}

impl Manifest {
    pub fn resolve_params(&self, _: &serde_json::Value) -> Result<(), String> {
        match *self {}
    }
}

/// A plug-in's command; there are none.
#[derive(Clone, Debug)]
pub enum Command {}

pub mod effect {
    use super::Plugin;

    pub const PREFIX: &str = super::PREFIX;

    /// The plug-in id of effect id `effect_id`.
    pub fn plugin_id(effect_id: &str) -> Option<&str> {
        effect_id.strip_prefix(PREFIX)
    }

    /// Safe Mode never skips an effect here: no plug-in effect runs.
    pub fn skipped(_: &str) -> bool {
        false
    }

    /// No plug-in is installed.
    pub fn installed(_: &str) -> Option<std::sync::Arc<Plugin>> {
        None
    }
}

pub mod export {
    use super::Plugin;

    /// No exporter plug-in adds a format.
    pub fn by_format(_: &str) -> Option<std::sync::Arc<Plugin>> {
        None
    }

    /// The format id of plug-in `plugin_id`: `plugin.<id>`.
    pub fn format_id(plugin_id: &str) -> String {
        format!("{}{plugin_id}", super::PREFIX)
    }

    /// The plug-in id of format id `format` (`plugin.<id>`).
    pub fn plugin_id(format: &str) -> Option<&str> {
        format.strip_prefix(super::PREFIX)
    }
}

pub mod import {
    use super::Plugin;

    /// No importer plug-in is installed.
    pub fn named(importer: &str) -> Result<std::sync::Arc<Plugin>, String> {
        Err(format!("no importer plug-in {importer:?} is installed: plug-ins are not available in this build"))
    }

    /// No importer plug-in reads any extension.
    pub fn for_extension(_: &str) -> Vec<std::sync::Arc<Plugin>> {
        Vec::new()
    }
}

pub mod safe_mode {
    /// The most characters [`display_name`] keeps, as vectorcraft-plugins has it.
    pub const MAX_NAME_CHARS: usize = 64;

    /// `s` safe to show in a sentence, as vectorcraft-plugins writes it.
    pub fn display_name(s: &str) -> String {
        let reorders = |c: char| matches!(c, '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}');
        let clean: Vec<char> = s.trim().chars().filter(|&c| !c.is_control() && !reorders(c)).collect();
        if clean.len() > MAX_NAME_CHARS {
            clean[..MAX_NAME_CHARS - 1].iter().chain(std::iter::once(&'…')).collect()
        } else {
            clean.into_iter().collect()
        }
    }

    /// Safe Mode is about plug-ins; with none it is never on.
    pub fn active() -> bool {
        false
    }
}

pub mod host {
    /// The progress step a plug-in reports, as vectorcraft-plugins names it.
    pub const PROGRESS_STEP: &str = "plug-in";
}

pub mod deferred {
    /// The first view's budget for live-effect plug-ins, as vectorcraft-plugins has it.
    pub const OPENING_BUDGET_MS: u64 = 2_000;

    /// A document's deferred plug-in stages: there are never any.
    #[derive(Clone, Debug, Default)]
    pub struct Deferrals;

    impl Deferrals {
        pub fn new() -> Self {
            Deferrals
        }

        /// Nothing is deferred, so there is nothing to report.
        pub fn take_report(&self) -> Option<usize> {
            None
        }

        /// Nothing waits for the first view.
        pub fn first_view_done(&self) {}

        /// Runs `f`: nothing is ever deferred.
        pub fn view<T>(&self, f: impl FnOnce() -> T) -> T {
            f()
        }
    }
}

pub mod registry {
    /// Never changes: nothing is installed or removed.
    pub fn revision() -> u64 {
        0
    }
}

pub mod permissions {
    /// Is `c` a character that changes the direction text is laid out in, or a line or paragraph
    /// separator, which a prompt must not let a manifest string use to disguise itself?
    fn is_bidi_or_separator(c: char) -> bool {
        matches!(c, '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{2028}' | '\u{2029}')
    }

    /// `s` as a prompt shows a string from a manifest: control, bidirectional formatting and line
    /// separator characters written as `\u{202E}` escapes, and at most `max` characters, the last of
    /// them `…` when it was cut.
    pub fn display_text(s: &str, max: usize) -> String {
        use std::fmt::Write as _;
        let mut out = String::new();
        // Characters written so far, and the byte length of the longest prefix of whole pieces
        // shorter than `max` characters, where the ellipsis goes when the text is cut.
        let mut count = 0;
        let mut cut = 0;
        for c in s.chars() {
            let start = out.len();
            let n = if c.is_control() || is_bidi_or_separator(c) {
                let _ = write!(out, "\\u{{{:04X}}}", c as u32);
                out.len() - start
            } else {
                out.push(c);
                1
            };
            if count + n > max {
                out.truncate(cut);
                out.push('…');
                return out;
            }
            count += n;
            if count < max {
                cut = out.len();
            }
        }
        out
    }

    /// The SHA-256 of `bytes`, in lower-case hexadecimal.
    pub fn sha256_hex(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
    }
}

/// What vectorcraft-plugins' functions refuse with; only Safe Mode's message is named here.
#[derive(Debug)]
pub enum Error {
    /// Safe Mode is on (never, in this build).
    SafeMode,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SafeMode => f.write_str("plug-ins are not available in this build"),
        }
    }
}

pub mod manifest {
    /// A color parameter's value: a color object, or a hex string (display sRGB, converted into the
    /// active working RGB space), its components clamped into range; as vectorcraft-plugins reads it.
    pub fn color_value(v: &serde_json::Value) -> Option<vectorcraft_color::Color> {
        use vectorcraft_color::{Color, cms};
        let c = match v {
            serde_json::Value::String(s) => match Color::from_hex(s)? {
                Color::Rgb { r, g, b } => {
                    let [r, g, b] = cms::active().srgb_to_rgb([r, g, b]);
                    Color::rgb(r, g, b)
                }
                other => other,
            },
            serde_json::Value::Object(_) => serde_json::from_value(v.clone()).ok()?,
            _ => return None,
        };
        let unit = |v: f32| v.clamp(0.0, 1.0);
        Some(match c {
            Color::Rgb { r, g, b } => Color::Rgb { r: unit(r), g: unit(g), b: unit(b) },
            Color::Cmyk { c, m, y, k } => Color::Cmyk { c: unit(c), m: unit(m), y: unit(y), k: unit(k) },
            Color::Gray { k } => Color::Gray { k: unit(k) },
            Color::Lab { l, a, b } => Color::Lab { l: l.clamp(0.0, 100.0), a: a.clamp(-128.0, 127.0), b: b.clamp(-128.0, 127.0) },
        })
    }
}
