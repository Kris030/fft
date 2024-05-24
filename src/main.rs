#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]
#![feature(split_array)]

use colors_transform::Color;
use num::{complex::Complex32 as C32, Zero};

pub mod fft;
use fft::fft;

fn main() -> anyhow::Result<()> {
    const SAMPLE_SIZE: usize = 1 << 12;
    const STEP: usize = 2;
    const OVERLAP: usize = SAMPLE_SIZE / STEP;

    let mut reader = hound::WavReader::open("test2.wav")?;
    let channels = reader.spec().channels as usize;
    let duration = reader.duration();

    let mut source = reader
        .samples::<i16>()
        .step_by(channels)
        .map(|s| C32::new(s.unwrap() as f32 / i16::MAX as f32, 0.))
        .chain(std::iter::repeat(C32::zero()));

    let mut img = image::RgbImage::new(duration.div_ceil(OVERLAP as u32), SAMPLE_SIZE as u32 / 2);

    let mut c = [C32::zero(); SAMPLE_SIZE];
    for x in 0..img.width() {
        if x % 256 == 0 {
            eprintln!("x: {x}");
        }

        // copy the second half into to first half
        c.copy_within((SAMPLE_SIZE - OVERLAP)..SAMPLE_SIZE, 0);

        // fill the second with new data
        let b = &mut c[(SAMPLE_SIZE - OVERLAP)..];
        b.fill_with(|| source.next().unwrap());

        let freqs = fft::<f32, false>(&c);

        #[allow(clippy::needless_range_loop)]
        for y in 0..(SAMPLE_SIZE / 2) {
            let fq = freqs[y].norm();

            let rgb: [f32; 3] = colors_transform::Hsl::from(fq * 360., 100., fq * 100.)
                .to_rgb()
                .as_tuple()
                .into();

            let rgb = rgb.map(|f| f as u8).into();

            img.put_pixel(x, img.height() - 1 - y as u32, rgb);
        }
    }

    img.save("test.png")?;
    println!(
        "samples left: {}",
        source.take_while(|c| !c.is_zero()).count()
    );

    Ok(())
}

/*
pub fn f(c: C32) -> C32 {
    c
}

fn main() -> anyhow::Result<()> {
    const SAMPLE_SIZE: usize = 1 << 14;

    let mut reader = hound::WavReader::open("sine.wav")?;
    let mut writer = hound::WavWriter::create("output.wav", reader.spec())?;

    let source = reader
        .samples::<i16>()
        .map(|s| C32::new(s.unwrap() as f32 / i16::MAX as f32, 0.));

    for (i, c) in source.array_chunks::<SAMPLE_SIZE>().enumerate() {
        eprintln!("chunk #{i}");

        let mut freqs = fft::<false>(&c);
        for fq in &mut freqs {
            *fq /= SAMPLE_SIZE as f32;
            // println!("{}", fq.norm());

            *fq = f(*fq);
        }

        // let mut freqs = vec![C32::zero(); SAMPLE_SIZE];
        // freqs[SAMPLE_SIZE / 5] = C32::one();

        let res = fft::<true>(&freqs);

        for cx in res.into_iter() {
            if cx.re.abs() > 0.5 {
                eprintln!("{}", cx.re);
            }

            println!("{}", cx.re);
            let s = (cx.re * i16::MAX as f32).round() as i16;
            // println!("{}", s);

            writer.write_sample(s)?;
        }

        if true {
            break;
        }
    }

    Ok(())
}
 */
