use std::io::Cursor;
use std::io::BufReader;

use bdf_parser::bdf::{
    parser::BdfParser,
    renderer::{concat_bitmaps, render_bitmap},
};

#[test]
fn test_empty_input_text() {
    let font_data = include_bytes!("../fonts/misaki_gothic.bdf");
    let cursor = Cursor::new(font_data);
    let reader = BufReader::new(cursor);
    let bdf = BdfParser::parse(reader).unwrap();

    let bitmaps: Vec<_> = "".chars().filter_map(|c| bdf.get_bitmap(c as u32)).collect();
    assert!(bitmaps.is_empty(), "Bitmaps should be empty for empty input text");
}

#[test]
fn test_default_pixel_and_space() {
    let font_data = include_bytes!("../fonts/misaki_gothic.bdf");
    let cursor = Cursor::new(font_data);
    let reader = BufReader::new(cursor);
    let bdf = BdfParser::parse(reader).unwrap();

    let bitmaps: Vec<_> = "A".chars().filter_map(|c| bdf.get_bitmap(c as u32)).collect();
    let concatenated_bitmap = concat_bitmaps(bitmaps);
    let rendered_text = render_bitmap(&concatenated_bitmap, "＠", "　");

    assert!(rendered_text.contains("＠"), "Rendered text should contain the default pixel character");
    assert!(rendered_text.contains("　"), "Rendered text should contain the default space character");
}

#[test]
fn test_custom_pixel_and_space() {
    let font_data = include_bytes!("../fonts/misaki_gothic.bdf");
    let cursor = Cursor::new(font_data);
    let reader = BufReader::new(cursor);
    let bdf = BdfParser::parse(reader).unwrap();

    let bitmaps: Vec<_> = "B".chars().filter_map(|c| bdf.get_bitmap(c as u32)).collect();
    let concatenated_bitmap = concat_bitmaps(bitmaps);
    let rendered_text = render_bitmap(&concatenated_bitmap, "#", ".");

    assert!(rendered_text.contains("#"), "Rendered text should contain the custom pixel character");
    assert!(rendered_text.contains("."), "Rendered text should contain the custom space character");
}

#[test]
fn test_unsupported_character_warning() {
    let font_data = include_bytes!("../fonts/misaki_gothic.bdf");
    let cursor = Cursor::new(font_data);
    let reader = BufReader::new(cursor);
    let bdf = BdfParser::parse(reader).unwrap();

    let unsupported_char = '😀';
    let bitmap = bdf.get_bitmap(unsupported_char as u32);
    assert!(bitmap.is_none(), "Bitmap should be None for unsupported characters");
}

#[test]
fn test_multiple_characters_rendering() {
    let font_data = include_bytes!("../fonts/misaki_gothic.bdf");
    let cursor = Cursor::new(font_data);
    let reader = BufReader::new(cursor);
    let bdf = BdfParser::parse(reader).unwrap();

    let bitmaps: Vec<_> = "ABC".chars().filter_map(|c| bdf.get_bitmap(c as u32)).collect();
    assert_eq!(bitmaps.len(), 3, "Bitmaps should contain three items for 'ABC'");

    let concatenated_bitmap = concat_bitmaps(bitmaps);
    let rendered_text = render_bitmap(&concatenated_bitmap, "*", " ");
    assert!(rendered_text.contains("*"), "Rendered text should contain the custom pixel character");
}
