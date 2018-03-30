use std::cmp::Ordering;

pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T { if a < b { a } else { b } }
pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct ForceOrd<T>(pub T);

impl<T: PartialEq> Eq for ForceOrd<T> {}
impl<T: PartialOrd> Ord for ForceOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
