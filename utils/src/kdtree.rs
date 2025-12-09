use std::{
    collections::{BTreeSet, HashSet},
    sync::Arc,
};

use itertools::Itertools;

#[derive(Debug)]
enum KdPtr<const K: usize, T> {
    Empty,
    Single([i64; K], T),
    KdTree(Box<KdTree<K, T>>),
}

type MetricFn<const K: usize> = Arc<Box<dyn Fn([i64; K], [i64; K]) -> i64>>;
pub struct KdTree<const K: usize, T> {
    dim: usize,
    split: i64,
    smaller: KdPtr<K, T>,
    bigger: KdPtr<K, T>,
    metric: MetricFn<K>,
}
impl<const K: usize, T: std::fmt::Debug> std::fmt::Debug for KdTree<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KdTree")
            .field("dim", &self.dim)
            .field("split", &self.split)
            .field("smaller", &self.smaller)
            .field("bigger", &self.bigger)
            .finish()
    }
}

impl<T: Copy + PartialEq + std::fmt::Debug, const K: usize> KdTree<K, T> {
    pub fn from(points: &[([i64; K], T)], metric: MetricFn<K>) -> Self {
        Self::from_(points, 0, metric)
    }
    fn from_(points: &[([i64; K], T)], dim: usize, metric: MetricFn<K>) -> Self {
        let sorted: Vec<([i64; K], T)> =
            points.iter().copied().sorted_by_key(|x| x.0[dim]).collect();
        let (smaller_points, bigger_points) = sorted.split_at(sorted.len() / 2);
        let median = bigger_points[0].0[dim];
        let smaller = match smaller_points.len() {
            0 => KdPtr::Empty,
            1 => KdPtr::Single(smaller_points[0].0, smaller_points[0].1),
            _ => KdPtr::KdTree(Box::new(Self::from_(
                smaller_points,
                (dim + 1) % K,
                metric.clone(),
            ))),
        };
        let bigger = match bigger_points.len() {
            0 => KdPtr::Empty,
            1 => KdPtr::Single(bigger_points[0].0, bigger_points[0].1),
            _ => KdPtr::KdTree(Box::new(Self::from_(
                bigger_points,
                (dim + 1) % K,
                metric.clone(),
            ))),
        };
        Self {
            dim,
            split: median,
            smaller,
            bigger,
            metric,
        }
    }
    // fn dist2(a: &[i64; K], b: &[i64; K]) -> i64 {
    //     (0..K).map(|d| (a[d] - b[d]).abs()).map(|x| x * x).sum()
    // }
    fn path_to_node_point<'a>(&'a self, target: [i64; K]) -> Vec<(&'a Self, &'a KdPtr<K, T>)> {
        let choose_path = |n: &'a Self| -> &'a KdPtr<K, T> {
            if target[n.dim] < n.split {
                &n.smaller
            } else {
                &n.bigger
            }
        };
        let ptr = choose_path(self);
        let mut vec = vec![(self, ptr)];
        loop {
            let (_, ptr) = *vec.last().unwrap();
            match ptr {
                KdPtr::KdTree(kd_tree) => {
                    vec.push((kd_tree, choose_path(kd_tree)));
                }
                _ => break,
            }
        }
        vec
    }
    pub fn closest_plane_point(&self, target: [i64; K]) -> [i64; K] {
        let mut ans = target;
        ans[self.dim] = self.split;
        ans
    }
    pub fn find_nearest_to(
        &self,
        target: [i64; K],
        exclude: &HashSet<[i64; K]>,
    ) -> Option<(i64, [i64; K], T)> {
        let mut path = self.path_to_node_point(target);
        let mut closest: Option<(i64, [i64; K], T)> = None;
        let set_closest =
            |cl: &mut Option<(i64, [i64; K], T)>, p: [i64; K], dist2: i64, t: T| match *cl {
                Some((bd, _, _)) if dist2 < bd => {
                    if !exclude.contains(&p) {
                        *cl = Some((dist2, p, t));
                    }
                }
                None => {
                    if !exclude.contains(&p) {
                        *cl = Some((dist2, p, t));
                    }
                }
                _ => {}
            };
        match path.last().unwrap().1 {
            KdPtr::Empty => (),
            KdPtr::Single(p, t) => set_closest(&mut closest, *p, (self.metric)(*p, target), *t),
            KdPtr::KdTree(_) => unreachable!(), //we cannot end the path on a non-leaf node.
        };
        //walk up the tree, recursing into potentially closer subtrees.
        while let Some((n, ptr_taken)) = path.pop() {
            let optr = if std::ptr::eq(ptr_taken, &n.smaller) {
                &n.bigger
            } else {
                assert!(std::ptr::eq(ptr_taken, &n.bigger));
                &n.smaller
            };
            match optr {
                KdPtr::Empty => {}
                KdPtr::Single(p, t) => {
                    set_closest(&mut closest, *p, (self.metric)(*p, target), *t);
                }
                KdPtr::KdTree(kd_tree) => {
                    //could anything in optr possibly be closer?
                    let closest_plane_point = n.closest_plane_point(target);
                    let distance_to_plane = (self.metric)(target, closest_plane_point);
                    match closest {
                        None => {
                            if let Some((d, p, t)) = kd_tree.find_nearest_to(target, exclude) {
                                set_closest(&mut closest, p, d, t);
                            }
                        }
                        Some((best_d, _, _)) if best_d >= distance_to_plane => {
                            if let Some((d, p, t)) = kd_tree.find_nearest_to(target, exclude) {
                                set_closest(&mut closest, p, d, t);
                            }
                        }
                        _ => {} //too far.
                    }
                }
            }
        }
        closest
    }
    pub fn iter_nearest(&self, target: [i64; K]) -> impl Iterator<Item = (i64, [i64; K], T)> {
        Nearests::new(self, target)
    }
}
pub struct Metrics;
impl Metrics {
    pub fn straight_line_squared<const K: usize>(a: [i64; K], b: [i64; K]) -> i64 {
        (0..K).map(|d| (a[d] - b[d]).abs()).map(|x| x * x).sum()
    }
    pub fn manhattan<const K: usize>(a: [i64; K], b: [i64; K]) -> i64 {
        (0..K).map(|d| (a[d] - b[d]).abs()).sum()
    }
    pub fn area_between<const K: usize>(a: [i64; K], b: [i64; K]) -> i64 {
        (0..K).map(|d| (a[d] - b[d]).abs()).product()
    }
}

#[derive(Debug)]
enum NearestOption<'tree, const K: usize, T> {
    Point {
        distance: i64,
        location: [i64; K],
        label: T,
    },
    Thunk {
        min_distance: i64,
        tree: &'tree KdTree<K, T>,
    },
}
impl<'tree, const K: usize, T> NearestOption<'tree, K, T> {
    fn dist(&self) -> i64 {
        match self {
            NearestOption::Point { distance, .. } => *distance,
            NearestOption::Thunk { min_distance, .. } => *min_distance,
        }
    }
    fn rank(&self) -> usize {
        match self {
            NearestOption::Point { .. } => 0,
            NearestOption::Thunk { tree, .. } => std::ptr::addr_of!(*tree) as usize,
        }
    }
}
pub struct Nearests<'tree, const K: usize, T> {
    base_point: [i64; K],
    stack: BTreeSet<NearestOption<'tree, K, T>>,
    last_returned_dist: i64,
    // kd_tree: &'tree KdTree<K, T>,
    // done_so_far: HashSet<[i64; K]>,
}

impl<'tree, const K: usize, T: PartialEq> std::cmp::PartialEq for NearestOption<'tree, K, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Point {
                    distance: l_distance,
                    location: l_location,
                    label: l_label,
                },
                Self::Point {
                    distance: r_distance,
                    location: r_location,
                    label: r_label,
                },
            ) => l_distance == r_distance && l_location == r_location && l_label == r_label,
            (
                Self::Thunk {
                    min_distance: l_min_distance,
                    tree: l_tree,
                },
                Self::Thunk {
                    min_distance: r_min_distance,
                    tree: r_tree,
                },
            ) => l_min_distance == r_min_distance && std::ptr::eq(l_tree, r_tree),
            _ => false,
        }
    }
}
impl<'tree, const K: usize, T: PartialEq> std::cmp::Eq for NearestOption<'tree, K, T> {}
impl<'tree, const K: usize, T: PartialEq> std::cmp::PartialOrd for NearestOption<'tree, K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'tree, const K: usize, T: PartialEq> std::cmp::Ord for NearestOption<'tree, K, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist()
            .cmp(&other.dist())
            .then(self.rank().cmp(&other.rank()))
    }
}
impl<'tree, const K: usize, T: Copy + PartialEq> Nearests<'tree, K, T> {
    pub fn new(tree: &'tree KdTree<K, T>, point: [i64; K]) -> Self {
        let mut stack = BTreeSet::new();
        stack.insert(NearestOption::Thunk {
            min_distance: 0,
            tree,
        });
        Self {
            base_point: point,
            last_returned_dist: 0,
            stack,
            // kd_tree: tree,
            // done_so_far: [point].into_iter().collect(),
        }
    }
}

impl<'tree, const K: usize, T: std::fmt::Debug + Copy + PartialEq> Iterator
    for Nearests<'tree, K, T>
{
    type Item = (i64, [i64; K], T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop_first() {
                Some(NearestOption::Point {
                    distance,
                    location,
                    label,
                }) => {
                    assert!(distance >= self.last_returned_dist);
                    self.last_returned_dist = distance;
                    return Some((distance, location, label));
                }
                Some(NearestOption::Thunk {
                    min_distance: _,
                    tree,
                }) => {
                    //need to evaluate this path, and put anything found back in the stack,
                    let dist_to_plane =
                        (tree.metric)(tree.closest_plane_point(self.base_point), self.base_point);
                    let (near, far) = if self.base_point[tree.dim] < tree.split {
                        (&tree.smaller, &tree.bigger)
                    } else {
                        (&tree.bigger, &tree.smaller)
                    };
                    //save work for later.
                    for p in [near, far] {
                        match p {
                            KdPtr::Empty => {}
                            KdPtr::Single(p, l) => {
                                self.stack.insert(NearestOption::Point {
                                    distance: (tree.metric)(*p, self.base_point),
                                    location: *p,
                                    label: *l,
                                });
                            }
                            KdPtr::KdTree(kd_tree) => {
                                let min_distance = if std::ptr::eq(p, near) {
                                    0
                                } else {
                                    dist_to_plane
                                };
                                self.stack.insert(NearestOption::Thunk {
                                    min_distance,
                                    tree: kd_tree,
                                });
                            }
                        }
                    }
                }
                None => return None,
            }
        }
        // let ans = self
        //     .kd_tree
        //     .find_nearest_to(self.base_point, &self.done_so_far);
        // if let Some((_, x, _)) = ans {
        //     self.done_so_far.insert(x);
        // }
        // ans
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc};

    use itertools::Itertools;
    use nom::{
        Parser,
        bytes::complete::tag,
        character::complete::{self, newline},
        combinator::all_consuming,
        multi::separated_list1,
    };

    use crate::{
        kdtree::{KdTree, Metrics},
        nom::NomError,
    };

    #[test]
    fn single() {
        let points = vec![([1, 2, 3], 0)];
        let _kd = KdTree::from(&points, Arc::new(Box::new(Metrics::straight_line_squared)));
        // dbg!(kd);
    }
    #[test]
    fn some() {
        let points: Vec<([i64; 2], i64)> = (0..10).map(|x| ([10 - x, x], x)).collect();
        let _kd = KdTree::from(&points, Arc::new(Box::new(Metrics::straight_line_squared)));
        // dbg!(kd);
    }
    #[test]
    fn from_2025d8() {
        const EG: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let coords = all_consuming(separated_list1(
            newline::<_, NomError>,
            separated_list1(tag(","), complete::i64).map(|v| [v[0], v[1], v[2]]),
        ))
        .parse(EG.trim())
        .unwrap()
        .1
        .into_iter()
        .zip(0..)
        .collect_vec();
        let kd = KdTree::from(&coords, Arc::new(Box::new(Metrics::straight_line_squared)));
        let p17 = coords[17].0;
        let p18 = coords[18].0;
        // let exclude: HashSet<[i64; 3]> = [p17].into_iter().collect();
        // let a = kd.find_nearest_to(p17, &exclude).unwrap();
        // assert_eq!(a, (111_326, p18, 18));
        // let exclude: HashSet<[i64; 3]> = [p18].into_iter().collect();
        // let a = kd.find_nearest_to(p18, &exclude).unwrap();
        // assert_eq!(a, (111_326, p17, 17));

        // let a17 = kd.iter_nearest(p17).collect_vec();
        // assert_eq!(a17[1], (111_326, p18, 18));

        let mut i = kd.iter_nearest(p18);
        let _ = i.next().unwrap(); //self.
        let expect17 = i.next().unwrap();
        assert_eq!(expect17, (111_326, p17, 17));
        let a18 = kd.iter_nearest(p18).collect_vec();
        assert_eq!(a18[1], (111_326, p17, 17));
    }
}
