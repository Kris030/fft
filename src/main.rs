#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]
#![feature(split_array)]

use std::{fs::File, io::BufReader, time::Duration};

use colors_transform::Color;
use num::{complex::Complex32 as C32, Zero};

pub mod fft;
use fft::fft;
use rodio::Source;

fn draw_spectogram<S, C>(
    source: S,
    channels: u16,
    duration: Duration,
    sample_rate: u32,
    sample_size: usize,
    step: usize,
    mut color: C,
) -> anyhow::Result<image::RgbImage>
where
    <S as Iterator>::Item: rodio::Sample,
    f32: rodio::cpal::FromSample<<S as Iterator>::Item> + rodio::Sample,
    S: rodio::Source,
    C: FnMut(f32) -> image::Rgb<u8>,
{
    let overlap = sample_size / step;

    let mut img = image::RgbImage::new(
        (duration.as_secs() * sample_rate as u64).div_ceil(overlap as u64) as u32,
        (sample_size / 2) as u32,
    );

    let mut source = source
        .convert_samples::<f32>()
        .step_by(channels as usize)
        .map(C32::from)
        .chain(std::iter::repeat(C32::zero()));

    let mut buff = vec![C32::zero(); sample_size];
    for x in 0..img.width() {
        if x % 256 == 0 {
            eprintln!("x: {x}");
        }

        // copy the second half into to first half
        buff.copy_within((sample_size - overlap)..sample_size, 0);

        // fill the second with new data
        buff[(sample_size - overlap)..].fill_with(|| source.next().expect("Not enough samples???"));

        let freqs = fft::<f32, false>(&buff);

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

fn main() -> anyhow::Result<()> {
    let reader = rodio::Decoder::new(BufReader::new(File::open("test2.wav")?))?;

    let sample_rate = reader.sample_rate();
    let channels = reader.channels();
    let duration = reader
        .total_duration()
        .ok_or(anyhow::Error::msg("We don't know total duration"))?;

    eprintln!("sample_rate: {sample_rate} channels: {channels} duration: {duration:?}");

    let (sample_size, step) = (1 << 12, 2);

    let img = draw_spectogram(
        reader,
        channels,
        duration,
        sample_rate,
        sample_size,
        step,
        |fq| {
            let rgb: [f32; 3] = colors_transform::Hsl::from(fq * 360., 100., fq * 100.)
                .to_rgb()
                .as_tuple()
                .into();

            rgb.map(|f| f as u8).into()
        },
    )?;

    img.save("test.png")?;

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
