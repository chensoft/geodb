GeoDB
==========================

Comprehensive geographical information

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][license-badge]][license-url]
[![Documentation][document-badge]][document-url]
[![Build Status][linux-badge]][linux-url]
[![Build Status][macos-badge]][macos-url]
[![Build Status][windows-badge]][windows-url]

[crates-badge]: https://img.shields.io/crates/v/geodb.svg
[crates-url]: https://crates.io/crates/geodb
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/chensoft/geodb/blob/master/LICENSE
[document-badge]: https://docs.rs/geodb/badge.svg
[document-url]: https://docs.rs/geodb
[linux-badge]: https://github.com/chensoft/geodb/actions/workflows/linux.yml/badge.svg
[linux-url]: https://github.com/chensoft/geodb/actions/workflows/linux.yml
[macos-badge]: https://github.com/chensoft/geodb/actions/workflows/macos.yml/badge.svg
[macos-url]: https://github.com/chensoft/geodb/actions/workflows/macos.yml
[windows-badge]: https://github.com/chensoft/geodb/actions/workflows/windows.yml/badge.svg
[windows-url]: https://github.com/chensoft/geodb/actions/workflows/windows.yml

## Features

- ISO 3166-1 alpha2 & alpha3 country codes
- ISO 3166-2 subdivision codes
- ISO 4217 currency codes
- BCP 47 language tags
- Human-validated datasets
- Comprehensive information

## Example

```rust
use geodb::*;

fn main() {
    assert_eq!(Country::from_code("US"), Some(Country::US));
    assert_eq!(Country::from_code("US"), Country::from_code("USA"));
    assert_eq!(Country::US.continent(), Continent::NA);
    assert_eq!(Country::US.language(), &[Language::EN_US]);
    assert_eq!(Country::US.currency(), Currency::USD);
    assert_eq!(Country::US.dial(), "+1");
    assert_eq!(Country::US.tld(), ".us");
    assert_eq!(Country::US.name(), "United States");
    assert_eq!(Country::US.capital(), "Washington, D.C.");

    assert_eq!(Country::from_code("CN"), Some(Country::CN));
    assert_eq!(Country::from_code("CN"), Country::from_code("CHN"));
    assert_eq!(Country::CN.continent(), Continent::AS);
    assert_eq!(Country::CN.language(), &[Language::ZH_CN]);
    assert_eq!(Country::CN.currency(), Currency::CNY);
    assert_eq!(Country::CN.dial(), "+86");
    assert_eq!(Country::CN.tld(), ".cn");
    assert_eq!(Country::CN.name(), "China");
    assert_eq!(Country::CN.capital(), "Beijing");

    assert_eq!(Subdivision::from_code("JP-13"), Some(Subdivision::JP_13));
    assert_eq!(Subdivision::JP_13.country(), Country::JP);
    assert_eq!(Subdivision::JP_13.category(), "Prefecture");
    assert_eq!(Subdivision::JP_13.name(), "Tokyo");
    assert_eq!(Subdivision::JP_13.native(), "東京");

    assert_eq!(Currency::from_code("GBP"), Some(Currency::GBP));
    assert_eq!(Currency::GBP.symbol(), "£");
    assert_eq!(Currency::GBP.name(), "British Pound");
    assert_eq!(Currency::GBP.plural(), "British Pounds");

    assert_eq!(Language::from_code("zh-HK"), Some(Language::ZH_HK));
    assert_eq!(Language::ZH_HK.name(), "Traditional Chinese (Hong Kong)");
    assert_eq!(Language::ZH_HK.native(), "繁體中文 (香港)");
}
```

## Documentation

The documentation is [available here](https://docs.rs/geodb).

## Reference

* [ISO 3166-1](https://www.iso.org/obp/ui/#search)
* [ISO 4217](https://www.six-group.com/en/products-services/financial-information/data-standards.html)
* [CLDR](https://cldr.unicode.org/index/downloads)
* [Wiki](https://en.wikipedia.org/wiki/Main_Page)
* [XE](https://www.xe.com/currencyconverter/convert/?Amount=100&From=HKD&To=USD)
* [Wise](https://wise.com/gb/currency-converter/currencies/usd-us-dollar)

## License

This software is released under the [MIT License](https://github.com/chensoft/geodb?tab=MIT-1-ov-file).