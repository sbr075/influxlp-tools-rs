use std::{collections::HashMap, fmt::Display};

use element::{FieldKey, FieldValue, Measurement, TagKey, TagValue};

pub mod builder;
pub mod element;
pub mod error;
pub mod parser;
pub mod traits;

#[derive(Debug, Clone)]
pub struct LineProtocol {
    /// The data point measurement name
    pub measurement: Measurement,

    /// The data point tag set
    pub tags: Option<HashMap<TagKey, TagValue>>,

    /// The data point field set
    pub fields: HashMap<FieldKey, FieldValue>,

    /// To ensure a data point includes the time a metric is observed (not
    /// received by InfluxDB), include a timestamp if not defined
    ///
    /// By default the timestamp is defined in nanoseconds. If you are using any
    /// other form of precision it needs to be defined when making the insert
    /// request
    // Unfortunately there is no way of knowing the timestamp precision from just the given number
    // as the precision is defined when you query the database. But the min/max timestamp value is
    // exactly a i64 https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#unix-timestamp
    pub timestamp: Option<i64>,
}

impl PartialEq for LineProtocol {
    fn eq(&self, other: &Self) -> bool {
        if self.measurement != other.measurement {
            return false;
        }

        let tags_matches = match (&self.tags, &other.tags) {
            (Some(tags1), Some(tags2)) => tags1 == tags2,
            _ => return false,
        };

        let timestamp_matches = match (self.timestamp, other.timestamp) {
            (Some(ts1), Some(ts2)) => ts1 == ts2,
            _ => return false,
        };

        // At this point we know the measurement is equal. If the tags and timestamp are
        // also equal its a duplicate line
        tags_matches && timestamp_matches
    }
}

impl Display for LineProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lp = match &self.build() {
            Ok(lp) => lp.to_string(),
            Err(e) => format!("invalid line protocol: {e}"),
        };

        write!(f, "{}", lp)
    }
}

impl LineProtocol {
    pub fn get_tag<K>(&self, key: K) -> Option<TagValue>
    where
        K: Into<TagKey>,
    {
        match &self.tags {
            Some(tags) => tags.get(&key.into()).cloned(),
            None => None,
        }
    }

    pub fn get_field<K>(&self, key: K) -> Option<FieldValue>
    where
        K: Into<FieldKey>,
    {
        self.fields.get(&key.into()).cloned()
    }
}
