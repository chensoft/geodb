use std::sync::LazyLock;

use serde::{Serialize, Deserialize};
use indexmap::{IndexMap};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CountryCode {
    /// e.g., US
    pub alpha2: String,

    /// e.g., USA
    pub alpha3: String,

    /// e.g., 840
    pub numeric: i32,
}

pub static COUNTRY_CODE: LazyLock<IndexMap<String, CountryCode>> = LazyLock::new(|| {
    let mut csv = csv::Reader::from_path("datasets/country-code.csv").expect("missing country code");
    let mut map = IndexMap::new();

    for val in csv.deserialize() {
        let val: CountryCode = val.expect("parse country code failed");
        map.insert(val.alpha2.clone(), val);
    }

    map
});