use crate::{
    html_parser,
    types::{self, UserInput},
};
use colored::Colorize;
use opener::open;
use reqwest::Client;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};
use types::{Category, Sort};

pub fn generate_url(
    query: &String,
    category: &Option<Category>,
    sort: &Option<Sort>,
    domain: &String,
    invert: bool,
    page: usize,
) -> String {
    let order = if invert { "asc" } else { "desc" };
    let url = format!("https://{domain}");
    match (&category, &sort) {
        (None, None) => format!("{url}/search/{query}/{page}/"),
        (None, Some(s)) => format!(
            "{url}/sort-search/{query}/{}/{order}/{page}/",
            s.to_string()
        ),
        (Some(c), None) => {
            format!("{url}/category-search/{query}/{}/{page}/", c.to_string(),)
        }
        (Some(c), Some(s)) => format!(
            "{url}/sort-category-search/{query}/{}/{}/{order}/{page}/",
            c.to_string(),
            s.to_string(),
        ),
    }
}

pub fn get_input(data_len: usize, current_page: usize, is_last_page: bool) -> UserInput {
    let mut buffer = String::new();
    loop {
        print!("Select what you want to download (Ex: 1 2 3, 1-3) > ");
        _ = stdout().flush();
        stdin().read_line(&mut buffer).expect("Failed to read line");

        buffer = buffer.trim().to_string();

        let user_input = if buffer.is_empty() {
            continue;
        } else if buffer == "n" {
            get_next_page(current_page, is_last_page)
        } else if buffer == "p" {
            get_previous_page(current_page)
        } else if buffer == "l" {
            get_last_page(is_last_page)
        } else if buffer == "f" {
            get_first_page(current_page)
        } else if buffer.contains('-') {
            get_input_from_dash(&mut buffer, data_len)
        } else {
            get_input_from_spaces(&mut buffer, data_len)
        };

        match user_input {
            Ok(v) => {
                return v;
            }
            Err(e) => {
                println!("{}", e.red());
                buffer.clear();
                continue;
            }
        }
    }
}

fn get_input_from_dash(buffer: &mut str, data_len: usize) -> Result<UserInput, String> {
    let range = buffer.split('-').collect::<Vec<&str>>();
    if range.len() != 2 {
        return Err(String::from("Invalid range"));
    }

    let start = match range[0].parse::<usize>() {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("Invalid start number: {e}"));
        }
    };
    let end = match range[1].parse::<usize>() {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("Invalid end number: {e}"));
        }
    };

    if start >= end {
        return Err(String::from("Invalid range"));
    }

    if start < 1 || end > data_len {
        return Err(String::from("Invalid range"));
    }

    Ok(UserInput::Range(start, end))
}

fn get_input_from_spaces(buffer: &mut str, data_len: usize) -> Result<UserInput, String> {
    let mut numbers = Vec::new();
    for n in buffer.split(' ') {
        match n.parse::<usize>() {
            Ok(v) => numbers.push(v),
            Err(e) => {
                return Err(format!("Invalid number: {e}"));
            }
        }
    }

    if numbers.is_empty() {
        return Err(String::from("Invalid number"));
    }

    for n in &numbers {
        if *n > data_len || *n < 1 {
            return Err(String::from("Invalid number"));
        }
    }
    Ok(UserInput::Space(numbers))
}

fn get_next_page(current_page: usize, is_last_page: bool) -> Result<UserInput, String> {
    if is_last_page {
        return Err(String::from("Already on the last page"));
    }
    Ok(UserInput::Next(current_page + 1))
}

fn get_previous_page(current_page: usize) -> Result<UserInput, String> {
    if current_page == 1 {
        return Err(String::from("Already on the first page"));
    }
    Ok(UserInput::Previous(current_page - 1))
}

fn get_last_page(is_last_page: bool) -> Result<UserInput, String> {
    if is_last_page {
        return Err(String::from("Already on the last page"));
    }
    Ok(UserInput::Last)
}

fn get_first_page(current_page: usize) -> Result<UserInput, String> {
    if current_page == 1 {
        return Err(String::from("Already on the first page"));
    }
    Ok(UserInput::First)
}

pub async fn get_magnet(client: Client, torrents: Vec<types::Torrent>, numbers: Vec<usize>) {
    for n in numbers {
        let req = client
            .get(&torrents[n - 1].link)
            .send()
            .await
            .expect("Failed to send request");

        if !req.status().is_success() {
            println!("{}", format!("Error: {}", req.status()).bold().red());
            exit(1);
        }
        match html_parser::extract_torrent_magnet(
            req.text().await.expect("Failed to read response"),
        ) {
            Ok(v) => {
                _ = open(v);
            }
            Err(e) => {
                println!("{}", e.bold().red());
                exit(1);
            }
        };
    }
}
