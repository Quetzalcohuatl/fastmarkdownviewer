use std::{io::Write as _, path::Path};

use fast_markdown_viewer::{
    document::Document,
    links::{self, LinkAction},
    math::render_formula_svg,
};

#[test]
fn loads_documents_beyond_the_former_32_mib_cutoff() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    let block = "Ordinary Markdown text.\n".repeat(50_000);
    for _ in 0..30 {
        file.write_all(block.as_bytes()).unwrap();
    }
    file.write_all(b"\n# End marker\n").unwrap();
    let document = Document::load(file.path()).unwrap();
    assert!(document.source.len() > 32 * 1024 * 1024);
    assert!(document.source.ends_with("# End marker\n"));
}

#[test]
fn accepts_unicode_and_spaces_in_paths() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("こんにちは notes.md");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all("# Καλημέρα 🌍".as_bytes()).unwrap();
    let document = Document::load(&path).unwrap();
    assert!(document.source.contains("Καλημέρα"));
    assert!(document.base_uri.starts_with("file:"));
}

#[test]
fn link_policy_handles_queries_fragments_and_unsafe_schemes() {
    let source = Path::new("C:/notes/start.md");
    assert!(matches!(
        links::resolve("https://example.com/a?b=c#d", Some(source)),
        LinkAction::Browser(_)
    ));
    assert_eq!(
        links::resolve("#same-page", Some(source)),
        LinkAction::Anchor("same-page".into())
    );
    assert_eq!(
        links::resolve("mailto:test@example.com", Some(source)),
        LinkAction::Inert
    );
}

#[test]
fn ratex_compatibility_corpus_renders_to_svg() {
    for line in include_str!("ratex-cases.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let svg = render_formula_svg(line, false, 1.5, [30, 32, 35, 255]).unwrap();
        assert!(
            svg.starts_with(b"<svg"),
            "formula did not produce SVG: {line}"
        );
    }
}
