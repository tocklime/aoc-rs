use aoc_harness::*;
use nalgebra::{SMatrix, SVector};
use utils::nums::exp_by_squares;

aoc_main!(2021 day 6, generator input::<usize, ','>, 
          both [solve] => (385_391,1_728_611_055_389), 
          part1 [matrices::<80>] => 385_391, part2 [matrices::<256>] =>1_728_611_055_389,
          example both EG => (5934, 26_984_457_539_usize));

fn solve(input: &[usize]) -> (usize, usize) {
    let mut counts = [0; 9];
    input.iter().for_each(|&x| counts[x] += 1);
    for d in 0..80 {
        counts[(d + 7) % 9] += counts[(d % 9)];
    }
    let p1 = counts.iter().sum();
    for d in 80..256 {
        counts[(d + 7) % 9] += counts[(d % 9)];
    }
    (p1, counts.iter().sum())
}
fn matrices<const GENERATIONS: usize>(input: &[usize]) -> usize {
    let mut counts: SVector<usize, 9> = SVector::from_element(0);
    input.iter().for_each(|&x| counts[x] += 1);
    let mut matrix: SMatrix<usize, 9, 9> = SMatrix::from_element(0);
    for x in 0..9 {
        matrix[(x, (x + 1) % 9)] = 1;
    }
    matrix[(6, 0)] = 1;

    // Matrix is now this:
    //   ┌                   ┐
    //   │ 0 1 0 0 0 0 0 0 0 │ //New first elem of array is the second
    //   │ 0 0 1 0 0 0 0 0 0 │ //New second elem is the third...
    //   │ 0 0 0 1 0 0 0 0 0 │ //...
    //   │ 0 0 0 0 1 0 0 0 0 │ //...
    //   │ 0 0 0 0 0 1 0 0 0 │ //...
    //   │ 0 0 0 0 0 0 1 0 0 │ //...
    //   │ 1 0 0 0 0 0 0 1 0 │ //New seventh elem of array is first + eigth.
    //   │ 0 0 0 0 0 0 0 0 1 │ //...
    //   │ 1 0 0 0 0 0 0 0 0 │ //New ninth elem of array is the first.
    //   └                   ┘

    let mat_pow = exp_by_squares(matrix, GENERATIONS - 1);
    //   For GENERATIONS = 80, mat_pow is now this:
    //   ┌                                     ┐
    //   │ 252  20 210  37 120  84  45 126  11 │ //new first elem of array is 252*first + 20*second + 210*third+...
    //   │  56 252  20 210  37 120  84  45 126 │
    //   │ 210  56 252  20 210  37 120  84  45 │
    //   │ 165 210  56 252  20 210  37 120  84 │
    //   │ 121 165 210  56 252  20 210  37 120 │
    //   │ 330 121 165 210  56 252  20 210  37 │
    //   │  57 330 121 165 210  56 252  20 210 │
    //   │ 210  37 120  84  45 126  11 126   9 │
    //   │  20 210  37 120  84  45 126  11 126 │
    //   └                                     ┘
    let ans = mat_pow * counts;
    ans.iter().map(|x| *x as usize).sum()
}
const EG: &str = "3,4,3,1,2";
