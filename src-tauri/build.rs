use std::{fs, path::Path};

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn ensure_windows_icon() {
    let icon_path = Path::new("icons").join("icon.ico");
    if icon_path.exists() {
        return;
    }

    let mut icon = Vec::with_capacity(4_286);
    push_u16(&mut icon, 0);
    push_u16(&mut icon, 1);
    push_u16(&mut icon, 1);
    icon.extend_from_slice(&[32, 32, 0, 0]);
    push_u16(&mut icon, 1);
    push_u16(&mut icon, 32);
    push_u32(&mut icon, 4_264);
    push_u32(&mut icon, 22);

    push_u32(&mut icon, 40);
    push_u32(&mut icon, 32);
    push_u32(&mut icon, 64);
    push_u16(&mut icon, 1);
    push_u16(&mut icon, 32);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 4_096);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 0);

    for y in (0..32).rev() {
        for x in 0..32 {
            let dx = x - 15;
            let dy = y - 15;
            let is_island = (dx * dx) / 2 + (dy * dy) <= 150;
            let is_accent = (9..=22).contains(&x) && (14..=17).contains(&y);
            let (blue, green, red) = if is_accent {
                (72, 196, 244)
            } else if is_island {
                (12, 12, 14)
            } else {
                (26, 22, 20)
            };
            icon.extend_from_slice(&[blue, green, red, 255]);
        }
    }

    icon.resize(icon.len() + 128, 0);

    fs::create_dir_all("icons").expect("failed to create src-tauri/icons");
    fs::write(icon_path, icon).expect("failed to create the Windows application icon");
}

fn main() {
    ensure_windows_icon();
    tauri_build::build()
}
