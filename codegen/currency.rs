use std::sync::LazyLock;

use serde::{Serialize, Deserialize};
use anyhow::Result;
use indexmap::IndexMap;
use handlebars::Handlebars;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Currency {
    /// e.g., "USD"
    pub code: String,

    /// e.g., 840
    pub numeric: i32,

    /// e.g., 2
    pub minor: i32,

    /// e.g., "$"
    pub symbol: String,

    /// e.g., "Dollar"
    pub unit: String,

    /// e.g., "US Dollar"
    pub name: String,

    /// e.g., "US Dollars"
    pub plural: String,
}

pub static CURRENCIES: LazyLock<IndexMap<String, Currency>> = LazyLock::new(|| {
    let mut csv = csv::Reader::from_path("datasets/currency.csv").expect("missing currency file");
    let mut map = IndexMap::new();

    for val in csv.deserialize() {
        let val: Currency = val.expect("parse currency failed");
        map.insert(val.code.clone(), val);
    }

    map
});

pub fn generate() -> Result<()> {
    let currencies: Vec<Currency> = CURRENCIES.values().cloned().collect();

    let mut template = Handlebars::new();
    template.register_template_string("default", r#"// !!!DO NOT EDIT THIS FILE!!! //

/// Currency
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumIter)]
pub enum Currency {
{{#each currencies}}
    {{this.code}},
{{/each}}
}

impl Currency {
    /// Get currency from code
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
{{#each currencies}}
            "{{this.code}}" => Some(Self::{{this.code}}),
{{/each}}
            _ => None,
        }
    }

    /// Get currency from numeric
    pub fn from_numeric(numeric: i32) -> Option<Self> {
        match numeric {
{{#each currencies}}
            {{this.numeric}} => Some(Self::{{this.code}}),
{{/each}}
            _ => None,
        }
    }

    /// ISO 4217 currency code
    pub const fn code(&self) -> &'static str {
        match self {
{{#each currencies}}
            Self::{{this.code}} => "{{this.code}}",
{{/each}}
        }
    }

    /// The numeric code is the same as the numeric country code where possible
    pub const fn numeric(&self) -> i32 {
        match self {
{{#each currencies}}
            Self::{{this.code}} => {{this.numeric}},
{{/each}}
        }
    }

    /// For currencies having minor units, ISO 4217:2015 also shows the relationship between the minor unit and the currency itself
    pub const fn minor(&self) -> i32 {
        match self {
{{#each currencies}}
            Self::{{this.code}} => {{this.minor}},
{{/each}}
        }
    }

    /// Currency symbol
    pub const fn symbol(&self) -> &'static str {
        match self {
{{#each currencies}}
            Self::{{this.code}} => "{{this.symbol}}",
{{/each}}
        }
    }

    /// Unit refers to the standard currency measure
    pub const fn unit(&self) -> &'static str {
        match self {
{{#each currencies}}
            Self::{{this.code}} => "{{this.unit}}",
{{/each}}
        }
    }

    /// ISO 4217 currency english name
    pub const fn name(&self) -> &'static str {
        match self {
{{#each currencies}}
            Self::{{this.code}} => "{{this.name}}",
{{/each}}
        }
    }

    /// The form of the currency name used when referring to more than one unit
    pub const fn plural(&self) -> &'static str {
        match self {
{{#each currencies}}
            Self::{{this.code}} => "{{this.plural}}",
{{/each}}
        }
    }
}"#)?;

    std::fs::write("src/currency.rs", template.render("default", &indexmap! { "currencies" => currencies})?)?;

    Ok(())
}