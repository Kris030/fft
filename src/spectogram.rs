use std::time::Duration;

use num::{complex::Complex32 as C32, Zero};

use crate::fft::fft;

pub fn draw_spectogram<S, C>(
    mut source: S,
    duration: Duration,
    sample_rate: u32,
    sample_size: usize,
    step: usize,
    mut color: C,
) -> anyhow::Result<image::RgbImage>
where
    S: Iterator<Item = C32>,
    C: FnMut(f32) -> image::Rgb<u8>,
{
    let overlap = sample_size / step;

    let mut img = image::RgbImage::new(
        (duration.as_secs() * sample_rate as u64).div_ceil(overlap as u64) as u32,
        (sample_size / 2) as u32,
    );

    let mut buff = vec![C32::zero(); sample_size];
    for x in 0..img.width() {
        if x % 256 == 0 {
            eprintln!("x: {x}");
        }

        // copy the second half into to first half
        buff.copy_within((sample_size - overlap)..sample_size, 0);

        // fill the second with new data
        buff[(sample_size - overlap)..].fill_with(|| source.next().expect("Not enough samples???"));

        let freqs = fft(&buff);

        #[allow(clippy::needless_range_loop)]
        for y in 0..(sample_size / 2) {
            let fq = freqs[y].norm();
            img.put_pixel(x, img.height() - 1 - y as u32, color(fq));
        }
    }

    println!(
        "samples left: {}",
        source.take_while(|c| !c.is_zero()).count()
    );

    Ok(img)
}
