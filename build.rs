use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=assets/windows.manifest");

    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let icon = out_dir.join("fast-markdown-viewer.ico");
    fs::write(&icon, make_icon())?;

    winresource::WindowsResource::new()
        .set_icon(icon.to_string_lossy().as_ref())
        .set_manifest_file("assets/windows.manifest")
        .set("ProductName", "FastMarkdownViewer")
        .set("FileDescription", "FastMarkdownViewer")
        .set("LegalCopyright", "FastMarkdownViewer contributors")
        .compile()?;
    Ok(())
}

/// Produce a deterministic 32-bit BGRA icon without requiring a binary source asset.
fn make_icon() -> Vec<u8> {
    const SIZE: usize = 32;
    const SIZE_U8: u8 = 32;
    const SIZE_I32: i32 = 32;
    let row_bytes = SIZE * 4;
    let xor_size = row_bytes * SIZE;
    let and_row_bytes = SIZE.div_ceil(32) * 4;
    let and_size = and_row_bytes * SIZE;
    let image_size = 40 + xor_size + and_size;

    let mut out = Vec::with_capacity(22 + image_size);
    out.extend_from_slice(&0_u16.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.push(SIZE_U8);
    out.push(SIZE_U8);
    out.push(0);
    out.push(0);
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&32_u16.to_le_bytes());
    out.extend_from_slice(
        &u32::try_from(image_size)
            .expect("icon fits in u32")
            .to_le_bytes(),
    );
    out.extend_from_slice(&22_u32.to_le_bytes());

    out.extend_from_slice(&40_u32.to_le_bytes());
    out.extend_from_slice(&SIZE_I32.to_le_bytes());
    out.extend_from_slice(&(SIZE_I32 * 2).to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&32_u16.to_le_bytes());
    out.extend_from_slice(&0_u32.to_le_bytes());
    out.extend_from_slice(
        &u32::try_from(xor_size)
            .expect("icon fits in u32")
            .to_le_bytes(),
    );
    out.extend_from_slice(&0_i32.to_le_bytes());
    out.extend_from_slice(&0_i32.to_le_bytes());
    out.extend_from_slice(&0_u32.to_le_bytes());
    out.extend_from_slice(&0_u32.to_le_bytes());

    for y in (0..SIZE).rev() {
        for x in 0..SIZE {
            let inside_page = (5..27).contains(&x) && (3..29).contains(&y);
            let fold = x >= 21 && y < 9 && x - 21 >= 8 - y;
            let line = (9..24).contains(&x) && matches!(y, 12 | 17 | 22) && (!fold || y >= 12);
            let (r, g, b, a) = if line {
                (31, 41, 55, 255)
            } else if inside_page && !fold {
                (243, 244, 246, 255)
            } else if inside_page {
                (145, 164, 188, 255)
            } else {
                (30, 111, 214, 255)
            };
            out.extend_from_slice(&[b, g, r, a]);
        }
    }
    out.resize(out.len() + and_size, 0);
    out
}
