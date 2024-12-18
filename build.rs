#![allow(non_snake_case)]

use std::sync::LazyLock;
use std::collections::HashMap;

use serde::Deserialize;
use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo::rerun-if-changed=datasets/datahub");

    gen_country()?;

    Ok(())
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct DataHubCountryCodes {
    pub ISO3166_1_Alpha_2: String, // e.g., AF
    pub ISO3166_1_Alpha_3: String, // e.g., AFG
    pub Dial: i32,                 // e.g., 93
}

static DATAHUB_COUNTRY_CODES: LazyLock<Result<HashMap<String, DataHubCountryCodes>>> = LazyLock::new(|| {
    let mut csv = csv::Reader::from_path("datasets/datahub/country-codes.csv")?;
    let mut map = HashMap::new();

    for val in csv.deserialize() {
        let val: DataHubCountryCodes = val?;
        map.insert(val.ISO3166_1_Alpha_2.clone(), val);
    }

    Ok(map)
});

fn gen_country() -> Result<()> {
    // todo

    Ok(())
}