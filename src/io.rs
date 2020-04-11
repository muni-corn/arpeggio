use crate::palette::Palette;
use crate::errors::ArpeggioError;
use std::fs;
use std::path::Path;

fn to_esc_sequences(p: &Palette) -> String {
    let mut s = String::new();

    s.push_str(&format!(
        "\u{001b}]4;0;{}\u{001b}\\",
        p.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;1;{}\u{001b}\\",
        p.maroon.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;2;{}\u{001b}\\",
        p.olive.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;3;{}\u{001b}\\",
        p.green.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;4;{}\u{001b}\\",
        p.teal.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;5;{}\u{001b}\\",
        p.navy.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;6;{}\u{001b}\\",
        p.purple.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;7;{}\u{001b}\\",
        p.silver.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;8;{}\u{001b}\\",
        p.gray.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;9;{}\u{001b}\\",
        p.red.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;10;{}\u{001b}\\",
        p.yellow.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;11;{}\u{001b}\\",
        p.lime.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;12;{}\u{001b}\\",
        p.aqua.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;13;{}\u{001b}\\",
        p.blue.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;14;{}\u{001b}\\",
        p.magenta.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;15;{}\u{001b}\\",
        p.white.to_hex_string()
    ));

    s.push_str(&format!(
        "\u{001b}]10;{}\u{001b}\\",
        p.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]11;{}\u{001b}\\",
        p.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]12;{}\u{001b}\\",
        p.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]13;{}\u{001b}\\",
        p.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]17;{}\u{001b}\\",
        p.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]19;{}\u{001b}\\",
        p.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;232;{}\u{001b}\\",
        p.black.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]4;256;{}\u{001b}\\",
        p.white.to_hex_string()
    ));
    s.push_str(&format!(
        "\u{001b}]708;{}\u{001b}\\",
        p.black.to_hex_string()
    ));

    s
}

pub fn write_sequences(p: &Palette, file: &Path) -> Result<(), ArpeggioError> {
    fs::write(file, to_esc_sequences(p))?;

    Ok(())
}
