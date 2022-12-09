use std::cmp::Ordering;

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

pub fn borrow_mut_twice<T>(arr: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    match a.cmp(&b) {
        Ordering::Less => {
            let (arr_a, arr_b) = arr.split_at_mut(b);
            (&mut arr_a[a], &mut arr_b[0])
        }
        Ordering::Greater => {
            let (arr_b, arr_a) = arr.split_at_mut(a);
            (&mut arr_a[0], &mut arr_b[b])
        }
        Ordering::Equal => panic!("Can't borrow twice from the same index"),
    }
}

pub fn slice_get_mut_two<T>(slice: &mut [T], index0: usize, index1: usize) -> (&mut T, &mut T) {
    assert_ne!(index0, index1);
    assert!(index0 < slice.len());
    assert!(index1 < slice.len());
    // SAFETY: guarantee that the indices are never the same. So it is safe to
    // have two mutable references into the Vec. We'll double check that the
    // indices are within the bounds.
    unsafe { slice_get_mut_two_unchecked(slice, index0, index1) }
}

/// # Safety
/// index0 and index1 must be < slice.len(), and must not be equal to each other.
pub unsafe fn slice_get_mut_two_unchecked<T>(
    slice: &mut [T],
    index0: usize,
    index1: usize,
) -> (&mut T, &mut T) {
    let ptr = slice.as_mut_ptr();
    let one = &mut *ptr.add(index0);
    let two = &mut *ptr.add(index1);
    (one, two)
}
