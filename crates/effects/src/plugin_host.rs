//! The live-effect plug-in host in a build without the `plugins` feature: no plug-in is ever
//! installed, so a plug-in effect a document names (`plugin.<id>`) takes the not-installed path:
//! its object draws its geometry without it and keeps the effect. The ids are read exactly as
//! vectorcraft-plugins reads them.

/// The names vectorcraft-plugins gives this crate, with nothing installed.
pub(crate) mod effect {
    use vectorcraft_doc::{Appearance, Document};
    use vectorcraft_geom::{FillRule, PathData, Rect};

    use super::Plugin;
    use super::deferred::Stage;
    use super::painted::Piece;

    /// The prefix of a plug-in's effect id (`plugin.<id>`), as vectorcraft-plugins has it.
    pub const PREFIX: &str = "plugin.";

    /// The plug-in id of effect id `effect_id`.
    pub fn plugin_id(effect_id: &str) -> Option<&str> {
        effect_id.strip_prefix(PREFIX)
    }

    /// The effect id of plug-in `plugin_id`.
    pub fn effect_id(plugin_id: &str) -> String {
        format!("{PREFIX}{plugin_id}")
    }

    /// No plug-in is installed.
    pub fn installed(_: &str) -> Option<std::sync::Arc<Plugin>> {
        None
    }

    /// No plug-in is installed, so none paints its output.
    pub fn paints_output(_: &str) -> bool {
        false
    }

    /// What a live effect is given besides its geometry.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Paints<'a> {
        pub appearance: Option<&'a Appearance>,
        pub item: Option<usize>,
        pub at: Option<Stage>,
    }

    /// `None`: the plug-in isn't installed (the caller keeps the geometry).
    pub fn apply(_: &str, _: &serde_json::Value, _: &PathData, _: Rect, _: FillRule, _: Paints, _: Option<&Document>) -> Option<PathData> {
        None
    }

    /// `None`: the plug-in isn't installed (the caller draws its input unchanged).
    pub fn apply_painted(
        _: &str,
        _: &serde_json::Value,
        _: &PathData,
        _: Rect,
        _: FillRule,
        _: Paints,
        _: Option<&Document>,
    ) -> Option<std::sync::Arc<Vec<Piece>>> {
        None
    }
}

pub(crate) mod deferred {
    /// Where a plug-in effect sits: the object, the index of the effect in its list, and the fill
    /// or stroke whose list that is.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Stage {
        pub object: u64,
        pub effect: usize,
        pub item: Option<usize>,
    }

    /// Never changes: no stage is ever deferred.
    pub fn revision() -> u64 {
        0
    }
}

pub(crate) mod painted {
    use vectorcraft_doc::{Appearance, AppearanceItem};
    use vectorcraft_geom::{FillRule, PathData};

    use super::Never;

    /// A piece a painted live effect returned; none ever is.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Piece {
        pub path: PathData,
        pub rule: FillRule,
        pub fills: Option<Never>,
        pub strokes: Option<Never>,
        pub(crate) never: Never,
    }

    pub fn appearance(_: &Appearance, p: &Piece) -> Appearance {
        match p.never {}
    }

    pub fn items(_: &AppearanceItem, p: &Piece) -> Vec<AppearanceItem> {
        match p.never {}
    }
}

pub(crate) mod registry {
    use super::Plugin;

    /// No plug-in is installed.
    pub fn list() -> Vec<std::sync::Arc<Plugin>> {
        Vec::new()
    }

    /// Never changes: nothing is installed or removed.
    pub fn revision() -> u64 {
        0
    }

    /// `s` as a `&'static str`; only an installed plug-in's labels are interned, and there are none.
    pub fn intern(s: &str) -> &'static str {
        Box::leak(s.to_string().into_boxed_str())
    }
}

pub(crate) mod objects {
    /// The most objects a filter's output may hold, as vectorcraft-plugins has it.
    pub const MAX_OBJECTS: usize = 100_000;
}

/// A value no stub ever holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Never {}

/// An installed plug-in; there are none.
#[derive(Debug)]
pub(crate) enum Plugin {}

impl Plugin {
    pub fn id(&self) -> &str {
        match *self {}
    }

    pub fn manifest(&self) -> &Manifest {
        match *self {}
    }
}

/// A plug-in's manifest; there are none (it holds a [`Never`], so none can be made).
#[derive(Debug)]
pub(crate) struct Manifest {
    pub id: String,
    pub name: String,
    pub params: Vec<Never>,
    never: Never,
}

impl Manifest {
    pub fn params_doc(&self) -> String {
        match self.never {}
    }

    pub fn defaults(&self) -> serde_json::Map<String, serde_json::Value> {
        match self.never {}
    }
}
