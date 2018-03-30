use Rgb24;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Entry {
    pub ident: &'static str,
    pub value: Rgb24,
}
