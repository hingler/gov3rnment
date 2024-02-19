use photon_rs::{helpers::dyn_image_from_raw, PhotonImage};
use serenity::{all::Message, async_trait, builder::{CreateAttachment, CreateMessage}, client::Context};

use crate::{args::arg_parser::ArgParser, command::base_command::BaseCommand};

use super::image_loader::ImageLoader;

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, RgbaImage};

type Gray16Image = ImageBuffer<Luma<u16>, Vec<u16>>;

pub struct Pixelsort;

const DEFAULT_THRESHOLD: &str = "0.5";

#[async_trait]
impl BaseCommand for Pixelsort {
  async fn handle_message(&self, ctx: &Context, msg: &Message, args: &ArgParser) {
    let threshold = args.args.get(1).unwrap_or(&DEFAULT_THRESHOLD).parse::<f64>().unwrap();
    if let Some(img) = ImageLoader::fetch_image_from_message(msg).await {
      let mut response = CreateMessage::new();
      let photon = Pixelsort::pixelsort_rotate(&img, threshold).expect("panic if fails lol");

      let attach = CreateAttachment::bytes(photon.get_bytes_jpeg(95u8), "output.jpg");
      response = response.add_file(attach);
      if let Err(e) = msg.channel_id.send_message(&ctx.http, response).await {
        println!("image upload failed - {}", e);
      }
    } else {
      println!("culdnt find an image :3");
    }
  }
}

impl Pixelsort {
  fn pixelsort_rotate(img: &PhotonImage, threshold: f64) -> Option<PhotonImage> {
    let dynamic_img = dyn_image_from_raw(img);
    let res = (Self::pixelsort(&dynamic_img, threshold))?;
    let (width, height) = res.dimensions();
    return Some(PhotonImage::new(res.into_bytes(), width, height));
  }

  fn pixelsort(img_dyn: &DynamicImage, threshold: f64) -> Option<DynamicImage> {
    let lumas = img_dyn.to_luma16();
    let mut img_buf = img_dyn.to_rgba8();

    let (width, height) = lumas.dimensions();

    for y in 0..height {
      Pixelsort::sort_row(&mut img_buf, &lumas, y as usize, width as usize, threshold);
    }

    let res: DynamicImage = DynamicImage::ImageRgba8(img_buf);
    return Some(res);
  }

  fn sort_row(buffer: &mut RgbaImage, luma: &Gray16Image, y: usize, width: usize, threshold: f64) {
    // while x is below width
    // maintain last pixel  bove sorting threshold
    // if pixel above sorting threshold: advance
    // if pixel below sorting threshold:
    // - sort everything from (1 + last above) to this
    // - func ignores single pixel case
    let mut last_threshold = 0;
    let mut cur: usize = 0;

    let threshold_u16 = (threshold as f64 * (u16::MAX as f64)) as u16;

    while cur < width {
      let px = luma.get_pixel(cur as u32, y as u32);
      if px[0] < threshold_u16 {
        // sort
        Pixelsort::sort_range(buffer, luma, last_threshold, cur, y, width);
        last_threshold = cur;
      }
      // else, we're in territory which will be sorted

      // lastly, increment cursor
      cur += 1;
    }

    // sort once last time, in case residual values
    Pixelsort::sort_range(buffer, luma, last_threshold, cur, y, width);
  }

  fn sort_range(buffer: &mut RgbaImage, luma: &Gray16Image, x_start: usize, x_end: usize, y: usize, width: usize ) {
  let slice_size = x_end - x_start;
  
  if slice_size <= 1 { return; }

  let luma_samples = &luma.as_flat_samples();
  let mut output_samples = buffer.as_flat_samples_mut();

  // lumas condensed, dont need to x4
  let slice = &luma_samples.as_slice()[y * width + x_start .. y * width + x_end];
  // col channels
  // need to re-ref bc we're getting a slice
  let out_slice: &mut [u8] = &mut (output_samples).as_mut_slice()[(y * width + x_start) * 4 .. (y * width + x_end) * 4];

  // copy slice so that we can reference it when rearranging unsorted elements
  let slice_read: Vec<u8> = out_slice.iter().cloned().collect();

  let mut indices: Vec<usize> = (0..slice_size).collect();

  // sort indices by their associated luma values
  indices.sort_by(|a, b| ((&slice)[*b].cmp(&slice[*a])));
  
  for i in 0..slice_size {
    // need to copy in bunches of 4
    // slice_read contains unsorted data
    // out slice fetches via index order
    out_slice[4 * i] = slice_read[4 * indices[i]];             // r
    out_slice[4 * i + 1] = slice_read[4 * indices[i] + 1];     // g
    out_slice[4 * i + 2] = slice_read[4 * indices[i] + 2];     // b
    out_slice[4 * i + 3] = slice_read[4 * indices[i] + 3];     // a
  }

  // outsli
}
}