//! Help → project links (this app's GitHub repository). The commands return the URL; the frontend
//! opens it.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// The app's id on GitHub.
pub const APP_ID: &str = "vectorcraft";

/// This app's source repository.
pub fn github_url() -> String {
    format!("https://github.com/storytold/{APP_ID}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.appPage", "VectorCraft Project Page", [], None, "{} → {url} this app's page (its GitHub repository)", always, |_, _| url(github_url())),
        cmd!(query "help.github", "VectorCraft on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {appPage, github}", always, |_, _| {
            Ok(json!({ "appPage": github_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    /// Every help link is the upstream repository; the community and website links are gone.
    #[test]
    fn links() {
        let mut s = Session::new();
        let repo = "https://github.com/storytold/vectorcraft";
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], repo);
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], repo);
        let links = s.execute("help.links", &json!({})).unwrap();
        assert_eq!(links, json!({ "appPage": repo, "github": repo }));
        for gone in ["help.discord", "help.website"] {
            assert!(s.execute(gone, &json!({})).is_err(), "{gone} is not a command");
        }
    }
}
