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

    pub const fn push(&mut self, value: T) {
        assert!(self.index < N);

        self.data[self.index] = value;
        self.index += 1;
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
        let mut array = Array::new();
        for x in iter {
            array.push(x);
        }

        array
    }
}
