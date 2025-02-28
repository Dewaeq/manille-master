pub trait ActionList<T> {
    fn pop(&mut self) -> Option<T>;

    fn is_empty(&self) -> bool;

    fn has(&self, item: &T) -> bool
    where
        T: PartialEq;
}

impl<T> ActionList<T> for Vec<T> {
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn has(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.contains(item)
    }
}
