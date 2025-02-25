use std::ops::Index;

pub struct Array<T: Copy + Default, const N: usize> {
    data: [T; N],
    index: usize,
}

impl<T: Copy + Default, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        Self {
            data: [Default::default(); N],
            index: 0,
        }
    }

    pub const fn clear(&mut self) {
        self.index = 0;
    }

    pub const fn get(&self, index: usize) -> Option<T> {
        if index >= self.index {
            None
        } else {
            Some(self.data[index])
        }
    }

    pub const fn push(&mut self, value: T) {
        assert!(self.index < N);

        self.data[self.index] = value;
        self.index += 1;
    }

    pub const fn remove(&mut self, pos: usize) {
        assert!(pos < self.index && pos < N);
        assert!(pos < N);
        assert!(self.index > 0);

        let mut i = pos + 1;
        while i < self.index {
            self.data[i - 1] = self.data[i];
            i += 1;
        }

        self.index -= 1;
    }

    pub const fn len(&self) -> usize {
        self.index
    }
}

impl<T: Copy + Default, const N: usize> Index<usize> for Array<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Copy + Default, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Array::new()
    }
}

impl<T: Copy + Default, const N: usize> FromIterator<T> for Array<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ar = Array::new();
        for x in iter {
            ar.push(x);
        }

        ar
    }
}
