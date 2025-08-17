#![no_std]

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceLineInfo {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl core::fmt::Display for SourceLineInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { file, line, column } = *self;
        // Same format as rustc's diagnostic reports
        write!(f, "{file}:{line}:{column}")
    }
}

impl SourceLineInfo {
    #[must_use]
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

#[macro_export]
macro_rules! source_line_info {
    () => {
        $crate::SourceLineInfo::new(file!(), line!(), column!())
    };
}