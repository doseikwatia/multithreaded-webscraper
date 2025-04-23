use crate::utils::WebScrapperError;
use reqwest::{get, Url};
use serde::Serialize;
use serde_json::json;
use soup::prelude::*;
use std::{fs, path::Path, str::FromStr};

#[cfg(test)]
mod tests;

async fn get_webpage(url: &str) -> Result<String, WebScrapperError> {
    let response = get(url).await;

    match response {
        Ok(res) => {
            let status_code = res.status();
            let status_code_num = status_code.as_u16();

            let body = match res.text().await {
                Ok(text) => text,
                Err(err) => {
                    return Err(WebScrapperError::from_err(err));
                }
            };

            if !status_code.is_success() {
                Err(WebScrapperError::HTTP(status_code_num, body))
            } else {
                Ok(body)
            }
        }
        Err(err) => Err(WebScrapperError::from_err(err)),
    }
}

#[derive(Serialize, Debug)]
pub struct Article {
    pub date: String,
    pub title: String,
    pub content: String,
}

pub fn parse_article(html: &str) -> Option<Article> {
    let default_soup = Soup::new("<div></div>").tag("div").find().unwrap();
    let soup = Soup::new(html)
        .tag("div")
        .attr("id", "mainbody")
        .find()
        .unwrap_or(default_soup.clone());
    //title
    let title = soup
        .tag("h1")
        .attr("style", "clear: both;")
        .find()
        .unwrap_or(default_soup.clone())
        .text();
    //date
    let date = soup
        .tag("a")
        .attr("id", "date")
        .find()
        .unwrap_or(default_soup.clone())
        .text();

    //content
    let content = soup
        .tag("div")
        .attr("class", "article-content-area")
        .find()
        .unwrap_or(default_soup.clone())
        .tag("p")
        .find()
        .unwrap_or(default_soup.clone())
        .text();

    Some(Article {
        date,
        title,
        content,
    })
}

pub fn get_list_of_articles_from_page(html: &str) -> Vec<String> {
    // println!("{}",html);
    let default_soup = Soup::new("<div></div>").tag("div").find().unwrap();
    let soup = Soup::new(html);
    soup.tag("div")
        .class("left_artl_list")
        .class("more_news")
        .find()
        .unwrap_or(default_soup.clone())
        .tag("a")
        .attr_name("title")
        .attr_name("href")
        .find_all()
        .filter_map(|a| a.get("href"))
        .collect()
}

pub async fn get_list_of_article_links(link: &str) -> Result<Vec<String>, WebScrapperError> {
    let url = Url::from_str(link).unwrap();
    let base_url = format!("{}://{}", url.scheme(), url.host_str().unwrap());
    get_webpage(link).await.and_then(|html| {
        Ok(get_list_of_articles_from_page(&html)
            .iter()
            .map(|p| format!("{}{}", base_url, p))
            .collect())
    })
}

pub fn write_article_to_file(base_dir: &str, article: &Article) -> Result<(), WebScrapperError> {
    let mut filename: String = article
        .title
        .chars()
        .map(|c| match c.is_whitespace() {
            false => c.to_ascii_lowercase(),
            true => '-',
        })
        .collect();
    filename=filename.replace("/", "");
    filename.push_str(".json");
    filename.insert_str(0, format!("{}_", article.date).as_str());
    let filepath = Path::new(base_dir).join(filename);
    let content = json!(article).to_string();
    let bytes = content.as_bytes();
    if let Err(err) = fs::write(filepath, bytes) {
        return Err(WebScrapperError::from_err(err));
    }
    Ok(())
}

pub async fn get_article(a_link: &str) -> Option<Article> {
    let html = get_webpage(a_link).await.unwrap_or("".to_owned());
    parse_article(&html)
}
