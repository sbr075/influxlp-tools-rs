pub trait Format {
    fn escape(&self) -> Self;
    fn unescape(&self) -> Self;
}
