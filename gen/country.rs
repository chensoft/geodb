use std::sync::LazyLock;

use serde::{Serialize, Deserialize};
use anyhow::Result;
use indexmap::IndexMap;
use handlebars::Handlebars;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Country {
    /// e.g., "US"
    pub code2: String,

    /// e.g., "USA"
    pub code3: String,

    /// e.g., 840
    pub numeric: i32,

    /// e.g., "NA"
    pub continent: String,

    /// e.g., false
    pub is_eu: bool,

    /// e.g., false
    pub is_eea: bool,

    /// e.g., true
    pub is_un: bool,

    /// e.g., "en-US"
    pub language: String,

    /// e.g., "USD"
    pub currency: String,

    /// e.g., "+1"
    pub dial: String,

    /// e.g., ".us"
    pub tld: String,

    /// e.g., "🇺🇸"
    pub flag: String,

    /// e.g., "United States"
    pub name: String,

    /// e.g., "United States of America"
    pub official: String,

    /// e.g., "Washington, D.C."
    pub capital: String,
}

pub static COUNTRIES: LazyLock<IndexMap<String, Country>> = LazyLock::new(|| {
    let mut csv = csv::Reader::from_path("res/country.csv").expect("missing country file");
    let mut map = IndexMap::new();

    for val in csv.deserialize() {
        let val: Country = val.expect("parse country failed");
        map.insert(val.code2.clone(), val);
    }

    map
});

pub fn generate() -> Result<()> {
    handlebars_helper!(language: |v: String| {
        let mut ret = "&[".to_string();

        for lan in v.split(",") {
            ret += format!("crate::Language::{}, ", lan.replace("-", "_").to_ascii_uppercase()).as_str();
        }

        ret += "]";
        ret
    });

    let countries: Vec<Country> = COUNTRIES.values().cloned().collect();

    let mut template = Handlebars::new();
    template.register_helper("language", Box::new(language));
    template.register_escape_fn(handlebars::no_escape);
    template.register_template_string("default", r#"// !!!DO NOT EDIT THIS FILE!!! //

/// Country
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, EnumIter)]
pub enum Country {
{{#each countries}}
    {{this.code2}},
{{/each}}
}

impl Country {
    /// Get country from code
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
{{#each countries}}
            "{{this.code2}}" => Some(Self::{{this.code2}}),
{{/each}}

{{#each countries}}
            "{{this.code3}}" => Some(Self::{{this.code2}}),
{{/each}}

            _ => None,
        }
    }

    /// Get country from numeric
    pub fn from_numeric(numeric: i32) -> Option<Self> {
        match numeric {
{{#each countries}}
            {{this.numeric}} => Some(Self::{{this.code2}}),
{{/each}}
            _ => None,
        }
    }

    /// ISO 3166-1 alpha2 code
    pub const fn code2(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.code2}}",
{{/each}}
        }
    }

    /// ISO 3166-1 alpha3 code
    pub const fn code3(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.code3}}",
{{/each}}
        }
    }

    /// ISO 3166-1 numeric code
    pub const fn numeric(&self) -> i32 {
        match self {
{{#each countries}}
            Self::{{this.code2}} => {{this.numeric}},
{{/each}}
        }
    }

    /// Two-letter continent code
    pub const fn continent(&self) -> crate::Continent {
        match self {
{{#each countries}}
            Self::{{this.code2}} => crate::Continent::{{this.continent}},
{{/each}}
        }
    }

    /// Is it an EU member state?
    pub const fn is_eu(&self) -> bool {
        match self {
{{#each countries}}
            Self::{{this.code2}} => {{this.is_eu}},
{{/each}}
        }
    }

    /// Is it a member of European Economic Area?
    pub const fn is_eea(&self) -> bool {
        match self {
{{#each countries}}
            Self::{{this.code2}} => {{this.is_eea}},
{{/each}}
        }
    }

    /// Is it a formal member of the United Nations?
    pub const fn is_un(&self) -> bool {
        match self {
{{#each countries}}
            Self::{{this.code2}} => {{this.is_un}},
{{/each}}
        }
    }

    /// Main official language codes
    pub const fn language(&self) -> &'static [crate::Language] {
        match self {
{{#each countries}}
            Self::{{this.code2}} => {{language this.language}},
{{/each}}
        }
    }

    /// Two-letter currency code
    pub const fn currency(&self) -> crate::Currency {
        match self {
{{#each countries}}
            Self::{{this.code2}} => crate::Currency::{{this.currency}},
{{/each}}
        }
    }

    /// International calling code
    pub const fn dial(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.dial}}",
{{/each}}
        }
    }

    /// ccTLD of the country
    pub const fn tld(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.tld}}",
{{/each}}
        }
    }

    /// Country's emoji flag
    pub const fn flag(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.flag}}",
{{/each}}
        }
    }

    /// Country's abbreviated name
    pub const fn name(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.name}}",
{{/each}}
        }
    }

    /// Country's official name
    pub const fn official(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.official}}",
{{/each}}
        }
    }

    /// Capital name, may be empty as some islands have no capital
    pub const fn capital(&self) -> &'static str {
        match self {
{{#each countries}}
            Self::{{this.code2}} => "{{this.capital}}",
{{/each}}
        }
    }
}"#)?;

    std::fs::write("src/country.rs", template.render("default", &indexmap! { "countries" => countries})?)?;

    Ok(())
}