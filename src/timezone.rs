//! Timezone

/// Timezone
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Timezone {
    /// Required - IANA timezone name (e.g., "America/New_York")
    pub zone: String,

    /// Optional - Timezone abbreviation (e.g., "EST")
    pub abbr: String,

    /// Optional - Human-readable offset name (e.g., "-05:00")
    pub offset: String,

    /// Optional - Numeric UTC offset in seconds (e.g., -18000)
    pub delta: i32,

    /// Optional - Local datetime (e.g., "2024-12-12T07:10:49-05:00")
    pub local: String,

    /// Optional - UTC datetime (e.g., "2024-12-12T12:10:49Z")
    pub utc: String,

    /// Optional - Unix timestamp, seconds since 1970-01-01 00:00:00 UTC (e.g., 1734005449)
    pub timestamp: i64,

    /// Optional - Indicates if Daylight Saving Time is in effect (e.g., false)
    pub is_dst: bool,
}

impl Timezone {
    pub fn from(zone: &str) -> Self {
        use std::str::FromStr;
        use chrono::Offset;
        use chrono_tz::OffsetComponents;

        let mut item = Self { zone: zone.to_string(), ..Default::default() };

        if let Ok(tz) = chrono_tz::Tz::from_str(zone) {
            let utc = chrono::Utc::now();
            let now = utc.with_timezone(&tz);
            let off = now.offset();

            item.abbr = now.format("%Z").to_string();
            item.offset = off.fix().to_string();
            item.delta = off.fix().local_minus_utc();
            item.local = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            item.utc = utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            item.timestamp = utc.timestamp();
            item.is_dst = !off.dst_offset().is_zero();
        }

        item
    }
}