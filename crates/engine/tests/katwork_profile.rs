//! The KatWork build profile: built without the `plugins` feature (no WebAssembly plug-in host),
//! every `plugin.*` command is unknown, and a document that names a plug-in live effect opens,
//! draws and saves as an upstream build with that plug-in not installed does.
//! `cargo test -p vectorcraft-engine --no-default-features --features bundled-fonts --test katwork_profile`
//! (the engine as the KatWork CLI builds it) runs every test here; the default build runs those not
//! marked for a gate. Without `heif` (the apps' `heic` gate) HEIC and HEIF files are refused.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use serde_json::json;
use vectorcraft_engine::Session;
use vectorcraft_engine::doc::NodeId;

/// A 400 × 200 document with a red 40 × 40 square; its id.
fn square() -> (Session, NodeId) {
    let mut s = Session::new();
    s.execute("file.new", &json!({"width": 400, "height": 200})).unwrap();
    let r = s.execute("shape.rectangle", &json!({"x": 20, "y": 20, "width": 40, "height": 40})).unwrap();
    let id = NodeId(r["id"].as_u64().unwrap());
    s.execute("paint.setFill", &json!({"ids": [id.0], "color": "#ff0000"})).unwrap();
    (s, id)
}

/// The 120 × 100 top left of the document, drawn on white.
fn drawn(s: &Session) -> Vec<u8> {
    let opts = vectorcraft_render::RenderOptions { background: Some([255, 255, 255, 255]), ..Default::default() };
    let r = vectorcraft_render::Renderer::new().render(&s.doc().unwrap().doc, 120, 100, vectorcraft_geom::Affine::IDENTITY, &opts);
    r.pixels.clone()
}

#[cfg(not(feature = "plugins"))]
#[test]
fn plugin_commands_are_unknown() {
    let (mut s, id) = square();
    let wasm = "AGFzbQEAAAA="; // the 8-byte WebAssembly header, base64
    let calls = [
        ("plugin.install", json!({"dataBase64": wasm, "name": "x.wasm"})),
        ("plugin.install", json!({"path": "/tmp/x.wasm"})),
        ("plugin.list", json!({})),
        ("plugin.info", json!({"id": "org.example.x"})),
        ("plugin.inspect", json!({"dataBase64": wasm})),
        ("plugin.run", json!({"id": "org.example.x", "ids": [id.0]})),
        ("plugin.remove", json!({"id": "org.example.x"})),
        ("plugin.reload", json!({})),
        // A plug-in command id (`plugin.<plug-in id>.<command id>`).
        ("plugin.org.example.x.go", json!({})),
    ];
    for (cmd, p) in calls {
        match s.execute(cmd, &p) {
            Err(vectorcraft_engine::EngineError::UnknownCommand(named)) => assert_eq!(named, cmd),
            other => panic!("{cmd}: expected the unknown-command error, got {other:?}"),
        }
    }
    assert!(vectorcraft_engine::cmd::command_specs().iter().all(|c| !c.id.starts_with("plugin.")), "no plugin.* command is registered");
    // Applying a plug-in effect is refused as for a plug-in that isn't installed.
    assert!(s.execute("effect.apply", &json!({"ids": [id.0], "effect": "plugin.org.example.x"})).is_err());
}

/// A plug-in live effect a document names (`plugin.<id>`) leaves its object's geometry as it is, and
/// the document keeps the effect when saved and opened again. Runs in both builds: with the host no
/// plug-in is installed here, so both builds must draw the same.
#[test]
fn a_plugin_live_effect_draws_as_without_the_plugin_and_is_kept() {
    let (mut s, id) = square();
    let plain = drawn(&s);
    {
        let st = s.doc_mut().unwrap();
        let d = Arc::make_mut(&mut st.doc);
        let n = d.node_mut(id).unwrap();
        n.appearance.effects.push(vectorcraft_engine::doc::Effect {
            id: "plugin.org.example.not-installed".into(),
            params: json!({"size": 9}),
            visible: true,
        });
    }
    assert_eq!(drawn(&s), plain, "the object draws its geometry without the effect");
    let saved = s.execute("document.serialize", &json!({})).unwrap();
    let data = ["dataBase64", "data"].iter().find_map(|k| saved[*k].as_str().map(str::to_string)).expect("serialized bytes");
    let mut t = Session::new();
    t.execute("document.open", &json!({"name": "kept.vectorcraft", "dataBase64": data})).unwrap();
    let mut kept = false;
    t.doc().unwrap().doc.walk(|n| kept |= n.appearance.effects.iter().any(|e| e.id == "plugin.org.example.not-installed"));
    assert!(kept, "the document keeps the plug-in effect");
    assert_eq!(drawn(&t), plain);
}

/// Without `heif` (the CLI's and web app's `heic` gate) a HEIC file is refused with a message: the
/// HEVC decoder is not in the build.
#[cfg(not(feature = "heif"))]
#[test]
fn heic_files_are_refused() {
    // The 24-byte `ftyp` box of a HEIC file: major brand `heic`, compatible `mif1` and `heic`.
    let ftyp = [&[0u8, 0, 0, 24][..], b"ftypheic", &[0, 0, 0, 0], b"mif1heic"].concat();
    let data = vectorcraft_format::base64_encode(&ftyp);
    let err = Session::new().execute("document.open", &json!({"name": "photo.heic", "dataBase64": data})).unwrap_err();
    assert!(err.to_string().contains("HEIC and HEIF images can't be opened in this build"), "{err}");
}
