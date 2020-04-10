use crate::palette::Palette;
use std::error::Error;
use std::fs;
use std::path::Path;

fn to_esc_sequences(p: &Palette) -> String {
    let mut s = String::new();

    s.push_str(&format!(
        "\u{001b}]4;0;{}\u{001b}\\",
        p.shades.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;1;{}\u{001b}\\",
        p.red.0.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;2;{}\u{001b}\\",
        p.yellow.0.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;3;{}\u{001b}\\",
        p.lime.0.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;4;{}\u{001b}\\",
        p.aqua.0.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;5;{}\u{001b}\\",
        p.blue.0.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;6;{}\u{001b}\\",
        p.magenta.0.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;7;{}\u{001b}\\",
        p.shades.silver.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;8;{}\u{001b}\\",
        p.shades.gray.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;9;{}\u{001b}\\",
        p.red.1.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;10;{}\u{001b}\\",
        p.yellow.1.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;11;{}\u{001b}\\",
        p.lime.1.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;12;{}\u{001b}\\",
        p.aqua.1.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;13;{}\u{001b}\\",
        p.blue.1.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;14;{}\u{001b}\\",
        p.magenta.1.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;15;{}\u{001b}\\",
        p.shades.white.to_hex_string()
    ));

    s.push_str(&format!(
        "\u{001b}]10;{}\u{001b}\\",
        p.shades.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]11;{}\u{001b}\\",
        p.shades.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]12;{}\u{001b}\\",
        p.shades.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]13;{}\u{001b}\\",
        p.shades.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]17;{}\u{001b}\\",
        p.shades.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]19;{}\u{001b}\\",
        p.shades.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;232;{}\u{001b}\\",
        p.shades.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;256;{}\u{001b}\\",
        p.shades.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]708;{}\u{001b}\\",
        p.shades.black.to_hex_string()
    ));

    s
}

pub fn write_sequences(p: &Palette, file: &Path) -> Result<(), Box<dyn Error>> {
    fs::write(file, to_esc_sequences(p))?;

    Ok(())
}
