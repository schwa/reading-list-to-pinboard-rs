#![allow(dead_code)]

use crate::bookmark::*;
use anyhow::{Ok, Result};
use pin::pinboard;
use reqwest::Url;
use std::env;
use std::str::FromStr;

impl Bookmarks {
    pub async fn post_all_to_pinboard(&self) -> Result<()> {
        for bookmark in &self.bookmarks {
            bookmark.post_to_pinboard_().await?;
        }
        Ok(())
    }
}

impl Bookmark {
    pub async fn post_to_pinboard_(&self) -> Result<()> {
        let access_token = env::var("PINBOARD_TOKEN").expect("PINBOARD_TOKEN must be set");

        let client = pinboard::Client::new("https://api.pinboard.in", access_token.as_str())?;
        let url = Url::parse(&self.link)?;
        let title = pinboard::Title::new(self.title.as_deref().unwrap_or(""))?;
        let preview_text = self.preview_text.clone().unwrap_or_default();
        let image_url = self.image_url.clone().unwrap_or_default();

        println!("{} {} {} {}", url, title, preview_text, image_url);

        let tags = ["SafariReadingList"];

        let post = pinboard::Post::new(
            url,
            title,
            tags.iter().map(|s| pinboard::Tag::from_str(s).unwrap()),
            true,
        );
        println!("Sending post...");
        client.send_post(&post).await.unwrap();
        println!("Done");
        Ok(())
    }
}
