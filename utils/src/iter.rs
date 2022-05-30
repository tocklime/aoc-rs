pub fn all_ix_pairs(arr_len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..arr_len - 1).flat_map(move |ix1| (ix1 + 1..arr_len).map(move |ix2| (ix1, ix2)))
}
