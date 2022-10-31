use dotenv::dotenv;
use sea_orm::prelude::*;
use std::env;

extern crate dotenv;

pub mod entities;
pub mod query_root;
pub use query_root::QueryRoot;

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}
pub struct EnvConfig {
    pub database_url: String,
    pub propublica_key: String,
    pub congress: u8,
    pub complexity_limit: Option<usize>,
    pub depth_limit: Option<usize>,
}

impl EnvConfig {
    /// Creates a new [`EnvConfig`].
    ///
    /// # Panics
    ///
    /// Panics if an expected environment variable is missing.
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            propublica_key: env::var("PROPUBLICA_KEY").expect("PROPUBLICA_KEY must be set"),
            congress: env::var("CONGRESS")
                .expect("CONGRESS must be set")
                .parse::<u8>()
                .expect("CONGRESS_NO Env Variable u8 conversion error!"),
            complexity_limit: env::var("COMPLEXITY_LIMIT")
                .map(|val| {
                    val.parse::<usize>()
                        .expect("COMPLEXITY_LIMIT is not a number")
                })
                .map_or(None, |val| Some(val)),
            depth_limit: env::var("DEPTH_LIMIT")
                .map(|val| val.parse::<usize>().expect("DEPTH_LIMIT is not a number"))
                .map_or(None, |val| Some(val)),
        }
    }
}

pub mod fec_data {
    use std::{
        cmp,
        fs::{self},
        path::{Path, PathBuf},
    };

    use indicatif::{ProgressBar, ProgressStyle};

    pub fn new_file_reading_progress_spinner(path: PathBuf) -> ProgressBar {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::with_template(
                "{prefix} {spinner:.cyan/blue} {human_pos:>7} [{elapsed}]",
            )
            .unwrap(),
        );
        bar.set_prefix(format!(
            "Parsing {}",
            path.file_name().unwrap().to_str().unwrap()
        ));

        return bar;
    }

    /// * Some data is overwritten in later years (e.g. committees change leadership or name). By sorting the paths in order from oldest to newest, we can overwrite the older data as we go. This ensures we always have the updated info.
    pub fn get_sorted_path_bufs(folder: &str) -> Vec<PathBuf> {
        let root_path = Path::new("./src/bin/").join(folder);
        let paths = fs::read_dir(root_path).unwrap();

        let mut sorted_paths: Vec<PathBuf> = paths
            .map(|path| path.expect("Invalid Path").path())
            .collect();

        sorted_paths.sort_by(|a, b| a.file_name().cmp(&b.file_name().to_owned()));

        sorted_paths
    }

    /// * Turns a `String` reference to an `Option` of a String, making it `None` if the passed string is empty, the `Some(value)` of the passed string otherwise.
    pub fn none_if_empty(x: &str) -> Option<String> {
        if x.is_empty() {
            None
        } else {
            Some(x.to_string())
        }
    }

    /// * Postgres and some DBs do not like it when you try to add_many for over 10k items. This splits a Vec of Type T into N Vecs of Type T, making each vec no bigger than page_size.
    pub fn page_data<T: Clone>(v: Vec<T>, page_size: usize) -> Vec<Vec<T>> {
        let mut paged_v: Vec<Vec<T>> = Vec::new();

        let mut right = 0;

        loop {
            let left = right;
            right = cmp::min(right + page_size, v.len());

            paged_v.insert(paged_v.len(), v[left..right].to_vec());

            if v.len() < 1 || right >= v.len() - 1 {
                break;
            }
        }

        paged_v
    }

    /// Parses an 18 or 11 length filing date into an election year.
    /// The Election Cycle is always determined by the occurence of a general election and is a two year period.
    /// For example, because 2018 is the year of a general election, 2017-2018 is one election cycle, denoted by just "2018".
    /// Cycle 2020 covers 2019-2020.
    /// # Panics
    ///
    /// Panics if a date format that is not 11 or 18 length.
    pub fn parse_filing_date(x: &str) -> String {
        return match x.len() {
            18 => {
                let year = x[0..4].parse::<i32>().unwrap();
                (year + year % 2).to_string()
            }
            11 => {
                let year = format!("20{}", &x[0..2])
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                (year + year % 2).to_string()
            }
            _ => panic!("INVALID DATE IN BULK DATA: {}", x),
        };
    }
}
