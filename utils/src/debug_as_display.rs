use std::fmt::{Debug, Display};

pub struct DebugAsDisplay<T> {
    inner: T,
}

impl<T> From<T> for DebugAsDisplay<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}
impl<T: Display> Debug for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.inner))
    }
}
