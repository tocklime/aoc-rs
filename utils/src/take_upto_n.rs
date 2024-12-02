pub struct TakeUpToN<'a, V, T: Iterator<Item = V>> {
    inner: &'a mut T,
    remaining: usize,
}
impl<V, T: Iterator<Item = V>> Drop for TakeUpToN<'_, V, T> {
    fn drop(&mut self) {
        for _ in 0..self.remaining {
            self.inner.next();
        }
    }
}
impl<V, T: Iterator<Item = V>> Iterator for TakeUpToN<'_, V, T> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            self.inner.next()
        }
    }
}
impl<'a, V, T: Iterator<Item = V>> TakeUpToN<'a, V, T> {
    pub fn new(inner: &'a mut T, item_count: usize) -> Self {
        Self {
            inner,
            remaining: item_count,
        }
    }
}
