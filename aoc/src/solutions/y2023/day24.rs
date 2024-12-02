use itertools::Itertools;
use nalgebra::SVector;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;
use num::rational::Ratio;
use num_traits::{CheckedDiv, Zero};
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 24, part1 [p1::<200_000_000_000_000, 400_000_000_000_000>], part2 [p2]/* , example part2 EG => 47*/);

type MyNum = Ratio<i128>;

#[derive(Copy, Clone, Debug)]
struct Hailstone {
    pos: SVector<MyNum, 3>,
    vel: SVector<MyNum, 3>,
}

impl Hailstone {
    fn parse(input: &str) -> IResult<Self> {
        let (input, (pos, vel)) = separated_pair(
            separated_list1(tag(",").terminated(space1), complete::i128),
            space1.terminated(tag("@")).terminated(space1),
            separated_list1(tag(",").terminated(space1), complete::i128),
        )(input)?;
        let pos = SVector::from_fn(|n, _| MyNum::from(pos[n]));
        let vel = SVector::from_fn(|n, _| MyNum::from(vel[n]));
        Ok((input, Self { pos, vel }))
    }
    fn time_when_at_2d(&self, (x, y): (MyNum, MyNum)) -> MyNum {
        //at what time is this hailstone at x,y?
        let by_x = (x - self.pos[0]) / self.vel[0];
        let by_y = (y - self.pos[1]) / self.vel[1];
        assert_eq!(by_x, by_y, "self: {self:?}, coords: {x}, {y}");
        by_x
    }

    fn linear_form_2d(&self) -> (MyNum, MyNum) {
        //don't care about time here, just need the expression of the line
        //pos_t = self.pos + self.vel * t.
        //x = t.vel[0] + pos[0]
        //y = t.vel[1] + pos[1]
        //y = mx + c.
        let m = self.vel[1] / self.vel[0];
        let c = self.pos[1] - self.pos[0] * m;
        (m, c)
    }
}

fn find_cross_x(a: (MyNum, MyNum), b: (MyNum, MyNum)) -> Option<(MyNum, MyNum)> {
    let x = (b.1 - a.1).checked_div(&(a.0 - b.0))?;
    let y = a.0 * x + a.1;
    Some((x, y))
}
fn p1<const MIN: i128, const MAX: i128>(input: &str) -> u32 {
    let puzzle = separated_list1(newline, Hailstone::parse)
        .terminated(newline)
        .all_consuming()
        .complete()
        .parse(input)
        .expect("parse")
        .1;
    let mut count = 0;
    for cs in puzzle.iter().combinations(2) {
        let a = cs[0];
        let b = cs[1];

        // println!(
        //     "Hailstone A: {}, {}, {} @ {}, {}, {}",
        //     a.pos[0], a.pos[1], a.pos[2], a.vel[0], a.vel[1], a.vel[2]
        // );
        // println!(
        //     "Hailstone B: {}, {}, {} @ {}, {}, {}",
        //     b.pos[0], b.pos[1], b.pos[2], b.vel[0], b.vel[1], b.vel[2]
        // );

        let eq1 = a.linear_form_2d();
        let eq2 = b.linear_form_2d();
        if let Some(cross) = find_cross_x(eq1, eq2) {
            let t_a = a.time_when_at_2d(cross);
            let t_b = b.time_when_at_2d(cross);
            if t_a < MyNum::zero() && t_b < MyNum::zero() {
                // println!("Hailstone's paths crossed in the past for both hailstones.");
            } else if t_a < MyNum::zero() {
                // println!("Hailstone's paths crossed in the past for hailstone A.");
            } else if t_b < MyNum::zero() {
                // println!("Hailstone's paths crossed in the past for hailstone B.");
            } else {
                let x_in = cross.0 >= MIN.into() && cross.0 <= MAX.into();
                let y_in = cross.1 >= MIN.into() && cross.1 <= MAX.into();
                if x_in && y_in {
                    count += 1;
                    // println!("Hailstones' paths will cross inside the test area (at x={:.3}, y={:.3})",cross.0.to_f64().unwrap(), cross.1.to_f64().unwrap());
                } else {
                    // println!("Hailstones' paths will cross outside the test area (at x={:.3}, y={:.3})",cross.0.to_f64().unwrap(), cross.1.to_f64().unwrap());
                }
            }
        } else {
            // println!("Hailstone's parths are parallel; they never intersect.");
        }

        //when does the 2 paths cross?
        //l1: y=mt+c.
        //a =
        // println!();
    }
    count
}

// struct Hailstones {
//     stones: [Hailstone; 3],
// }
// impl Problem for Hailstones {
//     type Field = f64;

//     fn domain(&self) -> gomez::Domain<Self::Field> {
//         gomez::Domain::unconstrained(9)
//     }
// }
// impl System for Hailstones {
//     fn eval<Sx, Srx>(
//         &self,
//         x: &nalgebra::Vector<Self::Field, nalgebra::Dyn, Sx>,
//         rx: &mut nalgebra::Vector<Self::Field, nalgebra::Dyn, Srx>,
//     ) where
//         Sx: nalgebra::Storage<Self::Field, nalgebra::Dyn> + nalgebra::IsContiguous,
//         Srx: nalgebra::StorageMut<Self::Field, nalgebra::Dyn>,
//     {
//         // vx*t0 + px = h[0].vel[0] * t0 + h[0].pos[0]
//         // vy*t0 + py = h[0].vel[1] * t0 + h[0].pos[1]
//         // vz*t0 + pz = h[0].vel[2] * t0 + h[0].pos[2]

//         // vx*t1 + px = h[1].vel[0] * t1 + h[1].pos[0]
//         // vy*t1 + py = h[1].vel[1] * t1 + h[1].pos[1]
//         // vz*t1 + pz = h[1].vel[2] * t1 + h[1].pos[2]

//         // vx*t2 + px = h[2].vel[0] * t2 + h[2].pos[0]
//         // vy*t2 + py = h[2].vel[1] * t2 + h[2].pos[1]
//         // vz*t2 + pz = h[2].vel[2] * t2 + h[2].pos[2]
//         let h = &self.stones;
//         // let [p0,p1,p2,v0,v1,v2,t0,t1,t2] = &x;
//         for h_ix in 0..3 {
//             rx[3 * h_ix + 0] = h[h_ix].vel[0].to_f64().unwrap() * x[6] + h[h_ix].pos[0].to_f64().unwrap() - x[3] * x[6] - x[0];
//             rx[3 * h_ix + 1] = h[h_ix].vel[1].to_f64().unwrap() * x[7] + h[h_ix].pos[1].to_f64().unwrap() - x[4] * x[7] - x[1];
//             rx[3 * h_ix + 2] = h[h_ix].vel[2].to_f64().unwrap() * x[8] + h[h_ix].pos[2].to_f64().unwrap() - x[5] * x[8] - x[2];
//         }
//     }
// }
fn p2(input: &str) -> usize {
    let puzzle = separated_list1(newline, Hailstone::parse)
        .terminated(newline)
        .all_consuming()
        .complete()
        .parse(input)
        .expect("parse")
        .1;
    //need to find v[0,1,2] and p[0,1,2] and times [t0,t1,...tN] such that
    //we collide.
    //we can get 3 equations from each hailstone
    //so for N = 1, we have 3 equations and 7 unknowns.
    // N =2 , 6 eqns, 8 unknowns.
    //N=3, 9 eqns, 9 unknowns.
    //so we only need the first 3 hailstones.

    // vx*t0 + px = h[0].vel[0] * t0 + h[0].pos[0]
    // vy*t0 + py = h[0].vel[1] * t0 + h[0].pos[1]
    // vz*t0 + pz = h[0].vel[2] * t0 + h[0].pos[2]

    // vx*t1 + px = h[1].vel[0] * t1 + h[1].pos[0]
    // vy*t1 + py = h[1].vel[1] * t1 + h[1].pos[1]
    // vz*t1 + pz = h[1].vel[2] * t1 + h[1].pos[2]

    // vx*t2 + px = h[2].vel[0] * t2 + h[2].pos[0]
    // vy*t2 + py = h[2].vel[1] * t2 + h[2].pos[1]
    // vz*t2 + pz = h[2].vel[2] * t2 + h[2].pos[2]
//     let smol = Hailstones{
//         stones: [
//             puzzle[0], puzzle[1],puzzle[2]
//         ]
//     };
//     let mut solver = SolverDriver::builder(&smol)
//     .with_initial(vec![0.;9])
//     .build();
// let (x, norm) = solver.find(|s| s.norm() <= 1e-6 || s.iter() >= 100)
// .expect("solver err");
// dbg!(x, norm);

    for (ix, h) in puzzle.iter().enumerate().take(3) {
        println!("pos{}, v{} = ([{},{},{}], [{},{},{}])", ix+1, ix+1, 
            h.pos[0].to_integer(),
            h.pos[1].to_integer(),
            h.pos[2].to_integer(),
            h.vel[0].to_integer(),
            h.vel[1].to_integer(),
            h.vel[2].to_integer(),
        );
    }
    0 //todo
}

#[cfg(test)]
mod test {
    use super::*;

const EG: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
    #[test]
    fn eg() {
        assert_eq!(p1::<7, 27>(EG), 2);
    }
}
