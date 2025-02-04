use crate::bookmark::Bookmark;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlistFile {
    pub children: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "WebBookmarkType")]
pub enum Entry {
    #[serde(rename_all = "PascalCase")]
    #[serde(rename(deserialize = "WebBookmarkTypeProxy"))]
    Proxy {/*title: Option<String>*/},

    #[serde(rename_all = "PascalCase")]
    #[serde(rename(deserialize = "WebBookmarkTypeList"))]
    List {
        title: Option<String>,
        children: Option<Vec<Entry>>,
    },

    #[serde(rename(deserialize = "WebBookmarkTypeLeaf"))]
    Leaf {
        #[serde(rename(deserialize = "URLString"))]
        url: String,
        #[serde(rename(deserialize = "imageUrl"))]
        image_url: Option<String>,
        #[serde(rename(deserialize = "URIDictionary"))]
        meta: Option<LeafMeta>,
        #[serde(rename(deserialize = "ReadingList"))]
        readinglist_meta: Option<ReadingListMeta>,
    },
}

#[derive(Debug, Deserialize)]
pub struct LeafMeta {
    pub title: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReadingListMeta {
    pub preview_text: Option<String>,
}

pub async fn reading_list_to_bookmarks() -> Result<Vec<Bookmark>> {
    let path = "~/Library/Safari/Bookmarks.plist";
    let path = shellexpand::tilde(path).to_string();
    let document: PlistFile = plist::from_file(path).unwrap();
    let reading_list = document
        .children
        .iter()
        .find(|entry| {
            if let Entry::List { title, .. } = entry {
                let title = title.as_ref().map(|s| s.as_str());
                if title == Some("com.apple.ReadingList") {
                    return true;
                }
            }
            false
        })
        .unwrap();

    let mut bookmarks = vec![];

    if let Entry::List { children, .. } = reading_list {
        let Some(children) = children else {
            println!("No children found in ReadingList");
            return Ok(vec![]);
        };
        for entry in children.iter() {
            if let Entry::Leaf {
                url,
                meta,
                readinglist_meta,
                image_url,
            } = entry
            {
                let title = meta.as_ref().map(|m| m.title.as_str());

                let preview_text = readinglist_meta.as_ref().unwrap().preview_text.as_deref();
                let image_url = image_url.as_ref().map(|s| s.as_str());

                let bookmark = Bookmark {
                    link: url.clone(),
                    title: title.map(|s| s.to_string()),
                    preview_text: preview_text.map(|s| s.to_string()),
                    image_url: image_url.map(|s| s.to_string()),
                    tags: vec!["SafariReadingList".to_string()],
                    private: true,
                    to_read: true,
                };
                bookmarks.push(bookmark);
            }
        }
    }

    Ok(bookmarks)
}
