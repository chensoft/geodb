//! Currency

/// Currency
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Currency {
    // /// Required - ISO 4217 currency code (e.g., "USD", "JPY")
    // pub code: String,
    //
    // /// Optional - Full name of the currency in English (e.g., "United States Dollar", "Japanese Yen")
    // pub name: String,
    //
    // /// Optional - International currency symbol (e.g., "$", "¥")
    // pub symbol: String,
}

// impl CurrencyData {
//     pub fn from(code: &str) -> Self {
//         let mut item = Self { code: code.to_string(), ..Default::default() };
//
//         if let Some(data) = iso_currency::Currency::from_code(code) {
//             for word in data.name().split_ascii_whitespace() {
//                 if !word.is_empty() {
//                     item.name += &word[..1].to_ascii_uppercase();
//                     item.name += &word[1..];
//                     item.name += " ";
//                 }
//             }
//
//             item.name.pop();
//             item.symbol = data.symbol().to_string();
//         }
//
//         item
//     }
// }