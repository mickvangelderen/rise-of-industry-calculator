pub trait ExactlyOne: Iterator + Sized {
    fn exactly_one(self) -> Option<Self::Item>;
}

impl<T> ExactlyOne for T
where
    T: Iterator,
{
    fn exactly_one(mut self) -> Option<Self::Item> {
        match (self.next(), self.next()) {
            (Some(first), None) => Some(first),
            _ => None,
        }
    }
}
