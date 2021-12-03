use ndarray::Array2;

pub fn step_grid<T, F>(g: &Array2<T>, f: F) -> (Array2<T>, usize)
where
    T: Clone + Default + PartialEq,
    F: Fn(&Array2<T>, &T, (usize, usize)) -> T,
{
    let mut changes = 0;
    let new_grid = Array2::from_shape_fn(g.raw_dim(), |p| {
        let curr = &g[p];
        let new = f(g, curr, p);
        if &new != curr {
            changes += 1;
        }
        new
    });
    (new_grid, changes)
}

pub fn step_grid_into<T, F>(g1: &Array2<T>, g2: &mut Array2<T>, f: F) -> usize
where
    T: Clone + Default + PartialEq,
    F: Fn(&Array2<T>, &T, (usize, usize)) -> T,
{
    let mut changes = 0;
    for (p, curr) in g1.indexed_iter() {
        let new = f(g1, curr, p);
        if &new != curr {
            changes += 1;
        }
        g2[p] = new;
    }
    changes
}

pub fn mut_grid<T, F>(g: &mut Array2<T>, f: F) -> usize
where
    T: PartialEq + Clone,
    F: Fn(&Array2<T>, &T, (usize, usize)) -> T,
{
    //I know this looks like a needless collect, but I need to
    //collect all the necessary changes, and then apply them to the grid, otherwise
    //the borrow checker gets angry. (and rightly so)
    #[allow(clippy::needless_collect)]
    let deltas = g
        .indexed_iter()
        .filter_map(|(p, curr)| {
            let new = f(g, curr, p);
            if curr == &new {
                None
            } else {
                Some((p, new))
            }
        })
        .collect::<Vec<_>>();
    deltas.into_iter().map(|(p, new)| g[p] = new).count()
}
