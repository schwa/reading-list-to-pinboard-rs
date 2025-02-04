// src/lib.rs

#![allow(dead_code)]

use anyhow::Result;
use chrono::Local;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

pub struct Bookmarks {
    pub bookmarks: Vec<Bookmark>,
}

pub struct Bookmark {
    pub link: String,
    pub title: Option<String>,
    pub preview_text: Option<String>,
    pub image_url: Option<String>,
    pub tags: Vec<String>,
    pub private: bool,
    pub to_read: bool,
}

impl Bookmarks {
    pub fn to_netscape_html(&self, extra_tags: HashSet<&str>) -> String {
        let mut output = String::new();
        output.push_str(
            r#"<!DOCTYPE NETSCAPE-Bookmark-file-1>
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<TITLE>Bookmarks</TITLE>
<H1>Bookmarks</H1>
<DL>
"#,
        );

        for bookmark in &self.bookmarks {
            let tags = HashSet::from_iter(bookmark.tags.iter().map(|s| s.as_str()));
            let tags = tags.union(&extra_tags).collect::<Vec<_>>();

            let mut attributes: Vec<String> = vec![];
            if bookmark.private {
                attributes.push("PRIVATE=\"1\"".to_string());
            }
            if bookmark.to_read {
                attributes.push("TOREAD=\"1\"".to_string());
            }
            if !tags.is_empty() {
                let mut sorted_tags = tags.clone();
                sorted_tags.sort();
                let tags_str = format!(
                    "TAGS=\"{}\"",
                    tags.iter().map(|s| **s).collect::<Vec<&str>>().join(",")
                );
                attributes.push(tags_str);
            }

            let attributes = format!(
                "{}{}",
                if attributes.is_empty() { "" } else { " " },
                attributes.join(" ")
            );
            output.push_str(&format!(
                "<p><DT><A HREF=\"{}\"{}>{}</A>\n",
                bookmark.link,
                attributes,
                bookmark.title.as_deref().unwrap_or("")
            ));
        }
        output
    }

    pub fn to_markdown(&self) -> String {
        let mut output = String::new();

        output.push_str("## Reading List Links\n\n");

        for bookmark in &self.bookmarks {
            let tags = if !bookmark.tags.is_empty() {
                format!(" [[{}]]", bookmark.tags.join(", "))
            } else {
                String::new()
            };

            // - [title](url) [tags]

            output.push_str(&format!(
                "- [{}]({}){}\n",
                bookmark.title.as_deref().unwrap_or(bookmark.link.as_str()),
                bookmark.link,
                tags
            ));
        }
        output
    }

    pub fn to_daily_notes(&self) -> Result<()> {
        let markdown = self.to_markdown();
        let today = Local::now();
        let today_str = today.format("%Y-%m-%d").to_string();
        let path = format!("~/Notes/Daily Notes/{}.md", today_str);
        let path = shellexpand::tilde(&path).to_string();
        let path = Path::new(&path);

        let mut file = if path.exists() {
            let mut existing_content = String::new();
            let mut existing_file = OpenOptions::new().read(true).open(path)?;
            existing_file.read_to_string(&mut existing_content)?;

            if existing_content.contains("Reading List Links") {
                OpenOptions::new().append(true).open(path)?
            } else {
                OpenOptions::new().create(true).truncate(true).write(true).open(path)?
            }
        } else {
            OpenOptions::new().create(true).truncate(true).write(true).open(path)?
        };

        file.write_all(markdown.as_bytes())?;

        Ok(())
    }
}
