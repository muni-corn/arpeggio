use crate::errors::ArpeggioError;
use crate::palette::Palette;
use std::fs;
use std::path::Path;

fn to_esc_sequences(p: &Palette) -> String {
    let mut s = String::new();

    s.push_str(&get_sequence_string(0, &p.black.to_hex_string()));
    s.push_str(&get_sequence_string(1, &p.maroon.to_hex_string()));
    s.push_str(&get_sequence_string(2, &p.green.to_hex_string()));
    s.push_str(&get_sequence_string(3, &p.olive.to_hex_string()));
    s.push_str(&get_sequence_string(4, &p.navy.to_hex_string()));
    s.push_str(&get_sequence_string(5, &p.purple.to_hex_string()));
    s.push_str(&get_sequence_string(6, &p.teal.to_hex_string()));
    s.push_str(&get_sequence_string(7, &p.silver.to_hex_string()));
    s.push_str(&get_sequence_string(8, &p.gray.to_hex_string()));
    s.push_str(&get_sequence_string(9, &p.red.to_hex_string()));
    s.push_str(&get_sequence_string(10, &p.lime.to_hex_string()));
    s.push_str(&get_sequence_string(11, &p.yellow.to_hex_string()));
    s.push_str(&get_sequence_string(12, &p.blue.to_hex_string()));
    s.push_str(&get_sequence_string(13, &p.magenta.to_hex_string()));
    s.push_str(&get_sequence_string(14, &p.aqua.to_hex_string()));
    s.push_str(&get_sequence_string(15, &p.white.to_hex_string()));

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

pub fn get_sequence_string(number: i32, color_string: &str) -> String {
    format!("\u{001b}]4;{};{}\u{001b}\\", number, color_string)
}

pub fn write_sequences<P: AsRef<Path>>(p: &Palette, to: P) -> Result<(), ArpeggioError> {
    fs::write(to, to_esc_sequences(p))?;

    Ok(())
}
