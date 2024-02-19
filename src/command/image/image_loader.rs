use photon_rs::{native::{open_image, open_image_from_bytes, Error}, PhotonImage};
use reqwest::Client;
use serenity::all::{Attachment, Message};

pub struct ImageLoader;

impl ImageLoader {
  pub async fn fetch_image_from_message(msg: &Message) -> Option<PhotonImage> {
    for attach in &msg.attachments {
      if let Some(c) = &attach.content_type {
        if c.contains("image") {
          return ImageLoader::get_image(attach).await;
        }
      }
    }

    return None;
  }

  async fn get_image(attach: &Attachment) -> Option<PhotonImage> {
    let bytes = attach.download().await.ok()?;
    return open_image_from_bytes(&bytes.as_slice()).ok();
  }
}