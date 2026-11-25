# KatWork fork of VectorCraft

This repository is the KatWork fork of VectorCraft (https://github.com/storytold/vectorcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on VectorCraft by the ArtCraft team (https://github.com/storytold/vectorcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `b1a1c0b7a83e872268a9cf9332bf103d1a7438d1`
- Upstream `storytold/vectorcraft` head at that time: `99f7de526289f9e50bf387b81d1fcefe904a0f15`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `fix(katwork): the color wheel's opacity row builds again` (the fork base did not compile; the file as upstream has it at 99f7de526) | not needed: already fixed upstream; dropped when the fork syncs |
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): plugins feature, on by default` | draft, not opened (in the fork PR body) |

## The `plugins` feature

The WebAssembly plug-in host (`vectorcraft-plugins`, the wasmi interpreter) is the Cargo feature `plugins`, on by default in
`vectorcraft-engine` (the `plugin.*` commands, the Plug-ins folder loaded at startup, exporter and importer plug-ins),
`vectorcraft-effects` (live-effect plug-ins), `vectorcraft-ui-egui` (the plug-in menus,
dialogs, install prompt and Help › Safe Mode), the CLI and the web app; the desktop app always turns it on. An upstream build
is unchanged. KatWork builds the CLI and the web app with `--no-default-features` (no other feature): `vectorcraft-plugins`
and `wasmi` are then in neither graph (`scripts/katwork-profile.sh`, CI job `katwork-profile`). Without it:

- every `plugin.*` command is unknown (`list`, `info`, `install` with `dataBase64` or `path`, `inspect`, `run`, `remove`,
  `reload`, and any `plugin.<plug-in id>.<command>` id); no plug-in loads at startup and the Plug-ins folder is never read;
- a document that names a plug-in live effect (`plugin.<id>`) opens and draws as when that plug-in isn't installed (the
  object keeps its geometry), and keeps the effect when saved;
- File › Open With and exporter formats from plug-ins are absent; Install Plug-in… and Help › Safe Mode say plug-ins are not
  available in this build;
- the names the rest of the code takes from the plug-in crate come from inert stand-ins (`crates/engine/src/plugin_host.rs`,
  `crates/engine/src/cmd/plugin_off.rs`, `crates/engine/src/cmd/fileio/importer_off.rs`,
  `crates/ui-egui/src/dialogs/plugin_off.rs`, `crates/ui-egui/src/dialogs/safe_mode_off.rs`,
  `crates/effects/src/plugin_host.rs`); the text helpers scripts share with plug-ins (`sha256_hex`, `display_text`) are the
  same code, so scripts behave as before.

The CLI and web app keep `vectorcraft-engine`'s `bundled-fonts` and `heif` as before (their manifests name them, not a
default), so the KatWork build has HEIC/HEIF import as upstream's does.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/vectorcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.vectorcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The app icon's attribution (`ASSETS.md` rows, `assets/app-icon/LICENSE.txt`, `assets/app-icon/README.md` provenance): the
  icon is original artwork by the project owner, drawn in ArtCraft; its author and provenance lines are licence and attribution
  notices, which the licences and the repository's clean-room rule require. The icon itself is kept (R43) and shows no mark.
- PhotoCraft's Windows install folder (`ArtCraft\PhotoCraft`) in `apps/vectorcraft/src/host_services.rs` and
  `docs/control-protocol.md`: the desktop app finds an installed PhotoCraft there; an installer path, kept like this app's own.
- The Flatpak manifests' read-only permission for `xdg-data/ArtCraft/versions`: the version folder of the separate ArtCraft
  Launcher program; KatWork builds no Flatpak.
- The Windows install policy check (`packaging/windows/check-install-policy.ps1`) and its messages about the ArtCraft suite
  folder and the ArtCraft Launcher: installer paths and the separate launcher program.
- Upstream CI and release configuration (`.github/workflows/release.yml` secret name, `.github/workflows/windows-arm64.yml`
  install path): unchanged upstream CI.
- `crates/ui-egui/src/theme.rs` selects the theme id `craft.artcraft` for the Classic medium-dark brightness: an identifier
  defined by the shared `craft-libs` theme crate (a git dependency, outside this repository), which names that theme
  "ArtCraft" inside the library. The app's own menus show the brightness name ("Medium Dark"), not the theme name. Renaming it
  needs a change in `craft-libs`; it is left as is and noted here.
- The "Based on" line inside code (`crates/ui-egui/src/credits.rs`, `BASED_ON`): the About window's plain-text line.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.vectorcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.vectorcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/vectorcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/vectorcraft-macos-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/vectorcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/vectorcraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/vectorcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/vectorcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/cursors/cursor-anchor-point.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-direct-anchor-selected.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-direct-anchor.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-loaded-text-link.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-loaded-text.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-bracket.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-edge.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-line-level.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-line-rising.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-ring.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-square.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-mark-star.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-pen-convert.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-pen-curvature.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-pen-join.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-pen-new.svg`: kept (viewed: no ArtCraft mark)
- `assets/cursors/cursor-reshape-segment.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-center-horizontal.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-center-vertical.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-end-horizontal.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-end-vertical.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-horizontal-justify-center.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-horizontal-justify-end.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-horizontal-justify-start.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-start-horizontal.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-start-vertical.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-vertical-justify-center.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-vertical-justify-end.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/align-vertical-justify-start.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/arrow-left-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/arrow-up-down.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/blend.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/brush.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/chart-column.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/chevron-down.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/chevron-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/chevron-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/chevrons-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/chevrons-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/circle-off.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/circle.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/clipboard-paste.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/cloud.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/combine.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/copy.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-actions.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-bottom.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-center.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-hcenter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-top.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-al-vcenter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-align.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-alignto-artboard.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-alignto-key.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-alignto-selection.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-anchor-smooth.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-anchor.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-appearance.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arc-three-point.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arc.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arrow-down.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arrow-extend.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arrow-fill.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arrow-tip.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-arrow-up.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-artboard-options.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-artboards.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-blend-align-page.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-blend-align-path.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-blend.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-bloat.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-cap-butt.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-cap-round.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-cap-square.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-center-hide.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-center-show.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-circle-points.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-clear.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-color-guide.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-contour.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-corner-tool.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-crystallize.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-cube.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-curvature-circle.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-cycle-sizes.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dash-align.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dash-exact.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dimension.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dir-off.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dir-on.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-direct.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-bottom.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-hcenter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-hspace.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-top.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-vcenter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-dist-vspace.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-draw-behind.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-draw-inside.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-draw-normal.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-ellipse.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-fill-area.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-fill-none.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-filter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-flare.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-folder.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-free-distort.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-free-transform.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-fx.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-gamut.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-conic.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-diamond.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-freeform.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-linear.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-radial.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-stroke-across.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-stroke-along.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grad-stroke-within.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-gradient.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-area.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-bar.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-column.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-line.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-pie.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-radar.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-scatter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-stacked-bar.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graph-stacked-column.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-graphic-styles.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-grid-view.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-group-select.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-help.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-join-bevel.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-join-miter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-join-round.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-join.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-knife.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-lasso-enclosing.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-lasso-polygonal.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-lasso.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-line-cut.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-line.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-list-view.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-live-bucket.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-live-select.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-mask-none.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-measure.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-mesh.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-mirror-cut.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-new-fill.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-new-item.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-new-stroke.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-objects-on-path.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-options.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-bullets.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-center.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-justify-all.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-justify-center.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-justify-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-justify-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-left.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-ltr.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-numbers.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-right.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-para-rtl.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-path-eraser.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pathfinder.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-perspective-select.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-perspective.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-crop.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-divide.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-exclude.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-intersect.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-merge.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-minus-back.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-minus-front.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-outline.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-trim.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pf-unite.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-place-symbol.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-polar-grid.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-polygon.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-pucker.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-puppet.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-quick-pen.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rearrange.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-recolor-arrow-off.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-recolor-arrow.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rect-cut.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rect-grid.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-reference-point.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-remove-brush.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-reshape.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-reverse.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rotate-view.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rounded-rect.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rule-evenodd.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-rule-nonzero.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-scallop.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-screen-mode.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-selection.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-shape-builder.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-shared-edge.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-shear.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-slice-select.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-slice.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-slide-anchor.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-smooth.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-stroke-center.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-stroke-inside.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-stroke-outside.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-stroke.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-swap.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-swatch-global.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-swatch-kinds.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-swatch-process.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-swatch-spot.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-screener.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-scruncher.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-shifter.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-sizer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-spinner.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-sprayer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-stainer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-symbol-styler.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-touch-type.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-transform-panel.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-transparency.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-twirl.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-type-area.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-type-path.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-type-vertical-area.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-type-vertical-path.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-type-vertical.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-type.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-width-profile.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-width.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-wrinkle.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-zoom-large.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/dc-zoom-small.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/ellipsis.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/eraser.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/external-link.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/eye-off.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/eye.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/file-plus.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/flip-horizontal-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/flip-vertical-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/folder-open.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/frame.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/git-branch.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/globe.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/graduation-cap.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/grid-3x3.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/group.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/hand.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/hexagon.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/history.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/house.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/image.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/info.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/lasso.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/layers.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/layout-grid.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/library.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/link-2-off.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/link.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/lock-open.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/lock.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/map.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/maximize-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/menu.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/message-circle.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/minus.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/monitor.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/move-diagonal-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/move.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/paint-bucket.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/paintbrush.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/palette.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pen-tool-add.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pen-tool-delete.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pen-tool.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pencil.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pilcrow.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pin.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/pipette.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/plus.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/printer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/redo-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/rotate-3d.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/rotate-ccw.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/rotate-cw.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/ruler.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/save.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/scaling.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/scissors.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/search.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/settings.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/shapes.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/share-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/slice.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/sparkles.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/spline-pointer.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/spline.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/spray-can.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/square-dashed.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/square.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/squares-exclude.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/squares-intersect.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/squares-subtract.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/squares-unite.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/squircle.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/star.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/sun.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/swatch-book.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/text-cursor-input.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/tornado.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/trash-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/triangle.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/type.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/undo-2.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/ungroup.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/wand-sparkles.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/waves.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/x.svg`: kept (viewed: no ArtCraft mark)
- `assets/icons/zoom-in.svg`: kept (viewed: no ArtCraft mark)
- `docs/images/art-envelope.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-mesh.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-neon-drive.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-pathfinder.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-repeat.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-ribbons.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-still-life.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/dusk-poster.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-1-neon.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `docs/images/shot-2-ribbons.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `docs/images/shot-3-sheet.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `docs/images/shot-4-bezier.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `docs/images/shot-5-perspective.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `docs/images/shot-6-revolve.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `docs/images/shot-7-dimensions.png`: kept: screenshot of the upstream build (viewed); the app bar's Discord button is painted over with the bar's colour
- `examples/dusk-poster.svg`: kept (viewed: no ArtCraft mark)
- `packaging/macos/dmg/background.svg`: kept (viewed: no ArtCraft mark)
- `packaging/macos/dmg/background.tiff`: kept (viewed: no ArtCraft mark)
