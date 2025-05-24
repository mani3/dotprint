use clap::Parser;
use rust_embed::RustEmbed;
use std::io::{self, BufReader, Cursor};

use bdf_parser::bdf::{
    parser::BdfParser,
    renderer::{concat_bitmaps, render_bitmap},
};

#[derive(RustEmbed)]
#[folder = "fonts/"]
struct Assets;

#[derive(Parser)]
#[command(
    name = "dotprint",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "Render text using a bitmap font",
)]
struct Cli {
    /// Rendering text
    text: String,

    /// Rendering pixel character (default: "＠")
    #[arg(default_value = "＠")]
    pixel: String,

    /// Rendering space character (default: "　")
    #[arg(default_value = "　")]
    space: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    // 埋め込みフォントを取得
    if let Some(font) = Assets::get("misaki_gothic.bdf") {
        let cursor = Cursor::new(font.data.into_owned());
        let reader = BufReader::new(cursor);
        let bdf = BdfParser::parse(reader)?;

        // pixel, space は先頭の文字を利用
        let pixel = args.pixel.chars().next().unwrap_or('＠');
        let space = args.space.chars().next().unwrap_or('　');

        // text の各文字についてビットマップを取得
        let bitmaps: Vec<_> = args
            .text
            .chars()
            .filter_map(|c| match bdf.get_bitmap(c as u32) {
                Some(bitmap) => Some(bitmap),
                None => {
                    eprintln!(
                        "Warning: Character '{}' (U+{:04X}) not found in BDF.",
                        c, c as u32
                    );
                    None
                }
            })
            .collect();

        if bitmaps.is_empty() {
            eprintln!("No valid characters found.");
            std::process::exit(1);
        }

        // 取得したビットマップを連結し、レンダリングして出力
        let concatenated_bitmap = concat_bitmaps(bitmaps);
        let rendered_text =
            render_bitmap(&concatenated_bitmap, &pixel.to_string(), &space.to_string());
        println!("{}", rendered_text);
    } else {
        eprintln!("BDF file not found");
        std::process::exit(1);
    }

    Ok(())
}
