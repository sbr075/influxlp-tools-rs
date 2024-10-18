pub trait Format {
    fn escape(&self) -> String;
    fn unescape(&self) -> String;
}
