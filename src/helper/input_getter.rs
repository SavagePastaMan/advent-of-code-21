use num;

use reqwest;
use reqwest::header;
use reqwest::header::COOKIE;

use serde;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

pub fn parse_csv<T>(raw: &str) -> Vec<T>
where
    T: num::PrimInt + Copy + std::str::FromStr,
{
    raw.split(",")
        .map(|x| num::NumCast::from(x.parse::<i128>().unwrap()).unwrap())
        .collect()
}

type InputData = HashMap<String, String>;

static INPUT_CACHE: &'static str = "inputs.json";

pub fn get_input(day: u32) -> Result<String, Box<dyn Error>> {
    let mut data: InputData = read_cache(INPUT_CACHE)?;

    let mut fetched = fetch(day)?;

    fetched = String::from(fetched.strip_suffix("\n").unwrap());

    let entry = data.entry(day.to_string()).or_insert(fetched);
    let ret = entry.clone();

    write_cache(INPUT_CACHE, &data)?;

    Ok(ret)
}

fn fetch(day: u32) -> Result<String, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    let client = reqwest::blocking::Client::new();
    headers.insert(
        COOKIE,
        header::HeaderValue::from_static("session=53616c7465645f5fe9aaba0876d8c78a543b7aa33b2ea5cd97dd25f998b9ccac27927da2b3933eb632e892ca17dcdcda"));
    let res = client
        .get(format!("https://adventofcode.com/2021/day/{}/input", day))
        .headers(headers)
        .send()?;

    res.text()
}

fn read_cache<P, R>(path: P) -> Result<R, Box<dyn Error>>
where
    P: AsRef<Path>,
    R: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

fn write_cache<P, D>(path: P, data: D) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    D: serde::ser::Serialize,
{
    let s = serde_json::to_string_pretty(&data)?;
    let mut file = OpenOptions::new().write(true).open(path)?;

    file.write_all(s.as_bytes())?;

    Ok(())
}
