use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_harness::*;
use itertools::chain;
use lazy_static::lazy_static;
use nalgebra::{Matrix4, Vector4};
use utils::imap::IMap;

aoc_main!(2021 day 19, generator whole_input_is::<Day19>, both [p1] => (419,13210), example both EG => (79,3621));

#[derive(Debug)]
struct Day19 {
    scanner_readings: Vec<Vec<Vector4<isize>>>,
}

impl FromStr for Day19 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanner_readings = s
            .split("\n\n")
            .map(|scanner| {
                scanner
                    .lines()
                    .skip(1)
                    .map(|rel_beacon| {
                        let vals = rel_beacon
                            .split(',')
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<isize>>();
                        Vector4::new(vals[0], vals[1], vals[2], 1)
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Self { scanner_readings })
    }
}
lazy_static! {
    static ref ALL_ROTATIONS: Vec<Matrix4<isize>> = all_rotation_matrices();
}

fn all_rotation_matrices() -> Vec<Matrix4<isize>> {
    let sin = [0, 1, 0, -1];
    let cos = [1, 0, -1, 0];
    let mut ans = HashSet::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                let m = Matrix4::from_iterator([
                    cos[a] * cos[b],
                    cos[a] * sin[b] * sin[c] - sin[a] * cos[c],
                    cos[a] * sin[b] * cos[c] + sin[a] * sin[c],
                    0,
                    sin[a] * cos[b],
                    sin[a] * sin[b] * sin[c] + cos[a] * cos[c],
                    sin[a] * sin[b] * cos[c] - cos[a] * sin[c],
                    0,
                    -sin[b],
                    cos[b] * sin[c],
                    cos[b] * cos[c],
                    0,
                    0,
                    0,
                    0,
                    1,
                ]);
                ans.insert(m);
            }
        }
    }
    ans.into_iter().collect()
}

struct PlacedScanners {
    scanners: Vec<Matrix4<isize>>,
    beacons: HashSet<Vector4<isize>>,
}
const MATCH_REQUIREMENT: usize = 12;
impl PlacedScanners {
    fn matches_enough(&self, locations: impl Iterator<Item = Vector4<isize>>) -> bool {
        let mut count = 0;
        for x in locations {
            if self.beacons.contains(&x) {
                count += 1;
                if count == 12 {
                    return true;
                }
            }
        }
        false
    }
    fn try_insert(&mut self, locations: &[Vector4<isize>]) -> bool {
        let match_count = locations
            .iter()
            .filter(|x| self.beacons.contains(x))
            .count();
        if match_count >= MATCH_REQUIREMENT {
            self.beacons.extend(locations);
            true
        } else {
            false
        }
    }
    fn find_biggest_distance_between_scanners(&self) -> isize {
        self.scanners
            .iter()
            .combinations(2)
            .map(|v| {
                (0..3)
                    .map(|d| max(v[0][(d, 3)], v[1][(d, 3)]) - min(v[0][(d, 3)], v[1][(d, 3)]))
                    .sum()
            })
            .max()
            .unwrap()
    }
    #[allow(dead_code)]
    fn try_find_mat_slow(&self, beacons: &[Vector4<isize>]) -> Option<Matrix4<isize>> {
        //hashmap of (rotation_ix, difference) to count of instances.
        let mut counts: Vec<HashMap<Vector4<isize>, usize>> = vec![HashMap::new(); 24];
        for my_point in beacons {
            for (r_ix, rotation) in ALL_ROTATIONS.iter().enumerate() {
                let r = rotation * my_point;
                for solid_point in &self.beacons {
                    let d = solid_point - r;
                    let entry = counts[r_ix].entry(d).or_default();
                    *entry += 1;
                    if *entry >= 12 {
                        let mut ans = *rotation;
                        ans[(0, 3)] = d[0];
                        ans[(1, 3)] = d[1];
                        ans[(2, 3)] = d[2];
                        return Some(ans);
                    }
                }
            }
        }
        None
    }
    fn suggest_dimension_translations(
        &self,
        dim: usize,
        beacons: &[Vector4<isize>],
    ) -> Vec<([isize; 3], isize)> {
        //hashmap of (rotation_ix, difference) to count of instances.
        let mut counts: Vec<IMap<usize>> = vec![IMap::with_elem(3000, 0); ALL_DIM_FLIPS.len()];
        let mut ans = Vec::new();
        for my_point in beacons {
            for (flip_ix, flip) in ALL_DIM_FLIPS.iter().enumerate() {
                let me: isize = (0..3).map(|x| my_point[x] * flip[x]).sum();
                for solid_point in &self.beacons {
                    let diff = solid_point[dim] - me;
                    {
                        let entry = &mut counts[flip_ix][diff];
                        *entry += 1;
                        if *entry == 12 {
                            ans.push((*flip, diff));
                        }
                    }
                }
            }
        }
        ans
    }
    fn try_find_mat_dimensionwise(&self, beacons: &[Vector4<isize>]) -> Option<Matrix4<isize>> {
        let suggestions = (0..3).map(|d| self.suggest_dimension_translations(d, beacons));
        // println!("s counts: {:?}", suggestions.iter().map(std::vec::Vec::len).collect_vec());
        for s in suggestions.multi_cartesian_product() {
            //1: do the flips match?
            let dims = s
                .iter()
                .map(|(f, _)| f.iter().position(|&x| x != 0).unwrap())
                .sorted()
                .collect_vec();
            if dims == [0, 1, 2] {
                //2. does the thing match enough?
                let mat = chain!(
                    s[0].0,
                    [s[0].1],
                    s[1].0,
                    [s[1].1],
                    s[2].0,
                    [s[2].1],
                    [0, 0, 0, 1]
                );
                let matrix = Matrix4::from_iterator(mat).transpose();
                // println!("Mat: {}", matrix);
                if self.matches_enough(beacons.iter().map(|b| matrix * b)) {
                    return Some(matrix);
                }
            }
        }
        None
    }
}

const ALL_DIM_FLIPS: [[isize; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn p1(input: &Day19) -> (usize, isize) {
    let mut i = input.scanner_readings.iter();
    let first = i.next().unwrap();
    let mut placed = PlacedScanners {
        scanners: vec![Matrix4::identity()],
        beacons: first.iter().copied().collect(),
    };
    let mut to_place = i.collect_vec();
    while !to_place.is_empty() {
        let mut try_later = Vec::new();
        let len_before = to_place.len();
        for scanner in to_place {
            let answer = placed.try_find_mat_dimensionwise(scanner);
            if let Some(answer) = answer {
                placed.scanners.push(answer);
                let in_absolute = scanner.iter().map(|b| answer * b).collect_vec();
                if !placed.try_insert(&in_absolute) {
                    try_later.push(scanner);
                }
            } else {
                try_later.push(scanner);
            }
        }
        to_place = try_later;
        assert!(to_place.len() < len_before);
    }
    (
        placed.beacons.len(),
        placed.find_biggest_distance_between_scanners(),
    )
}

const EG: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
