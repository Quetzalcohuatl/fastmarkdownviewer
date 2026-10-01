use std::{env, io};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=assets/windows.manifest");
    println!("cargo:rerun-if-changed=assets/icons/windows/app.ico");

    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return Ok(());
    }

    winresource::WindowsResource::new()
        .set_icon("assets/icons/windows/app.ico")
        .set_manifest_file("assets/windows.manifest")
        .set("ProductName", "FastMarkdownViewer")
        .set("FileDescription", "FastMarkdownViewer")
        .set("LegalCopyright", "FastMarkdownViewer contributors")
        .compile()?;
    Ok(())
}
