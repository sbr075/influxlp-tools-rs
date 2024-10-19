# InfluxDB V2 Line Protocol Tools

[![Influx line protocol tools on crates.io][crates.io-image]][crates.io]

---

[crates.io-image]: https://img.shields.io/badge/crates.io-influxlp--tools-orange
[crates.io]: https://crates.io/crates/influxlp-tools


**InfluxDB V2 Line Protocol Tools is a parsing and building library for InfluxDB v2's line protocol. It provides easy-to-use functionality with built-in validation, support for a builder pattern and dynamic population, and options for modifying existing line protocols**

</br>

InfluxDB's line protocol is a text-based format used to represent data points. It includes the measurement, tag set, field set, and an optional timestamp.

```
measurement         tag set             field set              timestamp
----------- ------------------- ------------------------- -------------------
measurement,tag1=val1,tag2=val2 field1="val1",field2=true 1729270461612452700
```

Read more about it [here!](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/)

## Documenation

See [docs.rs](https://docs.rs/influxlp-tools/latest/influxlp_tools/) for more information

## Usage

### Building a line protocol string

At minimum the measurement name and a field is required to build a valid line protocol string

```rust
let line_protocol = LineProtocol::new("measurement")
    .add_field("field", "value")
    .build()
    .unwrap();
```

You can overwrite the measurement name by calling the `measurement` method

```rust
let mut line_protocol = LineProtocol::new("measurement")
    .add_field("field", "value")
    .build()
    .unwrap();

line_protocol = line_protocol.measurement("new_measurement");
```

Multiple fields can be add by calling the `add_field` method multiple times

```rust
let line_protocol = LineProtocol::new("measurement")
    .add_field("field1", "value")
    .add_field("field2", "value")
    .build()
    .unwrap();
```

Optionally tags can be added. More tags can be added as with fields

```rust
let line_protocol = LineProtocol::new("measurement")
    .add_tag("tag1", "value")
    .add_tag("tag2", "value")
    .add_field("field", "value")
    .build()
    .unwrap();
```

A timestamp can be added with the `with_timestamp` method. By default the timestamp is defined in nanosecond precision. If you are using any other precision, e.g., seconds, it needs be defined when querying influx

```rust
let line_protocol = LineProtocol::new("measurement")
    .add_field("field", "value")
    .with_timestamp(1729270461612452700i64)
    .build()
    .unwrap();
```

A field, tag, and timestamp can be deleted if needed. This is done by calling the respective `delete` function

```rust
let mut line_protocol = LineProtocol::new("measurement")
    .add_tag("tag", "value")
    .add_field("field", "value");

line_protocol.delete_tag("tag")
```

**Note:** that deleting all fields will cause the building to fail as atleast **one** field is required

</br>

### Parsing a line protocol string

To parse a line protocol string the `parse_line` method can be used

```rust
let line = "measurement,tag2=value,tag=value field=\"hello\",field2=\"world\" 1729270461612452700";
let line_protocol = LineProtocol::parse_line(line).unwrap();
```

To parse multiple lines seperated by a newline the `parse_lines` method can be used instead

```rust
let lines = vec![
    "measurement,tag=value field=\"value\"",
    "measurement field=\"{\\\"test\\\": \\\"hello\\\"}\"",
    "measurement,tag2=value,tag=value field=\"value\",field2=\"{\\\"test\\\": \
            \\\"hello\\\"}\" 1729270461612452700"
].join("\n");

let result = LineProtocol::parse_lines(&lines);
```

**Note:** The parsed line can be modified and rebuilt if needed

## Issues or new features
If you discover any issues to be fixed or features you'd like to be introduced you can open up a issue and I'll take a look at it whenever I have time. I am going to be maintaing this crate on and off depending on how much time I have.


## License

This project is licensed under either of

* [MIT License](LICENSE-MIT)
* [APACHE License](LICENSE-APACHE)

at your option.