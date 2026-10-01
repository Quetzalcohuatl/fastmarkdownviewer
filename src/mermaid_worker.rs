use crate::mermaid::Appearance;
use std::path::Path;

fn prepare(input: &Path, appearance: &Appearance) -> Result<resvg::usvg::Tree, String> {
    let source = std::fs::read_to_string(input).map_err(|e| e.to_string())?;
    let theme = diagram_theme(appearance);
    let mut options = resvg::usvg::Options::default();
    // Bundled egui fonts keep labels readable even on systems with no installed fonts.
    for font in eframe::egui::FontDefinitions::default().font_data.values() {
        options.fontdb_mut().load_font_data(font.font.to_vec());
    }
    for name in [
        "segoeui.ttf",
        "segoeuib.ttf",
        "segoeuii.ttf",
        "consola.ttf",
        "DejaVuSans.ttf",
        "DejaVuSans-Bold.ttf",
        "DejaVuSansMono.ttf",
        "Arial.ttf",
        "Arial Bold.ttf",
        "Menlo.ttc",
    ] {
        if let Some(path) = crate::font_paths::find(name)
            && let Ok(bytes) = std::fs::read(path)
        {
            options.fontdb_mut().load_font_data(bytes);
        }
    }
    let supplemental: &[&str] = if source.chars().any(|c| {
        matches!(c as u32,
        0x1100..=0x11ff | 0x3040..=0x31ff | 0x3400..=0x9fff | 0xac00..=0xd7af)
    }) {
        &[
            "msyh.ttc",
            "msgothic.ttc",
            "malgun.ttf",
            "NotoSansCJK-Regular.ttc",
            "PingFang.ttc",
            "AppleSDGothicNeo.ttc",
        ]
    } else {
        &[]
    };
    for name in supplemental {
        if let Some(path) = crate::font_paths::find(name)
            && let Ok(bytes) = std::fs::read(path)
        {
            options.fontdb_mut().load_font_data(bytes);
        }
    }
    let sans = ["Segoe UI", "DejaVu Sans", "Arial", "Ubuntu"]
        .into_iter()
        .find(|name| {
            options
                .fontdb
                .faces()
                .any(|face| face.families.iter().any(|(family, _)| family == name))
        })
        .ok_or("No usable diagram font is available")?;
    options.font_family = sans.into();
    options.fontdb_mut().set_sans_serif_family(sans);
    options.fontdb_mut().set_serif_family(sans);
    let mono = ["Consolas", "DejaVu Sans Mono", "Menlo", "Hack"]
        .into_iter()
        .find(|name| {
            options
                .fontdb
                .faces()
                .any(|face| face.families.iter().any(|(family, _)| family == name))
        })
        .unwrap_or(sans);
    options.fontdb_mut().set_monospace_family(mono);
    // Use the same default proportional font as egui. Only load an additional
    // system font when the reader has explicitly selected it.
    let selected = selected_font(&mut options, appearance.font.as_deref());
    let family = selected.as_deref().unwrap_or("Ubuntu");
    let scene =
        rusty_mermaid_diagrams::render_to_scene(&source, &theme).map_err(|e| e.to_string())?;
    let scene = with_font(&scene, family);
    let svg = rusty_mermaid_svg::SvgRenderer::with_theme(&theme).render_themed(&scene, &theme);
    resvg::usvg::Tree::from_str(&svg, &options).map_err(|e| e.to_string())
}

pub fn render(input: &Path, output: &Path, appearance: &Appearance) -> Result<(), String> {
    let tree = prepare(input, appearance)?;
    let size = tree.size().to_int_size();
    let mut pixels = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or("Cannot allocate pixels")?;
    let [red, green, blue] = appearance.background;
    pixels.fill(resvg::tiny_skia::Color::from_rgba8(red, green, blue, 255));
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixels.as_mut(),
    );
    pixels.save_png(output).map_err(|e| e.to_string())?;
    Ok(())
}

/// Rasterize at display resolution without allocating a whole-diagram bitmap.
#[allow(clippy::cast_precision_loss)]
pub fn render_tiles(
    input: &Path,
    output: &Path,
    appearance: &Appearance,
    request: crate::mermaid_tiles::RasterRequest,
) -> Result<(), String> {
    use crate::mermaid_tiles::{Geometry, TILE_EDGE, tile_path};
    let tree = prepare(input, appearance)?;
    let geometry = Geometry::new(tree.size().width(), tree.size().height(), request)?;
    let directory = output.parent().ok_or("Missing diagram directory")?;
    let temporary = output.with_extension("tmp");
    std::fs::write(
        &temporary,
        serde_json::to_vec(&geometry).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    std::fs::rename(temporary, output).map_err(|e| e.to_string())?;
    let scale = geometry.width as f32 / geometry.source_width;
    let [red, green, blue] = appearance.background;
    for row in 0..geometry.height.div_ceil(TILE_EDGE) {
        for column in 0..geometry.width.div_ceil(TILE_EDGE) {
            let x = column * TILE_EDGE;
            let y = row * TILE_EDGE;
            let mut pixels = resvg::tiny_skia::Pixmap::new(
                (geometry.width - x).min(TILE_EDGE),
                (geometry.height - y).min(TILE_EDGE),
            )
            .ok_or("Cannot allocate diagram tile")?;
            pixels.fill(resvg::tiny_skia::Color::from_rgba8(red, green, blue, 255));
            resvg::render(
                &tree,
                resvg::tiny_skia::Transform::from_row(
                    scale,
                    0.0,
                    0.0,
                    scale,
                    -(x as f32),
                    -(y as f32),
                ),
                &mut pixels.as_mut(),
            );
            let destination = tile_path(directory, column, row);
            let temporary = destination.with_extension("tmp");
            pixels.save_png(&temporary).map_err(|e| e.to_string())?;
            std::fs::rename(temporary, destination).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn selected_font(options: &mut resvg::usvg::Options<'_>, file: Option<&str>) -> Option<String> {
    let path = crate::font_paths::find(file?)?;
    let bytes = std::fs::read(path).ok()?;
    let ids = options
        .fontdb_mut()
        .load_font_source(resvg::usvg::fontdb::Source::Binary(std::sync::Arc::new(
            bytes,
        )));
    options
        .fontdb
        .face(*ids.first()?)?
        .families
        .first()
        .map(|(name, _)| name.clone())
}

fn diagram_theme(appearance: &Appearance) -> rusty_mermaid_core::Theme {
    use rusty_mermaid_core::{Color, Theme};
    let color = |[r, g, b]: [u8; 3]| Color::rgb(r, g, b);
    let background = color(appearance.background);
    let surface = color(appearance.surface);
    let text = color(appearance.text);
    let muted = color(appearance.muted);
    let accent = color(appearance.accent);
    Theme {
        background,
        node_fill: surface,
        node_stroke: accent,
        node_text: text,
        edge_stroke: text,
        edge_label_text: text,
        edge_label_bg: background,
        start_fill: text,
        end_inner_fill: accent,
        composite_fill: background,
        composite_stroke: accent,
        composite_label: text,
        note_fill: surface,
        note_stroke: accent,
        note_text: text,
        subgraph_fill: surface,
        subgraph_stroke: muted,
        subgraph_label: text,
        divider_stroke: muted,
        region_stroke: muted,
        lifeline_stroke: muted,
        activation_fill: color(appearance.selection),
        activation_stroke: accent,
        grid_stroke: muted,
        muted_text: muted,
        face_fill: surface,
        detail_stroke: text,
        ..Theme::default()
    }
}

fn with_font(scene: &rusty_mermaid_core::Scene, family: &str) -> rusty_mermaid_core::Scene {
    use rusty_mermaid_core::{Primitive, Scene};
    fn set_font(primitive: &mut Primitive, family: &str) {
        match primitive {
            Primitive::Text { style, .. } => family.clone_into(&mut style.font_family),
            Primitive::Group { children, .. } => {
                for child in children {
                    set_font(child, family);
                }
            }
            _ => {}
        }
    }
    let mut result = Scene::new(scene.width, scene.height);
    for element in scene.elements() {
        let mut primitive = element.primitive.clone();
        set_font(&mut primitive, family);
        if let Some(id) = &element.id {
            result.push_identified(primitive, id.clone());
        } else {
            result.push(primitive);
        }
    }
    result
}
