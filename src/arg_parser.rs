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

    /// Search query
    pub query: String,

    #[arg(short, long, help = format!("Sort by\n{}", Sort::list()))]
    pub sort: Option<Sort>,

    #[arg(short, long, help = format!("Filter specific categories\n{}", Category::list()))]
    pub category: Option<Category>,

    /// Invert results order
    #[arg(short, long, default_value_t = false)]
    pub invert: bool,

    /// Change 1337x domain
    #[arg(short, long, default_value_t = String::from("1337x.to"))]
    pub domain: String,

    /// Print magnet link without opening the default torrent client
    #[arg(short, long, default_value_t = false)]
    pub print_only: bool,
}
