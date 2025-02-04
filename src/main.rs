use anyhow::{Ok, Result};
use bookmark::*;
mod bookmark;
mod pinboard;
mod raindrop;
mod safari_readinglist;
use chrono::Local;
use dialoguer::Confirm;

use safari_readinglist::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let bookmarks = reading_list_to_bookmarks().await?;

    let bookmarks = Bookmarks { bookmarks };
    //    bookmarks.post_all_to_pinboard().await.unwrap();

    if Confirm::new()
        .with_prompt("Do you want to write all bookmarks to an Obsidian daily note?")
        .interact()?
    {
        bookmarks.to_daily_notes().unwrap();
    }

    if Confirm::new()
        .with_prompt("Do you want to post all bookmarks to Raindrop?")
        .interact()?
    {
        let today = Local::now();
        let today_str = today.format("%Y-%m-%d").to_string();
        bookmarks
            .post_all_to_raindrop(vec![
                "SafariReadingList".to_string(),
                today_str,
            ])
            .await
            .unwrap();
    }

    Ok(())
}
