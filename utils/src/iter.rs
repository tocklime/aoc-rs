pub fn all_ix_pairs(arr_len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..arr_len - 1).flat_map(move |ix1| (ix1 + 1..arr_len).map(move |ix2| (ix1, ix2)))
}

pub fn all_new_greatest_with<T, TInner, F>(
    iter: impl Iterator<Item = T>,
    f: F,
) -> impl Iterator<Item = T>
where
    T: Ord,
    F: Fn(&T) -> TInner,
    TInner: PartialOrd + Copy,
{
    let mut biggest = None;
    iter.filter_map(move |i| {
        let val = f(&i);
        if Some(val) > biggest {
            biggest = Some(val);
            Some(i)
        } else {
            None
        }
    })
}
