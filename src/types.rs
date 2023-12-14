use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, EnumIter)]
pub enum Sort {
    Time,
    Size,
    Seeders,
    Leechers,
}

impl Sort {
    pub fn list() -> String {
        Sort::iter()
            .map(|s| {
                let binding = s.to_string();
                let mut chars = binding.chars();
                match chars.next() {
                    None => String::new(),
                    Some(v) => v.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<String>>()
            .join(" | ")
    }
}

impl FromStr for Sort {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "time" => Ok(Self::Time),
            "size" => Ok(Self::Size),
            "seeders" => Ok(Self::Seeders),
            "leechers" => Ok(Self::Leechers),
            _ => Err("Invalid sort"),
        }
    }
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Self::Time => "time".to_string(),
            Self::Size => "size".to_string(),
            Self::Seeders => "seeders".to_string(),
            Self::Leechers => "leechers".to_string(),
        }
    }
}

#[derive(Debug, Clone, EnumIter)]
pub enum Category {
    Movies,
    Tv,
    Games,
    Music,
    Apps,
    Documentaries,
    Anime,
    Other,
    Xxx,
}

impl Category {
    pub fn list() -> String {
        Category::iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" | ")
    }
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "movies" => Ok(Self::Movies),
            "tv" => Ok(Self::Tv),
            "games" => Ok(Self::Games),
            "music" => Ok(Self::Music),
            "apps" => Ok(Self::Apps),
            "documentaries" => Ok(Self::Documentaries),
            "anime" => Ok(Self::Anime),
            "other" => Ok(Self::Other),
            "xxx" => Ok(Self::Xxx),
            _ => Err("Invalid category"),
        }
    }
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Self::Movies => "Movies".to_string(),
            Self::Tv => "TV".to_string(),
            Self::Games => "Games".to_string(),
            Self::Music => "Music".to_string(),
            Self::Apps => "Apps".to_string(),
            Self::Documentaries => "Documentaries".to_string(),
            Self::Anime => "Anime".to_string(),
            Self::Other => "Other".to_string(),
            Self::Xxx => "XXX".to_string(),
        }
    }
}

pub struct Torrent {
    pub number: String,
    pub title: String,
    pub seeders: String,
    pub leechers: String,
    pub time: String,
    pub size: String,
    pub uploader: String,
    pub link: String,
}

#[derive(Debug)]
pub enum UserInput {
    Space(Vec<usize>),
    Range(usize, usize),
    Next(usize),
    Previous(usize),
    Last,
    First,
}

#[derive(Debug)]
pub enum Page {
    Next,
    Previous,
    First,
    Last(usize),
    Number(usize),
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First => write!(f, "First (1)"),
            Self::Next => write!(f, "->"),
            Self::Previous => write!(f, "<-"),
            Self::Last(v) => write!(f, "Last ({v})"),
            Self::Number(v) => write!(f, "{v}"),
        }
    }
}
