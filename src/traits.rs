use std::str::FromStr;

pub trait Format {
    /// Escapes [special character](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#special-characters) in the string
    fn escape(&self) -> Self;

    /// Unescapes the escaped string in reverse order
    fn unescape(&self) -> Self;
}

pub trait Convert {
    fn parse_from<T>(from: T) -> anyhow::Result<Self>
    where
        Self: Sized,
        T: ToString;

    fn parse_into<T>(&self) -> anyhow::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static;
}
