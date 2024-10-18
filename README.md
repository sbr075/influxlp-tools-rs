# InfluxDB V2 Line Protocol Tools
[![Influx line protocol tools on crates.io][crates.io-image]][crates.io]

[crates.io-image]: https://img.shields.io/badge/crates.io-influxlp--tools-orange
[crates.io]: https://crates.io/crates/influxlp-tools

---

</br>

```toml
[dependencies]
influxlp-tools = "0.1.2"
```

InfluxDB's line protocol is a text-based format used to represent data points. It includes the measurement, tag set, field set, and an optional timestamp.

```
measurement,tag=value field=value 1729270461612452700
```

Read more about it [here!](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/)

</br>

## Usage
### Builder
The builder allows you to create valid line protocol strings
```rust
let line_protocol = LineProtocol::new("measurement")
    .add_tag("tag", "value")
    .add_field("field", "value")
    .build()
    .unwrap();

// Output: measurement,tag=value field="value"
```

```rust
let line_protocol = LineProtocol::new("measurement")
    .add_field("field","{\"test\": \"hello\"}")
    .build()
    .unwrap();

// measurement field="{\"test\": \"hello\"}"
```

```rust
let line_protocol = LineProtocol::new("measurement")
    .add_tag("tag", "value")
    .add_tag("tag2", "value")
    .add_field("field", "value")
    .add_field("field2", "{\"test\": \"hello\"}")
    .with_timestamp(1729270461612452700i64)
    .build()
    .unwrap();

// Output: measurement,tag2=value,tag=value field="value",field2="{\"test\": \"hello\"}" 1729270461612452700
```

</br>

### Parser
The parser allows you to parse valid line protocol strings to the LineProtocol struct

```rust
let line = "measurement,tag2=value,tag=value field=\"value\",field2=\"{\\\"test\\\": \\\"hello\\\"}\" 1729270461612452700";
let line_protocol = LineProtocol::parse_line(line).unwrap();

// Output: LineProtocol {
//     measurement: Measurement(
//         "measurement",
//     ),
//     tags: Some(
//         {
//             TagKey(
//                 "tag2",
//             ): TagValue(
//                 "value",
//             ),
//             TagKey(
//                 "tag",
//             ): TagValue(
//                 "value",
//             ),
//         },
//     ),
//     fields: {
//         FieldKey(
//             "field2",
//         ): String(
//             "{\"test\": \"hello\"}",
//         ),
//         FieldKey(
//             "field",
//         ): String(
//             "value",
//         ),
//     },
//     timestamp: Some(
//         1729270461612452700,
//     ),
// }
```

You can also parse multiple lines

```rust
let lines = vec![
    "measurement,tag=value field=\"value\"",
    "measurement field=\"{\\\"test\\\": \\\"hello\\\"}\"",
    "measurement,tag2=value,tag=value field=\"value\",field2=\"{\\\"test\\\": \
            \\\"hello\\\"}\" 1729270461612452700"
].join("\n");

let result = LineProtocol::parse_lines(&lines);

// Output: [
//     LineProtocol {
//         measurement: Measurement(
//             "measurement",
//         ),
//         tags: Some(
//             {
//                 TagKey(
//                     "tag",
//                 ): TagValue(
//                     "value",
//                 ),
//             },
//         ),
//         fields: {
//             FieldKey(
//                 "field",
//             ): String(
//                 "value",
//             ),
//         },
//         timestamp: None,
//     },
//     LineProtocol {
//         measurement: Measurement(
//             "measurement",
//         ),
//         tags: None,
//         fields: {
//             FieldKey(
//                 "field",
//             ): String(
//                 "{\"test\": \"hello\"}",
//             ),
//         },
//         timestamp: None,
//     },
//     LineProtocol {
//         measurement: Measurement(
//             "measurement",
//         ),
//         tags: Some(
//             {
//                 TagKey(
//                     "tag2",
//                 ): TagValue(
//                     "value",
//                 ),
//                 TagKey(
//                     "tag",
//                 ): TagValue(
//                     "value",
//                 ),
//             },
//         ),
//         fields: {
//             FieldKey(
//                 "field2",
//             ): String(
//                 "{\"test\": \"hello\"}",
//             ),
//             FieldKey(
//                 "field",
//             ): String(
//                 "value",
//             ),
//         },
//         timestamp: Some(
//             1729270461612452700,
//         ),
//     },
// ]
```