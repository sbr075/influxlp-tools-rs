pub trait Format {
    /// Escapes [special character](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters) in the string
    fn escape(&self) -> Self;

    /// Unescapes the escaped string in reverse order
    fn unescape(&self) -> Self;
}

pub trait Convert {
    /// Parse a string into this type
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}
