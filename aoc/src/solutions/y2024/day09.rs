use utils::collections::VecLookup;

aoc_harness::aoc_main!(2024 day 9, part1 [p1] => 6_334_655_979_668, part2 [p2] => 6_349_492_251_099, example both EG => (1928, 2858));

#[allow(dead_code)]
fn print_disk(disk: &[Option<usize>]) {
    for c in disk {
        print!("{}", c.map(|x| x.to_string()).unwrap_or(".".to_string()));
    }
    println!();
}

fn p1(input: &str) -> usize {
    let mut disk = Vec::new();
    for (index, v) in input.trim().as_bytes().chunks(2).enumerate() {
        let file_size = v[0] - b'0';
        disk.extend(std::iter::repeat(Some(index)).take(file_size.into()));
        if let Some(&x) = v.get(1) {
            disk.extend(std::iter::repeat(None).take((x - b'0').into()));
        }
    }
    let mut gap = disk.iter().position(Option::is_none);
    while let Some(g) = gap {
        disk[g] = disk.pop().unwrap();
        gap = (g..disk.len()).find(|ix| disk[*ix].is_none());
        // print_disk(&disk);
        // println!("Gap now at {gap:?}");
    }

    disk.iter().enumerate().map(|x| x.0 * x.1.unwrap()).sum()
}
struct DiskSection {
    contiguous_files: Vec<(usize, usize)>,
    space: usize,
    moved_first_file: bool,
}
impl DiskSection {
    fn new(v: &[u8], id: usize) -> Self {
        let size = (v[0] - b'0') as usize;
        let space = v.get(1).map(|x| (x - b'0') as usize).unwrap_or_default();
        let mut contiguous_files = Vec::with_capacity(9);
        contiguous_files.push((id,size));
        Self {
            contiguous_files,
            space,
            moved_first_file: false,
        }
    }

    fn checksum(&self, mut index: usize) -> (usize, usize) {
        let mut ans = 0;
        let start_ix = if self.moved_first_file { index += self.contiguous_files[0].1; 1} else {0};
        for &(id, size) in &self.contiguous_files[start_ix..] {
            ans += id * ((size * index) + (size *(size-1)/2));
            index += size;
        }
        index += self.space;
        (ans, index)
    }
}
fn find_gap(disk: &[DiskSection], size: usize, start_at: usize, stop_at: usize) -> Option<usize> {
    (start_at..stop_at).find(|ix| disk[*ix].space >= size)
}
fn p2(input: &str) -> usize {
    let mut disk: Vec<DiskSection> = Vec::new();
    let mut index = 0;
    for v in input.trim().as_bytes().chunks(2) {
        disk.push(DiskSection::new(v, index));
        index += 1;
    }
    //map of gaps of at least X to vec of disk section indexes.
    let mut first_gap_of_size_n_or_greater : VecLookup<usize> = VecLookup::default();
    for size in 0..10 {
        if let Some(g) = find_gap(&disk, size, 0, disk.len()) {
            first_gap_of_size_n_or_greater.insert(size,g);
        } 
    }

    //now have a disk with N sections, each of which starts with an incrementing Id.
    for ix in (1..index).rev() {
        let size = disk[ix].contiguous_files[0].1;
        assert_eq!(disk[ix].contiguous_files[0].0, ix);
        //find a spot for it and remove it.
        let qni = first_gap_of_size_n_or_greater.get(size).copied().filter(|&x| x<ix);
        if let Some(new_ix) = qni {
            disk[new_ix].contiguous_files.push((ix, size));
            let old_space = disk[new_ix].space;
            disk[new_ix].space -= size;
            let new_space = disk[new_ix].space;
            //size at new_ix has reduced from old_space down to new_space. 
            //need to recalculate the lookup for those sizes, but they can't be left of where they were, or beyond ix..
            for size in new_space+1..=old_space {
                match first_gap_of_size_n_or_greater.get(size) {
                    Some(&i) if i == new_ix => {
                        if let Some(g) = find_gap(&disk, size, new_ix, ix) {
                            first_gap_of_size_n_or_greater.insert(size, g);
                        } else {
                            first_gap_of_size_n_or_greater.remove(size);
                        }
                    }
                    _ => (),
                }
            }
            disk[ix].moved_first_file = true;
        }
    }
    let mut checksum = 0;
    let mut index = 0;
    for s in &disk {
        let (c, i) = s.checksum(index);
        index = i;
        checksum += c;
    }
    checksum
}

const EG: &str = "2333133121414131402";
