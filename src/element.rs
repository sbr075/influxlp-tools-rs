use std::{fmt::Display, str::FromStr};

use anyhow::Context;
use regex::Regex;

use crate::traits::Format;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Measurement(pub String);

impl Measurement {
    // Characters that need to be escaped
    // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters
    pub fn escape(&self) -> String {
        self.0.replace(" ", r"\ ").replace(",", r"\,")
    }

    pub fn unescape(&self) -> String {
        self.0.replace(r"\,", ",").replace(r"\ ", " ")
    }
}

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TagKey(pub String);

impl Format for TagKey {
    // Characters that need to be escaped
    // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters
    fn escape(&self) -> String {
        self.0
            .replace(" ", r"\ ")
            .replace(",", r"\,")
            .replace("=", r"\=")
    }

    fn unescape(&self) -> String {
        self.0
            .replace(r"\=", "=")
            .replace(r"\,", ",")
            .replace(r"\ ", " ")
    }
}
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagValue(pub String);

impl Format for TagValue {
    // Characters that need to be escaped
    // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters
    fn escape(&self) -> String {
        self.0
            .replace(" ", r"\ ")
            .replace(",", r"\,")
            .replace("=", r"\=")
    }

    fn unescape(&self) -> String {
        self.0
            .replace(r"\=", "=")
            .replace(r"\,", ",")
            .replace(r"\ ", " ")
    }
}

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldKey(pub String);

impl Format for FieldKey {
    // Characters that need to be escaped
    // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters
    fn escape(&self) -> String {
        self.0
            .replace(" ", r"\ ")
            .replace(",", r"\,")
            .replace("=", r"\=")
    }

    fn unescape(&self) -> String {
        self.0
            .replace(r"\=", "=")
            .replace(r"\,", ",")
            .replace(r"\ ", " ")
    }
}

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
        match FieldValue::from_str(value) {
            Ok(value) => value,
            Err(_) => FieldValue::String(value.to_owned()),
        }
    }
}

impl From<&String> for FieldValue {
    fn from(value: &String) -> Self {
        match FieldValue::from_str(value) {
            Ok(value) => value,
            Err(_) => FieldValue::String(value.to_owned()),
        }
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        match FieldValue::from_str(&value) {
            Ok(value) => value,
            Err(_) => FieldValue::String(value),
        }
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

impl FromStr for FieldValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if string is a number that ends with an i
        let re = Regex::new(r"^-?\d+i$").unwrap();
        if re.is_match(s) {
            // Remove the `i`
            let mut number = s.to_string();
            number.pop();

            let value = match number.starts_with("-") {
                true => {
                    let int = s
                        .parse::<i64>()
                        .with_context(|| format!("number {s} is not a valid integer"))?;

                    FieldValue::Integer(int)
                }
                false => {
                    let uint = s
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
    // Characters that need to be escaped
    // https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters
    fn escape(&self) -> String {
        match self {
            FieldValue::String(string) => {
                let escaped = string.replace("\\", "\\\\").replace("\"", "\\\"");
                format!("\"{escaped}\"")
            }
            other => other.to_string(),
        }
    }

    fn unescape(&self) -> String {
        match self {
            FieldValue::String(string) => {
                let unescaped = match string.starts_with("\"") && string.ends_with("\"") {
                    true => &string[1..string.len() - 1],
                    false => string.as_str(),
                };
                unescaped.replace("\\\"", "\"").replace("\\\\", "\\")
            }
            other => other.to_string(),
        }
    }
}
