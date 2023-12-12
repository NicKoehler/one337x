mod arg_parser;
mod html_parser;
mod types;
mod utils;

use std::process::exit;

use arg_parser::Args;
use clap::Parser;
use colored::Colorize;
use reqwest::Client;
use types::{Page, UserInput};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut current_page = 1;

    loop {
        let url = utils::generate_url(
            &args.query,
            &args.category,
            &args.sort,
            &args.domain,
            args.invert,
            current_page,
        );
        let client = Client::new();

        let res = client.get(url).send().await;
        let Ok(res) = res else {
            return;
        };

        if !res.status().is_success() {
            println!("{}", format!("Error: {}", res.status()).bold().red());
            exit(1);
        }

        let (torrents, pages) = match html_parser::extract_torrent_data(
            res.text().await.expect("Failed to read response"),
            &args.domain,
        ) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        };

        if torrents.is_empty() {
            println!(
                "{}",
                format!("No torrents found for {}", args.query.underline())
                    .bold()
                    .red()
            );
            exit(1);
        }

        torrents.iter().rev().for_each(|torrent| {
            println!(
                "{:>2} · {}\n     Seeders {} · Leechers {} · Time {} · Size {} · Uploader {}\n",
                torrent.number.purple(),
                torrent.title.bold(),
                torrent.seeders.bold().green(),
                torrent.leechers.bold().yellow(),
                torrent.time.bold().blue(),
                torrent.size.bold().red(),
                torrent.uploader.bold().cyan()
            )
        });

        let mut is_last_page = true;
        if !pages.is_empty() {
            println!(
                "Pages · {}",
                pages
                    .iter()
                    .map(|page| {
                        match page {
                            Page::Number(page) => {
                                if *page == current_page {
                                    format!("{}", page).bold().green().to_string()
                                } else {
                                    format!("{}", page).to_string()
                                }
                            }
                            Page::Previous | Page::Next => format!("{}", page).bold().to_string(),
                            page => format!("{}", page).blue().to_string(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" · ")
            );
            is_last_page = !matches!(pages.last().expect("Pages is empty"), Page::Last(_));
        }

        let input = utils::get_input(torrents.len(), current_page, is_last_page);

        match input {
            UserInput::Range(start, end) => {
                break utils::get_magnet(client, torrents, (start..=end).collect()).await;
            }
            UserInput::Space(numbers) => {
                break utils::get_magnet(client, torrents, numbers).await;
            }
            UserInput::Next(n) => {
                current_page = n;
                continue;
            }
            UserInput::Previous(n) => {
                current_page = n;
                continue;
            }
            UserInput::Last => match pages.last().unwrap() {
                Page::Last(s) => current_page = *s,
                _ => unreachable!("Last page is not last"),
            },
            UserInput::First => current_page = 1,
        }
    }
}
