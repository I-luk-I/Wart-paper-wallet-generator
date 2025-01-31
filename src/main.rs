use image::{ImageFormat, load_from_memory_with_format, open, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use ab_glyph::{FontArc, PxScale};
use paper_wallet::Wartkey;
use paper_wallet::Wallet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;
use rayon::prelude::*;

#[derive(Debug,Parser)]
struct Countwallet{
    #[arg(short, long, default_value_t = 1)]
    count:i32
}
fn main() {
    let countwallet = Countwallet::parse();

    let time = std::time::Instant::now();

    (0..countwallet.count).into_par_iter().for_each(|i| {

        let wallet_png: &[u8] = include_bytes!("../wallet.png");

        let mut image = load_from_memory_with_format(wallet_png, ImageFormat::Png).unwrap().to_rgba8();

        let font_data = include_bytes!("../dejavu/DejaVuSans-BoldOblique.ttf");

        let pair_key_address = Wartkey::new();

        let font = FontArc::try_from_slice(font_data).unwrap();

        let color = Rgba([0, 0, 0, 200]);

        let scale_priv_key = PxScale::from(15.8);

        let scale_address = PxScale::from(20.0);

        let (width, height) = image.dimensions();

        let (text_width, text_height) = text_size(scale_priv_key, &font, pair_key_address.get_priv_key());

        let (text_width, text_height) = text_size(scale_address, &font, pair_key_address.get_address());

        let x = width - text_width;

        let y = height - text_height;

        draw_text_mut(&mut image, color, (x - 71) as i32, (y - 212) as i32, scale_priv_key, &font, pair_key_address.get_priv_key());

        draw_text_mut(&mut image, color, (x - 71) as i32, (y - 126) as i32, scale_address, &font, pair_key_address.get_address());

        let path = env::current_exe().unwrap();

        let dir = path.parent().unwrap();

        let target_dir: PathBuf = [dir.to_str().unwrap(), "paperwallet/"].iter().collect::<PathBuf>();

        if !target_dir.exists() {
            fs::create_dir_all(&target_dir).unwrap();
        }
        let image_path = target_dir.join(format!("wallet{}.png", i));

        image.save(&image_path).unwrap();
    });
    println!("Done \u{2705}  time - {}",time.elapsed().as_secs_f64())
}