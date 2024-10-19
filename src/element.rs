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
