use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
enum KdPtr<const K: usize, T> {
    Empty,
    Single([i64; K], T),
    KdTree(Box<KdTree<K, T>>),
}

#[derive(Debug)]
pub struct KdTree<const K: usize, T> {
    dim: usize,
    split: i64,
    smaller: KdPtr<K, T>,
    bigger: KdPtr<K, T>,
}

impl<T: Copy, const K: usize> KdTree<K, T> {
    pub fn from(points: &[([i64; K], T)]) -> Self {
        Self::from_(points, 0)
    }
    fn from_(points: &[([i64; K], T)], dim: usize) -> Self {
        let sorted: Vec<([i64; K], T)> =
            points.iter().copied().sorted_by_key(|x| x.0[dim]).collect();
        let (smaller_points, bigger_points) = sorted.split_at(sorted.len() / 2);
        let median = bigger_points[0].0[dim];
        let smaller = match smaller_points.len() {
            0 => KdPtr::Empty,
            1 => KdPtr::Single(smaller_points[0].0, smaller_points[0].1),
            _ => KdPtr::KdTree(Box::new(Self::from_(smaller_points, (dim + 1) % K))),
        };
        let bigger = match bigger_points.len() {
            0 => KdPtr::Empty,
            1 => KdPtr::Single(bigger_points[0].0, bigger_points[0].1),
            _ => KdPtr::KdTree(Box::new(Self::from_(bigger_points, (dim + 1) % K))),
        };
        Self {
            dim,
            split: median,
            smaller,
            bigger,
        }
    }
    fn dist2(a: &[i64; K], b: &[i64; K]) -> i64 {
        (0..K).map(|d| (a[d] - b[d]).abs()).map(|x| x * x).sum()
    }
    fn path_to_node_point<'a>(
        &'a self,
        target: [i64; K],
    ) -> Vec<(&'a KdTree<K, T>, &'a KdPtr<K, T>)> {
        let choose_path = |n: &'a KdTree<K, T>| -> &'a KdPtr<K, T> {
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
            KdPtr::Single(p, t) => set_closest(&mut closest, *p, Self::dist2(p, &target), *t),
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
                    set_closest(&mut closest, *p, Self::dist2(&target, p), *t);
                }
                KdPtr::KdTree(kd_tree) => {
                    //could anything in optr possibly be closer?
                    let distance_to_plane = (n.split - target[n.dim]).abs();
                    let dist2 = distance_to_plane * distance_to_plane;
                    match closest {
                        None => {
                            if let Some((d, p, t)) = kd_tree.find_nearest_to(target, exclude) {
                                set_closest(&mut closest, p, d, t);
                            }
                        }
                        Some((best_d, _, _)) if best_d >= dist2 => {
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
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use itertools::Itertools;
    use nom::{
        Parser,
        bytes::complete::tag,
        character::complete::{self, newline},
        combinator::all_consuming,
        multi::separated_list1,
    };

    use crate::{kdtree::KdTree, nom::NomError};

    #[test]
    fn single() {
        let points = vec![([1, 2, 3], 0)];
        let kd = KdTree::from(&points);
        dbg!(kd);
    }
    #[test]
    fn some() {
        let points: Vec<([i64; 2], i64)> = (0..10).map(|x| ([10 - x, x], x)).collect();
        let kd = KdTree::from(&points);
        dbg!(kd);
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
        let kd = KdTree::from(&coords);
        dbg!(&kd);
        let t_point = [862, 61, 35];
        let exclude: HashSet<[i64; 3]> = [t_point].into_iter().collect();
        let a = kd.find_nearest_to(t_point, &exclude).unwrap();
        assert_eq!(a, (111_326, [984, 92, 344], 18));
    }
}
