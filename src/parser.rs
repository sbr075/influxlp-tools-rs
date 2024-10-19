use std::{collections::HashMap, hash::Hash};

use crate::error::{ParseError, Result};

use crate::{
    element::{FieldKey, FieldValue, Measurement, TagKey, TagValue},
    traits::{Convert, Format},
    LineProtocol,
};

impl LineProtocol {
    /// Split a line protocol part from the rest of the line protocol
    fn parse_part<P>(chars: &mut P) -> String
    where
        P: Iterator<Item = char>,
    {
        let mut in_quote = false;
        let mut is_escaped = false;

        // Parse the measurement name
        let mut part = String::new();
        while let Some(char) = chars.next() {
            // If the current character is a \ (slash) then we know the next character must
            // be escaped
            if char == '\\' {
                is_escaped = true;
            }
            // Toggle the `in_quote` flag if the current character is a double quote and the
            // previous character was not an escape character
            else if char == '"' && !is_escaped {
                in_quote = !in_quote;
            // If the current character is a ' ' (space) and we are not in a
            // quote or its not escaped we've finished a part
            } else if char == ' ' && (!is_escaped && !in_quote) {
                break;
            } else {
                // We've gone past the escaped character
                is_escaped = false;
            }

            part.push(char);
        }

        part.trim().to_string()
    }

    /// Parses a set (tag- or field set) into a hashmap of the defined key-value
    /// types
    fn parse_set<K, V>(set: &str) -> Result<HashMap<K, V>>
    where
        K: Format + Convert + Hash + PartialEq + Eq,
        V: Format + Convert,
    {
        let mut in_quote = false;
        let mut is_escaped = false;

        let mut word = String::new();
        let mut words = Vec::new();
        for char in set.chars() {
            // If the current character is a \ (slash) then we know the next character must
            // be escaped
            if char == '\\' {
                is_escaped = true;
                word.push(char);
            }
            // We toggle the `in_quote` flag if the current character is a double quote and the
            // previous character was not an escape character
            else if char == '"' && !is_escaped {
                in_quote = !in_quote;
                word.push(char);
            }
            // If the current character is a `=` (equals sign) and its not escaped we've finished a
            // word or if the current character is a `,` (comma) and we are not in a quote we've
            // finished a word
            else if (char == '=' && !is_escaped) || (char == ',' && !in_quote) {
                words.push(word.clone());
                word.clear();
                continue;
            } else {
                // We've gone past the escaped character
                is_escaped = false;
                word.push(char);
            }
        }

        // Push whatever is left
        words.push(word);

        // If we don't have an even number of words the given set is invalid
        if words.len() % 2 != 0 {
            return Err(
                ParseError::InvalidSet("set contains uneven amount of values".into()).into(),
            );
        }

        // Transform to a hashmap and unescape words
        let mut set = HashMap::new();
        for word in words.chunks_exact(2) {
            // Only FieldValue can actually return an error
            let key = K::parse(&word[0]).map_err(|e| ParseError::InvalidSet(e.into()))?;
            let value = V::parse(&word[1]).map_err(|e| ParseError::InvalidSet(e.into()))?;

            set.insert(key.unescape(), value.unescape());
        }

        Ok(set)
    }

    /// Parses the identifier (measurement and tag set)
    fn parse_identifiers(
        input: String,
    ) -> Result<(Measurement, Option<HashMap<TagKey, TagValue>>)> {
        let mut chars = input.chars();
        let mut is_escaped = false;

        let mut measurement = String::new();
        while let Some(char) = chars.next() {
            // If the current character is a \ (slash) then we know the next character must
            // be escaped
            if char == '\\' {
                is_escaped = true;
            } else if char == ',' && !is_escaped {
                break;
            } else {
                is_escaped = false;
            }

            measurement.push(char);
        }

        let measurement = Measurement::from(measurement).unescape();

        let tag_set = chars.collect::<String>();
        let tags = match !tag_set.is_empty() {
            true => Some(LineProtocol::parse_set::<TagKey, TagValue>(&tag_set)?),
            false => None,
        };

        Ok((measurement, tags))
    }

    /// Parse a single line protocol line into the [LineProtocol] struct
    ///
    /// Allows for modifying the line protocol by adding or removing fields/tags
    /// and rebuilding
    ///
    /// # Example
    /// ```rust,ignore
    /// let line = "measurement,tag=value field=true 1729270461612452700"
    /// let parsed_line = LineProtocol::parse_line(line).unwrap();
    ///
    /// parsed_line.delete_tag("tag");
    /// parsed_line.add_field("field2", "hello");
    /// parsed_line.with_timestamp(1729270461612452800i64)
    ///
    /// let line = parsed_line.build().unwrap();
    /// // Output: measurement field=true,field2="hello" 1729270461612452800
    /// ```
    ///
    /// # Args
    /// * `line` - A InfluxDB line protocol line
    pub fn parse_line(line: &str) -> Result<Self> {
        // Trim away leading and trailing whitespace
        let line = line.trim();

        // Comment line
        if line.starts_with("#") {
            return Err(ParseError::CommentLine.into());
        }

        // Can't parse empty lines
        if line.is_empty() {
            return Err(ParseError::EmptyLine.into());
        }

        let mut chars = line.chars();

        // Parse measurement and tags
        let identifiers = LineProtocol::parse_part(&mut chars);
        let (measurement, tags) = LineProtocol::parse_identifiers(identifiers)?;

        // Parse field set
        let field_set = LineProtocol::parse_part(&mut chars);
        if field_set.is_empty() {
            return Err(ParseError::MissingFields.into());
        }

        let fields = LineProtocol::parse_set::<FieldKey, FieldValue>(&field_set)?;

        // Timestamp is the only part remaining
        let timestamp = chars.collect::<String>();
        let timestamp = match !timestamp.is_empty() {
            true => {
                let timestamp = match timestamp.parse::<i64>() {
                    Ok(timestamp) => timestamp,
                    Err(_) => return Err(ParseError::InvalidTimestamp.into()),
                };
                Some(timestamp)
            }
            false => None,
        };

        let line_protocol = Self {
            measurement: Measurement::from(measurement),
            tags,
            fields,
            timestamp,
        };
        Ok(line_protocol)
    }

    /// Parse a vector of lines
    ///
    /// Empty lines and comment lines are silently ignored
    ///
    /// # Example
    /// ```rust,ignore
    /// let lines = vec![
    ///     "measurement,tag=value field=\"value\"",
    ///     "measurement,tag=value field=true 1729270461612452700",
    ///     ...
    /// ];
    ///
    /// let parsed = LineProtocol::parse_vec(lines).unwrap();
    /// ```
    ///
    /// # Args
    /// * `lines` - An array of InfluxDB line protocol lines
    pub fn parse_vec(lines: Vec<&str>) -> Result<Vec<Self>> {
        let mut parsed_lines: Vec<LineProtocol> = Vec::new();
        for line in lines {
            // Ignore comment lines
            if line.starts_with("#") {
                continue;
            }

            // Ignore empty lines
            if line.is_empty() {
                continue;
            }

            // If the line protocol has been parsed earlier but is a duplicate we just add
            // the fields value to the original but favor the latter
            let parsed_line = LineProtocol::parse_line(line)?;
            match parsed_lines.iter_mut().find(|l| **l == parsed_line) {
                Some(lp) => lp.fields.extend(parsed_line.fields),
                None => parsed_lines.push(parsed_line),
            }
        }

        Ok(parsed_lines)
    }

    /// Parse multiple lines seprated by a newline (\n)
    ///
    /// Empty lines and comment lines are silently ignored
    ///
    /// # Example
    /// ```rust,ignore
    /// let lines = vec![
    ///     "measurement,tag=value field=\"value\"",
    ///     "measurement,tag=value field=true 1729270461612452700",
    ///     ...
    /// ];
    ///
    /// let parsed = LineProtocol::parse_lines(lines.join("\n")).unwrap();
    /// ```
    ///
    /// # Args
    /// * `lines` - Multiple InfluxDB line protocol lines seperated by a newline
    pub fn parse_lines(lines: &str) -> Result<Vec<Self>> {
        let parsed_lines = LineProtocol::parse_vec(lines.lines().collect())?;
        Ok(parsed_lines)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_protocol_parser_measurement_name() {
        let line = "some\\,\\ name,tag=value field=\"value\" 1729270461612452700";
        let result = LineProtocol::parse_line(line);
        assert!(result.is_ok())
    }

    #[test]
    fn test_line_protocol_parser_ok() {
        let line = "measurement,tag2=value,tag=value field=\"value\",field2=\"{\\\"test\\\": \
                    \\\"hello\\\"}\" 1729270461612452700";
        let result = LineProtocol::parse_line(line);
        assert!(result.is_ok())
    }

    #[test]
    fn test_line_protocol_parser_lines() {
        let lines = vec![
            "measurement,tag=value field=\"value\",field2=true",
            "measurement field=\"{\\\"test\\\": \\\"hello\\\"}\"",
            "measurement,tag2=value,tag=value field=\"value\",field2=\"{\\\"test\\\": \
             \\\"hello\\\"}\" 1729270461612452700",
        ]
        .join("\n");

        let result = LineProtocol::parse_lines(&lines);
        assert!(result.is_ok())
    }
}
