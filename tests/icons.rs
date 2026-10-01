use fast_markdown_viewer::platform;

#[test]
fn runtime_icon_is_large_transparent_and_reusable() {
    let icon = platform::app_icon();
    assert_eq!((icon.width, icon.height), (256, 256));
    assert_eq!(icon.rgba.len(), 256 * 256 * 4);
    assert_eq!(
        icon.rgba[3], 0,
        "Outside the silhouette must be transparent"
    );
    assert!(icon.rgba.chunks_exact(4).any(|p| p[3] == 255));
    assert!(icon.rgba.chunks_exact(4).any(|p| p[3] > 0 && p[3] < 255));
    assert_eq!(icon.rgba, platform::app_icon().rgba);
}
