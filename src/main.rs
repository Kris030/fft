#![feature(vec_push_within_capacity)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]
#![feature(split_array)]

use num::complex::Complex32 as C32;

pub mod fft;
pub mod mod_freqs;
pub mod spectogram;

use mod_freqs::ModFreqs;

// fn main() -> anyhow::Result<()> {
//     use std::{fs::File, io::BufReader};
//     use colors_transform::Color;
//     use rodio::Source;
//     use spectogram::draw_spectogram;

//     let path = std::env::args()
//         .nth(1)
//         .unwrap_or_else(|| String::from("input.wav"));

//     let reader = rodio::Decoder::new(BufReader::new(File::open(path)?))?;

//     let sample_rate = reader.sample_rate();
//     let channels = reader.channels();
//     let duration = reader
//         .total_duration()
//         .ok_or(anyhow::Error::msg("We don't know total duration"))?;

//     eprintln!("sample_rate: {sample_rate} channels: {channels} duration: {duration:?}");

//     let (sample_size, step) = (1 << 12, 2);

//     let source = reader
//         .convert_samples::<f32>()
//         .step_by(channels as usize)
//         .map(C32::from);

//     let img = draw_spectogram(source, duration, sample_rate, sample_size, step, |fq| {
//         let rgb: [f32; 3] = colors_transform::Hsl::from(fq * 360., 100., fq * 100.)
//             .to_rgb()
//             .as_tuple()
//             .into();

//         rgb.map(|f| f as u8).into()
//     })?;

//     img.save("spectogram.png")?;

//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    let sample_size = 1 << 15;

    let mut reader = hound::WavReader::open("input.wav")?;
    let mut writer = hound::WavWriter::create("output.wav", reader.spec())?;

    let channels = reader.spec().channels as usize;

    let source = reader
        .samples::<i16>()
        .step_by(channels)
        .map(|s| C32::new(s.unwrap() as f32 / i16::MAX as f32, 0.));

    let source = ModFreqs::new(source, |f| f, sample_size);

    for cx in source {
        writer.write_sample((cx.re * i16::MAX as f32).round() as i16)?;
    }

    Ok(())
}
