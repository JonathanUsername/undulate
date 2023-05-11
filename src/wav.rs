use anyhow::Result;
use hound::WavReader;
use log::{debug, error, info, warn};

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct WavFileSummary {
    source_file: String,
    sample_rate: u32,
    bits: u16,
    samples_per_pixel: u32,
    time_duration: f64,
    processed_time_duration: f64,
    samples_length: usize,
    pub samples: Vec<SampleOverview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleOverview {
    min: i32,
    max: i32,
    rms: f32,
}

// See what the RMS stand for https://manual.audacityteam.org/man/glossary.html#rms
fn calculate_rms(samples: &Vec<i32>) -> f32 {
    let sqr_sum = samples.iter().fold(0.0, |sqr_sum, s| {
        let sample = *s as f32;
        sqr_sum + sample * sample
    });
    (sqr_sum / samples.len() as f32).sqrt()
}

pub fn extract_rms_samples(
    filename: &str,
    mut samples_per_pixel: u32,
    width: &u32,
) -> Result<Vec<SampleOverview>> {
    let mut reader: WavReader<io::BufReader<File>> = hound::WavReader::open(filename)?;

    let samples: Vec<i32> = reader.samples::<i32>().flatten().collect();
    let sample_length = reader.len();
    // let file_duration = reader.duration() as f64;
    // let spec = reader.spec();
    // let total_time = file_duration / spec.sample_rate as f64;

    if samples_per_pixel == 0 {
        warn!("No zoom specified, the whole file will be printed.");
        let temp_val = &(sample_length / width);
        samples_per_pixel = *temp_val;
        debug!(
            "Calculated samples per pixel(=zoom) according to the image width(='{}'px.) is {}",
            width, samples_per_pixel
        );
    }

    let (mut min, mut max) = (0, 0);

    let mut samples_overview: Vec<SampleOverview> = Vec::new();

    let mut count: u32 = 0;
    let mut rms_range: Vec<i32> = Vec::new();

    for i in 0..sample_length {
        let index: usize = i as usize;
        let sample = samples[index];
        rms_range.push(sample);
        if sample < min {
            min = sample
        }
        if sample > max {
            max = sample
        }

        count += 1;
        if count == samples_per_pixel {
            let rms = calculate_rms(&rms_range);
            samples_overview.push(SampleOverview { min, max, rms });
            count = 0;
            min = 0;
            max = 0;
            rms_range = Vec::new();
        }
    }

    Ok(samples_overview)
}

// pub fn extract_samples(
//     filename: &str,
//     mut samples_per_pixel: u32,
//     width: &u32,
// ) -> Result<WavFileSummary> {
//     let mut reader: WavReader<io::BufReader<File>> = hound::WavReader::open(filename)?;

//     let samples: Vec<i32> = reader.samples::<i32>().flatten().collect();
//     let sample_length = reader.len();
//     let file_duration = reader.duration() as f64;
//     let spec = reader.spec();
//     let total_time = file_duration / spec.sample_rate as f64;

//     if samples_per_pixel == 0 {
//         warn!("No zoom specified, the whole file will be printed.");
//         let temp_val = &(sample_length / width);
//         samples_per_pixel = *temp_val;
//         debug!(
//             "Calculated samples per pixel(=zoom) according to the image width(='{}'px.) is {}",
//             width, samples_per_pixel
//         );
//     }

//     let (mut min, mut max) = (0, 0);

//     let mut samples_overview: Vec<SampleOverview> = Vec::new();

//     let mut count: u32 = 0;
//     let mut rms_range: Vec<i32> = Vec::new();

//     for i in 0..sample_length {
//         let index: usize = i as usize;
//         let sample = samples[index];
//         rms_range.push(sample);
//         if sample < min {
//             min = sample
//         }
//         if sample > max {
//             max = sample
//         }

//         count += 1;
//         if count == samples_per_pixel {
//             let rms = calculate_rms(&rms_range);
//             samples_overview.push(SampleOverview { min, max, rms });
//             count = 0;
//             min = 0;
//             max = 0;
//             rms_range = Vec::new();
//         }
//     }

//     let image_duration = total_time / samples_overview.len() as f64 * *width as f64;

//     Ok(WavFileSummary {
//         source_file: filename.to_owned(),
//         sample_rate: spec.sample_rate,
//         bits: spec.bits_per_sample,
//         samples_per_pixel: samples_per_pixel.to_owned(),
//         time_duration: total_time,
//         processed_time_duration: image_duration,
//         samples_length: samples_overview.len(),
//         samples: samples_overview,
//     })
// }

// fn draw_waveform(
//     samples: &Vec<SampleOverview>,
//     filename: &str,
//     width: u32,
//     height: u32,
// ) -> Result<()> {
//     // let waveform_color = Rgb([63, 77, 155]);
//     let rms_colour = Rgb([121, 128, 225]);
//     let mut img: RgbImage = RgbImage::new(width, height);

//     for x in 0..width {
//         let index: usize = x as usize;

//         if index == samples.len() {
//             error!("Not enough samples!");
//             break;
//         }

//         let sample_overview = &samples[index];
//         let mut min = sample_overview.min;
//         let mut max = sample_overview.max;

//         // Convert values from [-32768, 32767] to [0, 65536].
//         if min < -32768 {
//             min = -32768;
//         }
//         min += 32768;
//         if max > 32767 {
//             max = 32767;
//         }
//         max += 32768;

//         let mut rms = sample_overview.rms;

//         if rms < -32768f32 {
//             rms = -32768f32;
//         }
//         if rms > 32767f32 {
//             rms = 32767f32;
//         }
//         rms += 32768f32;

//         // Scale to fit the bitmap
//         // let low_y = height as i32 - min * height as i32 / 65536;
//         // let high_y = height as i32 - max * height as i32 / 65536;
//         let rms_y = height as f32 - rms * height as f32 / 65536f32;
//         let low_rms_y = height as f32 - rms_y;

//         // Full waveform
//         // draw_line_segment_mut(
//         //     &mut img,
//         //     (x as f32, low_y as f32),
//         //     (x as f32, high_y as f32),
//         //     waveform_color,
//         // );
//         // Draw RMS for this sample group.
//         draw_line_segment_mut(
//             &mut img,
//             (x as f32, low_rms_y),
//             (x as f32, rms_y),
//             rms_colour,
//         );
//     }
//     img.save(&filename)?;
//     info!(aveform(
//     samples: &Vec<SampleOverview>,
//     filename: &str,
//     width: u32,
//     height: u32,
// ) -> Result<()> {
//     // let waveform_color = Rgb([63, 77, 155]);
//     let rms_colour = Rgb([121, 128, 225]);
//     let mut img: RgbImage = RgbImage::new(width, height);

//     for x in 0..width {
//         let index: usize = x as usize;

//         if index == samples.len() {
//             error!("Not enough samples!");
//             break;
//         }

//         let sample_overview = &samples[index];
//         let mut min = sample_overview.min;
//         let mut max = sample_overview.max;

//         // Convert values from [-32768, 32767] to [0, 65536].
//         if min < -32768 {
//             min = -32768;
//         }
//         min += 32768;
//         if max > 32767 {
//             max = 32767;
//         }
//         max += 32768;

//         let mut rms = sample_overview.rms;

//         if rms < -32768f32 {
//             rms = -32768f32;
//         }
//         if rms > 32767f32 {
//             rms = 32767f32;
//         }
//         rms += 32768f32;

//         // Scale to fit the bitmap
//         // let low_y = height as i32 - min * height as i32 / 65536;
//         // let high_y = height as i32 - max * height as i32 / 65536;
//         let rms_y = height as f32 - rms * height as f32 / 65536f32;
//         let low_rms_y = height as f32 - rms_y;

//         // Full waveform
//         // draw_line_segment_mut(
//         //     &mut img,
//         //     (x as f32, low_y as f32),
//         //     (x as f32, high_y as f32),
//         //     waveform_color,
//         // );
//         // Draw RMS for this sample group.
//         draw_line_segment_mut(
//             &mut img,
//             (x as f32, low_rms_y),
//             (x as f32, rms_y),
//             rms_colour,
//         );
//     }
//     img.save(&filename)?;
//     info!(
//         "The waveform image has successfully been created. '{}'",
//         filename
//     );
//     Ok(())
// }
