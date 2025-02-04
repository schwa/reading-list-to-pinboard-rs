#![allow(dead_code)]

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

use crate::bookmark::*;
use std::env;

impl Bookmarks {
    pub async fn post_all_to_raindrop(&self, extra_tags: Vec<String>) -> Result<()> {
        let access_token =
            env::var("RAINDROP_ACCESS_TOKEN").expect("RAINDROP_ACCESS_TOKEN must be set");

        let client = reqwest::Client::new();

        let extra_tags: HashSet<String> = extra_tags.into_iter().collect();
        let raindrops = self
            .bookmarks
            .iter()
            .map(|bookmark| {
                let mut raindrop = Raindrop::new(bookmark);
                raindrop
                    .tags
                    .extend(extra_tags.iter().map(|s| s.to_string()));
                raindrop
            })
            .collect::<Vec<_>>();

        let url = "https://api.raindrop.io/rest/v1/raindrops";

        let items: HashMap<&str, Vec<Raindrop>> = [("items", raindrops)].iter().cloned().collect();

        let response = client
            .post(url)
            .bearer_auth(access_token)
            .json(&items)
            .send()
            .await;

        println!("{:?}", response);

        let response_json = response.unwrap().json::<serde_json::Value>().await.unwrap();

        std::fs::write(
            "response.json",
            serde_json::to_string_pretty(&response_json)
                .unwrap()
                .as_bytes(),
        )?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Raindrop {
    pub title: String,
    pub link: String,
    pub tags: Vec<String>,
}

impl Raindrop {
    pub fn new(bookmark: &Bookmark) -> Self {
        Raindrop {
            title: bookmark.title.clone().unwrap_or_default(),
            link: bookmark.link.clone(),
            tags: bookmark.tags.clone(),
        }
    }
}
