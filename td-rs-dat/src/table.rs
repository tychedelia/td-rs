use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct Table<T> {
    size: [usize; 2],
    r: HashMap<[usize; 2], T>,
    w: HashMap<[usize; 2], T>,
}

impl<T> Table<T> {
    pub fn new(size: [usize; 2]) -> Self {
        Self {
            size,
            r: HashMap::new(),
            w: HashMap::new(),
        }
    }

    pub fn size(&self) -> [usize; 2] {
        self.size
    }

    pub fn contains_key(&self, index: [usize; 2]) -> bool {
        self.r.contains_key(&index) || self.w.contains_key(&index)
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&[usize; 2], &T),
    {
        for (k, v) in self.w.iter() {
            f(k, v);
        }
    }

    pub fn resize(&mut self, size: [usize; 2]) {
        self.size = size;
    }
}

impl<T> Index<[usize; 2]> for Table<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, col] = index;
        if row >= self.size[0] || col >= self.size[1] {
            panic!("Index out of bounds: {:?}", index);
        }

        if self.w.contains_key(&index) {
            return &self.w[&index];
        }

        &self.r[&index]
    }
}

impl<T> IndexMut<[usize; 2]> for Table<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, col] = index;
        if row >= self.size[0] || col >= self.size[1] {
            panic!("Index out of bounds: {:?}", index);
        }

        self.w.entry(index).or_insert_with(|| T::default())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    #[test]
    fn test_table() {
        let mut table = super::Table::new([2, 2]);
        table[[0, 0]] = 1;
        assert_eq!(table[[0, 0]], 1);
    }

    #[test]
    fn test_table_default() {
        let table = super::Table::<i32>::new([2, 2]);
        assert_eq!(table[[0, 0]], 0);
    }

    #[test]
    fn test_table_default_2() {
        let mut table = super::Table::<i32>::new([2, 2]);
        table[[0, 0]] = 1;
        assert_eq!(table[[0, 0]], 1);
        assert_eq!(table[[0, 1]], 0);
    }

    #[test]
    fn test_table_for_each() {
        let mut table = super::Table::new([2, 2]);
        table[[0, 0]] = 1;
        table[[0, 1]] = 2;
        table[[1, 0]] = 3;
        table[[1, 1]] = 4;
        let mut sum = 0;
        table.for_each(|_, v| sum += v);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_table_resize() {
        let mut table = super::Table::new([2, 2]);
        table[[0, 0]] = 1;
        table[[0, 1]] = 2;
        table[[1, 0]] = 3;
        table[[1, 1]] = 4;
        table.resize([3, 3]);
        assert_eq!(table[[0, 0]], 1);
        assert_eq!(table[[0, 1]], 2);
        assert_eq!(table[[1, 0]], 3);
        assert_eq!(table[[1, 1]], 4);
        assert_eq!(table[[2, 0]], 0);
    }
}
