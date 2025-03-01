pub trait ActionList<T> {
    fn uninit() -> Self;

    fn pop_random(&mut self) -> Option<T>;

    fn push(&mut self, action: T);

    fn is_empty(&self) -> bool;

    fn has(&self, item: &T) -> bool
    where
        T: PartialEq;

    fn without(&self, other: &Self) -> Self;
}

//impl<T> ActionList<T> for Vec<T> {
//    fn pop_random(&mut self) -> Option<T> {
//        self.pop()
//    }
//
//    fn is_empty(&self) -> bool {
//        self.is_empty()
//    }
//
//    fn has(&self, item: &T) -> bool
//    where
//        T: PartialEq,
//    {
//        self.contains(item)
//    }
//}
