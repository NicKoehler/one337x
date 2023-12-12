use clap::Parser;

use crate::types::{Category, Sort};

/// Fetch and download torrents from 1337x
#[derive(Parser, Debug)]
#[command(
    author = "NicKoehler",
    version,
    about,
    long_about = "Fetch and download torrents from 1337x"
)]
pub struct Args {
    #[arg()]
    pub query: String,

    #[arg(short, long, help = Sort::list())]
    pub sort: Option<Sort>,

    #[arg(short, long, help = Category::list())]
    pub category: Option<Category>,

    #[arg(short, long, default_value_t = false)]
    pub invert: bool,

    #[arg(short, long, default_value_t = String::from("1337x.to"))]
    pub domain: String,
}
