//! Project links: the URL About and Home list (the upstream GitHub repository).

use crate::VectorcraftApp;

/// The GitHub link as (English label, URL): the one source About and Home list it from.
pub fn link_urls(app: &mut VectorcraftApp) -> [(&'static str, String); 1] {
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let url = |k: &str| l[k].as_str().unwrap_or("").to_string();
    [(tl!("Source code on GitHub"), url("github"))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The URLs the frame after `app`'s last action asked to open.
    fn opened_in_frame(app: &mut VectorcraftApp, ctx: &egui::Context) -> Vec<String> {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| app.tick(ui.ctx()));
        out.textures_delta.clear();
        out.platform_output
            .commands
            .into_iter()
            .filter_map(|c| match c {
                egui::OutputCommand::OpenUrl(open) => Some(open.url),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn link_clicks_open_the_right_urls() {
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), Default::default());
        let ctx = crate::testing::ui_context();
        let mut opened = vec![];
        for id in ["help.appPage", "help.github"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
            opened.extend(opened_in_frame(&mut app, &ctx));
        }
        assert_eq!(opened, ["https://github.com/storytold/vectorcraft", "https://github.com/storytold/vectorcraft"]);
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.github", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://github.com/storytold/vectorcraft");
        assert!(opened_in_frame(&mut app, &ctx).is_empty(), "nothing more is opened");
    }

    #[test]
    fn link_urls_reads_the_github_link() {
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), Default::default());
        let links = link_urls(&mut app);
        assert_eq!(links, [("Source code on GitHub", "https://github.com/storytold/vectorcraft".to_string())]);
    }
}
