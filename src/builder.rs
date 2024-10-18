use std::collections::HashMap;

use crate::{
    element::{FieldKey, FieldValue, Measurement, TagKey, TagValue},
    error::BuilderError,
    traits::Format,
    LineProtocol,
};

use crate::error::Result;

impl LineProtocol {
    pub fn new<T>(measurement: T) -> Self
    where
        T: Into<Measurement>,
    {
        Self {
            measurement: measurement.into(),
            tags: None,
            fields: HashMap::new(),
            timestamp: None,
        }
    }

    pub fn add_tag<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<TagKey>,
        V: Into<TagValue>,
    {
        self.tags
            .get_or_insert(HashMap::new())
            .insert(key.into(), value.into());
        self
    }

    pub fn add_tag_ref<K, V>(&mut self, key: K, value: V)
    where
        K: Into<TagKey>,
        V: Into<TagValue>,
    {
        self.tags
            .get_or_insert(HashMap::new())
            .insert(key.into(), value.into());
    }

    pub fn add_field<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<FieldKey>,
        V: Into<FieldValue>,
    {
        self.fields.insert(key.into(), value.into());
        self
    }

    pub fn add_field_ref<K, V>(&mut self, key: K, value: V)
    where
        K: Into<FieldKey>,
        V: Into<FieldValue>,
    {
        self.fields.insert(key.into(), value.into());
    }

    pub fn with_timestamp<T>(mut self, timestamp: T) -> Self
    where
        T: Into<i64>,
    {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn with_timestamp_ref<T>(&mut self, timestamp: T)
    where
        T: Into<i64>,
    {
        self.timestamp = Some(timestamp.into());
    }

    pub fn build(&self) -> Result<String> {
        if self.measurement.0.is_empty() {
            return Err(BuilderError::EmptyMeasurement.into());
        }

        if self.measurement.0.starts_with("_") {
            return Err(BuilderError::InvalidMeasurement.into());
        }

        let mut line_protocol = format!("{}", self.measurement.escape());

        if let Some(tags) = &self.tags {
            let mut formatted_tags = Vec::new();
            for (key, value) in tags {
                // Influx naming restriction
                // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#naming-restrictions
                if key.0.is_empty() {
                    return Err(BuilderError::EmptyTagKey.into());
                }

                if key.0.starts_with("_") {
                    return Err(BuilderError::InvalidTagKey.into());
                }

                if value.0.is_empty() {
                    return Err(BuilderError::EmptyTagValue.into());
                }

                formatted_tags.push(format!("{}={}", key.escape(), value.escape()));
            }

            // Influx best practices
            // https://docs.influxdata.com/influxdb/v2/write-data/best-practices/optimize-writes/#sort-tags-by-key
            formatted_tags.sort();
            line_protocol = format!("{line_protocol},{}", formatted_tags.join(","))
        }

        let mut formatted_fields = Vec::new();
        for (key, value) in &self.fields {
            // Influx naming restriction
            // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#naming-restrictions
            if key.0.is_empty() {
                return Err(BuilderError::EmptyFieldKey.into());
            }

            if key.0.starts_with("_") {
                return Err(BuilderError::InvalidFieldKey.into());
            }

            if let FieldValue::String(string) = value {
                if string.is_empty() {
                    return Err(BuilderError::EmptyFieldValue.into());
                }
            }

            formatted_fields.push(format!("{}={}", key.escape(), value.escape()));
        }

        if formatted_fields.is_empty() {
            return Err(BuilderError::MissingFields.into());
        }

        line_protocol = format!("{line_protocol} {}", formatted_fields.join(","));

        if let Some(timestamp) = self.timestamp {
            line_protocol = format!("{line_protocol} {timestamp}");
        }

        Ok(line_protocol)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_protocol_builder_ok() {
        let line_protocol = LineProtocol::new("measurement")
            .add_tag("tag", "value")
            .add_field("field", "value")
            .build();
        assert!(line_protocol.is_ok());

        let line_protocol = LineProtocol::new("measurement")
            .add_field("field", "{\"test\": \"hello\"}")
            .build();
        assert!(line_protocol.is_ok());

        let line_protocol = LineProtocol::new("measurement")
            .add_tag("tag", "value")
            .add_tag("tag2", "value")
            .add_field("field", "value")
            .add_field("field2", "{\"test\": \"hello\"}")
            .with_timestamp(1729270461612452700i64)
            .build();
        assert!(line_protocol.is_ok());
    }

    #[test]
    fn test_line_protocol_builder_missing_field() {
        let line_protocol = LineProtocol::new("measurement")
            .add_tag("tag", "value")
            .build();
        assert!(line_protocol.is_err());
    }

    #[test]
    fn test_line_protocol_builder_invalid_measurement() {
        let line_protocol = LineProtocol::new("_measurement")
            .add_tag("tag", "value")
            .add_field("field", "value")
            .build();
        assert!(line_protocol.is_err());
    }

    #[test]
    fn test_line_protocol_builder_invalid_tag_key() {
        let line_protocol = LineProtocol::new("measurement")
            .add_tag("_tag", "value")
            .add_field("field", "value")
            .build();
        assert!(line_protocol.is_err());
    }

    #[test]
    fn test_line_protocol_builder_invalid_field_key() {
        let line_protocol = LineProtocol::new("measurement")
            .add_tag("tag", "value")
            .add_field("_field", "value")
            .build();
        assert!(line_protocol.is_err());
    }
}
