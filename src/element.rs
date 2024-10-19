//! Elements are whats makes up the individual parts of a line protocol string
//!
//! # Line Protocol
//!
//! ```text
//! measurement         tag set             field set              timestamp
//! ----------- ------------------- ------------------------- -------------------
//! measurement,tag1=val1,tag2=val2 field1="val1",field2=true 1729270461612452700
//! ```
//!
//! ## Explanation
//! - measurement: The measurement name
//! - tag set: Optional key value pairs used to filter data points
//! - field set: Required key value pairs containing the data point data
//! - timestamp: Optional unix timestamp

use std::fmt::Display;

use anyhow::Context;
use regex::Regex;

use crate::traits::{Convert, Format};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Measurement(pub String);

impl From<&str> for Measurement {
    fn from(value: &str) -> Self {
        Measurement(value.to_string())
    }
}

impl From<&String> for Measurement {
    fn from(value: &String) -> Self {
        Measurement(value.to_owned())
    }
}

impl From<String> for Measurement {
    fn from(value: String) -> Self {
        Measurement(value)
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Format for Measurement {
    fn escape(&self) -> Self {
        Measurement(self.0.replace(" ", r"\ ").replace(",", r"\,"))
    }

    fn unescape(&self) -> Self {
        Measurement(self.0.replace(r"\,", ",").replace(r"\ ", " "))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TagKey(pub String);

impl From<&str> for TagKey {
    fn from(value: &str) -> Self {
        TagKey(value.to_string())
    }
}

impl From<&String> for TagKey {
    fn from(value: &String) -> Self {
        TagKey(value.to_owned())
    }
}

impl From<String> for TagKey {
    fn from(value: String) -> Self {
        TagKey(value)
    }
}

impl Display for TagKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Convert for TagKey {
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(TagKey(s.to_string()))
    }
}

impl Format for TagKey {
    fn escape(&self) -> Self {
        TagKey(
            self.0
                .replace(" ", r"\ ")
                .replace(",", r"\,")
                .replace("=", r"\="),
        )
    }

    fn unescape(&self) -> Self {
        TagKey(
            self.0
                .replace(r"\=", "=")
                .replace(r"\,", ",")
                .replace(r"\ ", " "),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagValue(pub String);

impl From<&str> for TagValue {
    fn from(value: &str) -> Self {
        TagValue(value.to_string())
    }
}

impl From<&String> for TagValue {
    fn from(value: &String) -> Self {
        TagValue(value.to_owned())
    }
}

impl From<String> for TagValue {
    fn from(value: String) -> Self {
        TagValue(value)
    }
}

impl Display for TagValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Convert for TagValue {
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(TagValue(s.to_string()))
    }
}

impl Format for TagValue {
    fn escape(&self) -> Self {
        TagValue(
            self.0
                .replace(" ", r"\ ")
                .replace(",", r"\,")
                .replace("=", r"\="),
        )
    }

    fn unescape(&self) -> Self {
        TagValue(
            self.0
                .replace(r"\=", "=")
                .replace(r"\,", ",")
                .replace(r"\ ", " "),
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldKey(pub String);

impl From<&str> for FieldKey {
    fn from(value: &str) -> Self {
        FieldKey(value.to_string())
    }
}

impl From<&String> for FieldKey {
    fn from(value: &String) -> Self {
        FieldKey(value.to_owned())
    }
}

impl From<String> for FieldKey {
    fn from(value: String) -> Self {
        FieldKey(value)
    }
}

impl Display for FieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Convert for FieldKey {
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(FieldKey(s.to_string()))
    }
}

impl Format for FieldKey {
    fn escape(&self) -> Self {
        FieldKey(
            self.0
                .replace(" ", r"\ ")
                .replace(",", r"\,")
                .replace("=", r"\="),
        )
    }

    fn unescape(&self) -> Self {
        FieldKey(
            self.0
                .replace(r"\=", "=")
                .replace(r"\,", ",")
                .replace(r"\ ", " "),
        )
    }
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    /// Represent a floating point number field value
    Float(f64),

    /// Represent a signed integer number field value
    Integer(i64),

    /// Represent an unsigned integer number field value
    UInteger(u64),

    /// Represent a string field value
    String(String),

    /// Represent a boolean field value
    Boolean(bool),
}

impl From<&str> for FieldValue {
    fn from(value: &str) -> Self {
        FieldValue::String(value.to_owned())
    }
}

impl From<&String> for FieldValue {
    fn from(value: &String) -> Self {
        FieldValue::String(value.to_owned())
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        FieldValue::String(value)
    }
}

impl From<f32> for FieldValue {
    fn from(value: f32) -> Self {
        FieldValue::Float(value.into())
    }
}

impl From<f64> for FieldValue {
    fn from(value: f64) -> Self {
        FieldValue::Float(value)
    }
}

impl From<i8> for FieldValue {
    fn from(value: i8) -> Self {
        FieldValue::Integer(value.into())
    }
}

impl From<i16> for FieldValue {
    fn from(value: i16) -> Self {
        FieldValue::Integer(value.into())
    }
}

impl From<i32> for FieldValue {
    fn from(value: i32) -> Self {
        FieldValue::Integer(value.into())
    }
}

impl From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        FieldValue::Integer(value)
    }
}

impl From<u8> for FieldValue {
    fn from(value: u8) -> Self {
        FieldValue::UInteger(value.into())
    }
}

impl From<u16> for FieldValue {
    fn from(value: u16) -> Self {
        FieldValue::UInteger(value.into())
    }
}

impl From<u32> for FieldValue {
    fn from(value: u32) -> Self {
        FieldValue::UInteger(value.into())
    }
}

impl From<u64> for FieldValue {
    fn from(value: u64) -> Self {
        FieldValue::UInteger(value)
    }
}

impl From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        FieldValue::Boolean(value)
    }
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FieldValue::Float(number) => format!("{number}"),
            FieldValue::Integer(number) => format!("{number}i"),
            FieldValue::UInteger(number) => format!("{number}i"),
            FieldValue::String(string) => format!("{string}"),
            FieldValue::Boolean(boolean) => format!("{boolean}"),
        };

        write!(f, "{}", value)
    }
}

impl PartialEq for FieldValue {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Convert for FieldValue {
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Check if string is a number that ends with an i
        let re = Regex::new(r"^-?\d+i$").unwrap();
        if re.is_match(s) {
            // Remove the `i`
            let mut number = s.to_string();
            number.pop();

            let value = match number.starts_with("-") {
                true => {
                    let int = number
                        .parse::<i64>()
                        .with_context(|| format!("number {s} is not a valid integer"))?;

                    FieldValue::Integer(int)
                }
                false => {
                    let uint = number
                        .parse::<u64>()
                        .with_context(|| format!("number {s} is not a valid unsigned integer"))?;

                    FieldValue::UInteger(uint)
                }
            };

            return Ok(value);
        };

        // Check if string is a float or just a regular number without and `i`
        if let Ok(number) = s.parse::<f64>() {
            return Ok(FieldValue::Float(number));
        }

        // Check if its a boolean, else treat as a string
        let value = match s {
            "t" | "T" | "true" | "True" | "TRUE" => FieldValue::Boolean(true),
            "f" | "F" | "false" | "False" | "FALSE" => FieldValue::Boolean(false),
            _ => FieldValue::String(s.to_string()),
        };

        Ok(value)
    }
}

impl Format for FieldValue {
    fn escape(&self) -> Self {
        match self {
            FieldValue::String(string) => {
                let escaped = string.replace("\\", "\\\\").replace("\"", "\\\"");
                FieldValue::String(format!("\"{escaped}\""))
            }
            other => other.clone(),
        }
    }

    fn unescape(&self) -> Self {
        match self {
            FieldValue::String(string) => {
                let unescaped = match string.starts_with("\"") && string.ends_with("\"") {
                    true => &string[1..string.len() - 1],
                    false => string.as_str(),
                };
                FieldValue::String(unescaped.replace("\\\"", "\"").replace("\\\\", "\\"))
            }
            other => other.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag_key_escape_unescape() {
        let key = TagKey::from("some, value=");
        let escaped_key = key.escape();

        assert_eq!(escaped_key.to_string(), "some\\,\\ value\\=");

        let unescaped_key = escaped_key.unescape();
        assert_eq!(unescaped_key.to_string(), "some, value=");
    }

    #[test]
    fn test_tag_value_escape_unescape() {
        let value = TagValue::from("some, value=");
        let escaped_value = value.escape();

        assert_eq!(escaped_value.to_string(), "some\\,\\ value\\=");

        let unescaped_value = escaped_value.unescape();
        assert_eq!(unescaped_value.to_string(), "some, value=");
    }

    #[test]
    fn test_field_key_escape_unescape() {
        let key = FieldKey::from("some, value=");
        let escaped_key = key.escape();

        assert_eq!(escaped_key.to_string(), "some\\,\\ value\\=");

        let unescaped_key = escaped_key.unescape();
        assert_eq!(unescaped_key.to_string(), "some, value=");
    }

    #[test]
    fn test_field_value_escape_unescape() {
        // Only strings are escaped, every other value is as is
        let value = FieldValue::from("{\"foo\": [\"bar=\\baz\"]}");
        let escaped_value = value.escape();

        assert_eq!(
            escaped_value.to_string(),
            "\"{\\\"foo\\\": [\\\"bar=\\\\baz\\\"]}\""
        );

        let unescaped_value = escaped_value.unescape();
        assert_eq!(unescaped_value.to_string(), "{\"foo\": [\"bar=\\baz\"]}");
    }

    #[test]
    fn test_field_value_parse_float() {
        let parsed = FieldValue::parse("10.0").unwrap();
        let expected = FieldValue::Float(10.);
        assert_eq!(parsed, expected)
    }

    #[test]
    fn test_field_value_parse_signed_integer() {
        let parsed = FieldValue::parse("-10i").unwrap();
        let expected = FieldValue::Integer(-10);
        assert_eq!(parsed, expected);

        let parsed = FieldValue::parse("10i").unwrap();
        let expected = FieldValue::Integer(10);
        assert_eq!(parsed, expected)
    }

    #[test]
    fn test_field_value_parse_unsigned_integer() {
        // Only if a number cannot fit in an i64 it will parsed into a u64
        let parsed = FieldValue::parse("9223372036854775808i").unwrap();
        let expected = FieldValue::UInteger(9223372036854775808);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_field_value_parse_boolean() {
        let true_variants = vec!["t", "T", "true", "True", "TRUE"];
        for variant in true_variants {
            let parsed = FieldValue::parse(variant).unwrap();
            let expected = FieldValue::Boolean(true);
            assert_eq!(parsed, expected);
        }

        let false_variants = vec!["f", "F", "false", "False", "FALSE"];
        for variant in false_variants {
            let parsed = FieldValue::parse(variant).unwrap();
            let expected = FieldValue::Boolean(false);
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_field_value_display() {
        assert_eq!(FieldValue::Float(10.0).to_string(), "10");
        assert_eq!(FieldValue::Float(10.5).to_string(), "10.5");
        assert_eq!(FieldValue::Integer(10).to_string(), "10i");
        assert_eq!(FieldValue::UInteger(10).to_string(), "10i");
        assert_eq!(FieldValue::String("hello".to_string()).to_string(), "hello");
        assert_eq!(FieldValue::Boolean(true).to_string(), "true");
        assert_eq!(FieldValue::Boolean(false).to_string(), "false");
    }
}
