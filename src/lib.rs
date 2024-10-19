//! InfluxDB V2 Line Protocol Tools is a parsing and building library for
//! InfluxDB v2's line protocol. It provides easy-to-use functionality with
//! built-in validation, support for a builder pattern and dynamic population,
//! and options for modifying existing line protocols
//!
//! # Example
//! Below is an example of building a line protocol string and parsing one
//!
//! ## Building a line protocol string
//!
//! At minimum the measurement name and a field is required to build a valid
//! line protocol string
//!
//! ```rust
//! let line_protocol = LineProtocol::new("measurement")
//!     .add_field("field", "value")
//!     .build()
//!     .unwrap();
//! ```
//!
//! You can overwrite the measurement name by calling the `measurement` method
//!
//! ```rust
//! let mut line_protocol = LineProtocol::new("measurement")
//!     .add_field("field", "value")
//!     .build()
//!     .unwrap();
//!
//! line_protocol = line_protocol.measurement("new_measurement");
//! ```
//!
//! Multiple fields can be add by calling the `add_field` method multiple times
//!
//! ```rust
//! let line_protocol = LineProtocol::new("measurement")
//!     .add_field("field1", "value")
//!     .add_field("field2", "value")
//!     .build()
//!     .unwrap();
//! ```
//!
//! Optionally tags can be added. More tags can be added as with fields
//!
//! ```rust
//! let line_protocol = LineProtocol::new("measurement")
//!     .add_tag("tag1", "value")
//!     .add_tag("tag2", "value")
//!     .add_field("field", "value")
//!     .build()
//!     .unwrap();
//! ```
//!
//! A timestamp can be added with the `with_timestamp` method. By default the
//! timestamp is defined in nanosecond precision. If you are using any other
//! precision, e.g., seconds, it needs be defined when querying influx
//!
//! ```rust
//! let line_protocol = LineProtocol::new("measurement")
//!     .add_field("field", "value")
//!     .with_timestamp(1729270461612452700i64)
//!     .build()
//!     .unwrap();
//! ```
//!
//! A field, tag, and timestamp can be deleted if needed. This is done by
//! calling the respective `delete` function
//!
//! ```rust
//! let mut line_protocol = LineProtocol::new("measurement")
//!     .add_tag("tag", "value")
//!     .add_field("field", "value");
//!
//! line_protocol.delete_tag("tag")
//! ```
//!
//! **Note:** that deleting all fields will cause the building to fail as
//! atleast **one** field is required
//!
//! ## Parsing a line protocol string
//!
//! To parse a line protocol string the `parse_line` method can be used
//!
//! ```rust
//! let line =
//!     "measurement,tag2=value,tag=value field=\"hello\",field2=\"world\" 1729270461612452700";
//! let line_protocol = LineProtocol::parse_line(line).unwrap();
//! ```
//!
//! To parse multiple lines seperated by a newline the `parse_lines` method can
//! be used instead
//!
//! ```rust
//! let lines = vec![
//!     "measurement,tag=value field=\"value\"",
//!     "measurement field=\"{\\\"test\\\": \\\"hello\\\"}\"",
//!     "measurement,tag2=value,tag=value field=\"value\",field2=\"{\\\"test\\\": \
//!      \\\"hello\\\"}\" 1729270461612452700",
//! ]
//! .join("\n");
//!
//! let result = LineProtocol::parse_lines(&lines);
//! ```
//!
//! **Note:** The parsed line can be modified and rebuilt if needed

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
            println!("name not equal");
            return false;
        }

        let tags_matches = match (&self.tags, &other.tags) {
            (Some(tags1), Some(tags2)) => tags1 == tags2,
            (None, None) => true,
            _ => return false,
        };

        let timestamp_matches = match (self.timestamp, other.timestamp) {
            (Some(ts1), Some(ts2)) => ts1 == ts2,
            (None, None) => true,
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
    /// Get a cloned version of the measurement
    pub fn get_measurement(&self) -> Measurement {
        self.measurement.clone()
    }

    /// Get a reference of the measurement
    pub fn get_measurement_ref(&self) -> &Measurement {
        &self.measurement
    }

    /// Get a mutable reference of the measurement
    pub fn get_measurement_mut(&mut self) -> &mut Measurement {
        &mut self.measurement
    }

    /// Get the tag value associated with the provided tag key
    ///
    /// # Args
    /// * `key` - A [valid](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters)
    ///   tag key
    pub fn get_tag<K>(&self, key: K) -> Option<TagValue>
    where
        K: Into<TagKey>,
    {
        match &self.tags {
            Some(tags) => tags.get(&key.into()).cloned(),
            None => None,
        }
    }

    /// Get a reference to the tag value associated with the provided tag key
    ///
    /// # Args
    /// * `key` - A [valid](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters)
    ///   tag key
    pub fn get_tag_ref<K>(&self, key: K) -> Option<&TagValue>
    where
        K: Into<TagKey>,
    {
        match &self.tags {
            Some(tags) => tags.get(&key.into()),
            None => None,
        }
    }

    /// Get a mutable reference to the tag value associated with the provided
    /// tag key
    ///
    /// # Args
    /// * `key` - A [valid](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters)
    ///   tag key
    pub fn get_tag_mut<K>(&mut self, key: K) -> Option<&mut TagValue>
    where
        K: Into<TagKey>,
    {
        match &mut self.tags {
            Some(tags) => tags.get_mut(&key.into()),
            None => None,
        }
    }

    /// Get the field value associated with the provided field key
    ///
    /// # Args
    /// * `key` - A [valid](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters)
    ///   field key
    pub fn get_field<K>(&self, key: K) -> Option<FieldValue>
    where
        K: Into<FieldKey>,
    {
        self.fields.get(&key.into()).cloned()
    }

    /// Get a reference to the field value associated with the provided field
    /// key
    ///
    /// # Args
    /// * `key` - A [valid](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters)
    ///   field key
    pub fn get_field_ref<K>(&self, key: K) -> Option<&FieldValue>
    where
        K: Into<FieldKey>,
    {
        self.fields.get(&key.into())
    }

    /// Get a mutable reference to the field value associated with the provided
    /// field key
    ///
    /// # Args
    /// * `key` - A [valid](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters)
    ///   field key
    pub fn get_field_mut<K>(&mut self, key: K) -> Option<&mut FieldValue>
    where
        K: Into<FieldKey>,
    {
        self.fields.get_mut(&key.into())
    }

    /// Get a cloned version of the timestamp
    pub fn get_timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    /// Get a reference of the timestamp
    pub fn get_timestamp_ref(&self) -> Option<&i64> {
        self.timestamp.as_ref()
    }

    /// Get a mutable reference of the timestamp
    pub fn get_timestamp_mut(&mut self) -> Option<&mut i64> {
        self.timestamp.as_mut()
    }
}
