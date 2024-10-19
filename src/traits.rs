pub trait Format {
    fn escape(&self) -> Self;
    fn unescape(&self) -> Self;
}

pub trait Convert {
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}
