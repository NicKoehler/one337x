use ego_tree::NodeRef;
use scraper::{Html, Node, Selector};

use crate::types::{Page, Torrent};

pub fn extract_torrent_data(
    html: String,
    domain: &String,
) -> Result<(Vec<Torrent>, Vec<Page>), String> {
    let document = Html::parse_document(&html);
    let vec = extract_body(&document, domain)?;
    let pages = extract_pages(&document)?;
    Ok((vec, pages))
}

pub fn extract_torrent_magnet(html: String) -> Result<String, String> {
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a[href*='magnet:?']").expect("failed to parse selector");
    let result = document
        .select(&selector)
        .next()
        .and_then(|v| v.attr("href"))
        .map(|v| v.to_string());

    match result {
        Some(v) => Ok(v),
        None => Err(String::from("Failed to extract magnet link")),
    }
}

fn extract_body(document: &Html, domain: &String) -> Result<Vec<Torrent>, String> {
    let mut results = vec![];
    let selector: Selector = Selector::parse("tbody > tr").expect("failed to parse selector");

    for (n, mut children) in document
        .select(&selector)
        .map(|v| v.children().filter(|v| v.value().is_element()))
        .enumerate()
    {
        let (title, link) = extract_title_and_href(children.next())?;
        let seeders = extract_generic(children.next())?;
        let leechers = extract_generic(children.next())?;
        let time = extract_generic(children.next())?;
        let size = extract_generic(children.next())?;
        let uploader = extract_uploader(children.next())?;

        results.push(Torrent {
            number: format!("{}", n + 1),
            title,
            seeders,
            leechers,
            time,
            size,
            uploader,
            link: format!("https://{}{}", domain, link),
        })
    }
    Ok(results)
}

fn extract_pages(document: &Html) -> Result<Vec<Page>, String> {
    let pagination: Selector =
        Selector::parse("div.pagination > ul > li > a").expect("failed to parse selector");

    let mut pages = vec![];

    for page in document.select(&pagination) {
        let Some(v) = page.children().next() else {
            return Err(String::from("Failed to get <a> tag"));
        };

        let Some(v) = v.value().as_text().map(|v| v.to_string()) else {
            return Err(String::from("Failed to convert Node into Text"));
        };

        let Some(link) = page.value().attr("href") else {
            return Err(String::from("Failed to get href text"));
        };

        match v.as_str() {
            "First" => pages.push(Page::First),
            "Last" => pages.push(Page::Last(
                link.split('/')
                    .filter(|v| !v.is_empty())
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
            )),
            "<<" => pages.push(Page::Previous),
            ">>" => pages.push(Page::Next),
            e => {
                if let Ok(v) = e.parse::<usize>() {
                    pages.push(Page::Number(v));
                } else {
                    return Err(String::from("Failed to parse page number"));
                }
            }
        }
    }
    Ok(pages)
}

fn extract_title_and_href(child: Option<NodeRef<'_, Node>>) -> Result<(String, String), String> {
    let Some(v) = child else {
        return Err(String::from("Failed to extract title"));
    };
    let Some(v) = v.children().nth(1) else {
        return Err(String::from("Failed to get second <a> tag"));
    };
    let Some(href) = v.value().as_element() else {
        return Err(String::from("Failed to get href element"));
    };
    let Some(href) = href.attr("href") else {
        return Err(String::from("Failed to get href text"));
    };
    let Some(v) = v.children().next() else {
        return Err(String::from("Failed to get text in <a> tag"));
    };
    let Some(v) = v.value().as_text() else {
        return Err(String::from("Failed to convert Node into Text"));
    };
    Ok((v.to_string(), href.to_string()))
}

fn extract_uploader(child: Option<NodeRef<'_, Node>>) -> Result<String, String> {
    let Some(v) = child else {
        return Err(String::from("Failed to extract uploader"));
    };
    let Some(v) = v.children().next() else {
        return Err(String::from("Failed to get <a> tag"));
    };
    let Some(v) = v.children().next() else {
        return Err(String::from("Failed to get text in <a> tag"));
    };
    let Some(v) = v.value().as_text() else {
        return Err(String::from("Failed to convert Node into Text"));
    };
    Ok(v.to_string())
}

fn extract_generic(child: Option<NodeRef<'_, Node>>) -> Result<String, String> {
    let Some(v) = child else {
        return Err(String::from("Failed to extract uploader"));
    };
    let Some(v) = v.children().next() else {
        return Err(String::from("Failed to get <a> tag"));
    };
    let Some(v) = v.value().as_text() else {
        return Err(String::from("Failed to convert Node into Text"));
    };
    Ok(v.to_string())
}
