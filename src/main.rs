use image::{ImageFormat, DynamicImage, ImageError};
use std::path::Path;

fn compress_image(input_path: &str, output_path: &str, quality: u8) -> Result<(), ImageError> {
    // 画像を読み込む
    let img = image::open(input_path)?;

    // JPEG形式で圧縮
    let mut output_file = std::fs::File::create(output_path)?;
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, quality);

    // 画像をエンコードして保存
    encoder.encode_image(&img)?;

    println!(
        "Image compressed successfully: {} -> {} with quality {}",
        input_path, output_path, quality
    );

    Ok(())
}

fn main() {
    let input_path = "image/pastel_blue.png";  // 入力ファイルのパス
    let output_path = "image/output.jpg"; // 出力ファイルのパス
    let quality = 10; // 圧縮品質 (1〜100)

    match compress_image(input_path, output_path, quality) {
        Ok(_) => println!("Compression completed successfully!"),
        Err(e) => eprintln!("Failed to compress image: {}", e),
    }
}