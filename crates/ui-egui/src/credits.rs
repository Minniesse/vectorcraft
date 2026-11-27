//! The About dialog's credits: every contributor (GitHub login always; display and real names only
//! with recorded consent) and the AI models that helped. The tables are baked into the binary by
//! build.rs from contributors/contributors.json; nothing is read at run time.

use craft::ui::patterns::form_dialogs::{self as about, AboutInfo, AboutLink, Credits, HostedAbout, ModelCredit};

use crate::VectorcraftApp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Contributor {
    pub login: &'static str,
    pub display_name: Option<&'static str>,
    pub real_name: Option<&'static str>,
    pub prs: u64,
    pub commits: u64,
    pub lines_added: u64,
    pub lines_deleted: u64,
    pub binary_added: u64,
    pub binary_deleted: u64,
    /// ISO 8601 UTC; empty for someone credited only through a merged PR.
    pub first_commit: &'static str,
    pub last_commit: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub company: &'static str,
    pub model: &'static str,
    pub version: &'static str,
    pub commits: u64,
    pub lines_added: u64,
    pub lines_deleted: u64,
}

include!(concat!(env!("OUT_DIR"), "/credits.rs"));

/// The baked tables as the About dialog's credits.
pub fn credits() -> Credits {
    let contributors: Vec<about::Contributor> = CONTRIBUTORS
        .iter()
        .map(|c| {
            let mut row = about::Contributor::new(c.login)
                .profile(format!("https://github.com/{}", c.login))
                .counts(c.prs, c.commits)
                .lines(c.lines_added, c.lines_deleted)
                .binary(c.binary_added, c.binary_deleted)
                .active(c.first_commit, c.last_commit);
            if let Some(name) = c.display_name {
                row = row.display_name(name);
            }
            if let Some(name) = c.real_name {
                row = row.real_name(name);
            }
            row
        })
        .collect();
    let models: Vec<ModelCredit> =
        MODELS.iter().map(|m| ModelCredit::new(m.company, m.model, m.version).work(m.commits, m.lines_added, m.lines_deleted)).collect();
    Credits::new(contributors, models, TOTAL_COMMITS)
}

/// The plain-text line that says what this build is based on (no mark, no community link).
pub const BASED_ON: &str =
    "Based on VectorCraft by the ArtCraft team (https://github.com/storytold/vectorcraft). This build is not an ArtCraft product.";

/// The About tab's contents.
pub fn about_info(app: &mut VectorcraftApp) -> AboutInfo {
    let links = crate::community::link_urls(app).map(|(label, url)| AboutLink::new(tl!(label), url));
    let mut built_with =
        format!("{BASED_ON}\n{}", tl!("MIT OR Apache-2.0. Fonts: Source Sans 3, Inter, JetBrains Mono (OFL). Icons: Lucide (ISC) + VectorCraft."));
    // The GPU the window renders with, for bug reports (Preferences › Performance picks it).
    if let Some(adapter) = &app.graphics_adapter {
        built_with.push('\n');
        built_with.push_str(&crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())]));
    }
    AboutInfo::new("VectorCraft", tl!("Open-source vector illustration in pure Rust."), env!("CARGO_PKG_VERSION"))
        .links(links)
        .built_with(built_with)
        .icon(crate::brand::image())
}

/// Help › About: opens the dialog on the window's host unless it is open. Before the window's first
/// frame there is no host and nothing opens.
pub fn open(app: &mut VectorcraftApp) {
    let (Some(host), Some(ctx)) = (app.dialogs(), app.ui_ctx.clone()) else { return };
    let dialog = HostedAbout::new(about_info(app), credits());
    // An open About stays as it is: a second request has nothing to add.
    let _ = host.with(|h| h.open_unless_open_in(&ctx, dialog).ok());
}

/// Opens the link the user activated in the About dialog, once. Only browsable URLs open.
pub fn open_taken_link(app: &mut VectorcraftApp) {
    let Some(host) = app.dialogs() else { return };
    let link = host.with(|h| h.get_mut::<HostedAbout>().and_then(HostedAbout::take_link));
    if let Some(url) = link
        && crate::services::browsable_url(&url).is_ok()
    {
        app.open_url(&url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// build.rs bakes a display or real name only for a login whose people.toml entry is verified;
    /// this keeps that rule from silently loosening.
    #[test]
    fn every_baked_name_has_its_owners_consent() {
        let json: serde_json::Value = serde_json::from_str(include_str!("../../../contributors/contributors.json")).unwrap();
        let people = include_str!("../../../contributors/people.toml");
        let rows = json["contributors"].as_array().cloned().unwrap_or_default();
        for c in CONTRIBUTORS.iter().filter(|c| c.display_name.is_some() || c.real_name.is_some()) {
            let verified = rows.iter().any(|r| r["login"] == c.login && r["people_entry"] == true);
            assert!(verified, "{} has a baked name without a verified entry", c.login);
            let header = people.contains(&format!("[people.{}]", c.login)) || people.contains(&format!("[people.\"{}\"]", c.login));
            assert!(header, "{} has no [people.{}] entry", c.login, c.login);
        }
    }

    /// About names the app in plain text and says what the build is based on; no mark, no
    /// community button, and its one link is the upstream repository.
    #[test]
    fn about_names_the_app_in_plain_text_and_what_it_is_based_on() {
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), Default::default());
        let info = about_info(&mut app);
        assert_eq!(info.app_name.as_str(), "VectorCraft");
        assert!(info.mark.is_none() && info.part_of.is_none() && info.community.is_none());
        assert_eq!(info.built_with.as_str().lines().filter(|l| *l == BASED_ON).count(), 1, "{:?}", info.built_with);
        let urls: Vec<&str> = info.links.iter().map(|l| l.url.as_str()).collect();
        assert_eq!(urls, ["https://github.com/storytold/vectorcraft"]);
    }
}
