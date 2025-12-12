use std::fmt::Display;

use itertools::Itertools;
use num::Rational64;

use crate::collections::VecLookup;

#[derive(Debug, Clone)]
pub struct SimplexProb<Label> {
    labels: Vec<Label>,
    le_constraints: Vec<(VecLookup<Rational64>, Rational64)>,
    ge_constraints: Vec<(VecLookup<Rational64>, Rational64)>,
    eq_constraints: Vec<(VecLookup<Rational64>, Rational64)>,
    objective: VecLookup<Rational64>,
    biggest_coefficient: Rational64,
}
#[derive(Debug, Copy, Clone)]
enum VarType {
    //Real var user cares about
    //usize is index into labels.
    Real(usize),
    //slack variable to turn <= into equality.
    //usize is index into constraints.
    Slack(usize),
    //surplus variable to turn >= into equality
    //usize is index into constraints.
    Surplus(usize),
    //artificial variable so we can start with surplus variables at 0.
    //usize is index into constraints.
    Artificial(usize),
}

#[derive(Debug, Clone)]
pub struct SimplexTableau<'prob, L> {
    prob: &'prob SimplexProb<L>,
    grid: Vec<Vec<Rational64>>,
    basis: Vec<VarType>,
    col_names: Vec<VarType>,
}
impl<L: Eq + Copy + Display> SimplexProb<L> {
    pub fn solve_integer(&self, debug: bool) -> Option<Rational64> {
        let mut todo: Vec<Self> = vec![self.clone()];
        let best = self.solve()?.0;
        let best_poss_int = best.floor();
        let mut best_all_int: Option<Rational64> = None;
        if debug {
            println!("Best non-int solution is {best}, so best plausible int is {best_poss_int}");
        }
        while let Some(p) = todo.pop() {
            if debug {
                println!("Solving {p}");
            }
            let t = p.solved_tableau(debug);
            let val = t.objective_value();
            if debug {
                println!("Got objective value {val}");
                for (l, v) in t.get_assigned_values() {
                    print!("{l} = {v}, ");
                }
                println!();
            }
            if !t.check_all_constraints(debug) {
                //this isn't solvable. prune it.
                if debug {
                    println!("Solution not viable, pruning");
                }
            } else if let Some(b) = best_all_int
                && b >= val
            {
                //we can prune this branch, since it is not better than our best known integer solution.
                if debug {
                    println!("Pruning this branch - it's worse than best {b}");
                }
            } else if let Some((l, v)) = t.get_assigned_values().find(|x| !x.1.is_integer()) {
                //v is a non-integer value for l.
                //make two new puzzles - one that says l <= v.floor(), one that says l >= v.ceil().
                if debug {
                    println!(
                        "variable {l} has non-integer value {v}. Splitting on <= {} and >= {}",
                        v.floor(),
                        v.ceil()
                    );
                }
                let mut p1 = p.clone();
                p1.add_le_constraint(&[(1, *l)], v.floor());
                todo.push(p1);
                let mut p2 = p.clone();
                p2.add_ge_constraint(&[(1, *l)], v.ceil());
                todo.push(p2);
            } else {
                if debug {
                    println!("Found all-integer solution {val}");
                    for (l, v) in t.get_assigned_values() {
                        println!("{l} is {v}");
                    }
                    println!("Final tableau:\n{t}");
                }
                if val == best_poss_int {
                    //this is the largest int less than the non-integer max. it's the biggest.
                    if debug {
                        println!("That is best achievable, quitting");
                    }
                    return Some(val);
                }
                //this is an all int solution.

                if let Some(b) = best_all_int {
                    if val > b {
                        best_all_int = Some(val);
                    }
                } else {
                    best_all_int = Some(val);
                }
            }
        }
        best_all_int
    }
}
impl<L: Display> SimplexProb<L> {
    pub fn new() -> Self {
        Self {
            biggest_coefficient: 0.into(),
            labels: Vec::new(),
            le_constraints: Vec::new(),
            ge_constraints: Vec::new(),
            eq_constraints: Vec::new(),
            objective: VecLookup::new(),
        }
    }
    fn big_m(&self) -> Rational64 {
        // self.biggest_coefficient * Rational64::from_integer(1_000_000)
        Rational64::from_integer(1_000_000)
    }
    pub fn solve(&self) -> Option<(Rational64, Vec<(&L, Rational64)>)> {
        let mut tab = self.build_tableau();
        tab.solve(false);
        let val_col = tab.col_names.len();
        let final_val = tab.grid[tab.basis.len()][val_col];
        let vals = tab
            .basis
            .iter()
            .enumerate()
            .filter_map(|(ix, v)| {
                let n = match v {
                    VarType::Real(r) => &self.labels[*r],
                    _ => return None,
                };
                Some((n, tab.grid[ix][val_col]))
            })
            .collect();
        Some((final_val, vals))
    }
    pub fn solved_tableau<'a>(&'a self, debug: bool) -> SimplexTableau<'a, L> {
        let mut t = self.build_tableau();
        t.solve(debug);
        t
    }
    pub fn build_tableau<'a>(&'a self) -> SimplexTableau<'a, L> {
        let big_m: Rational64 = self.big_m();
        let surplus: Vec<VarType> = (0..self.ge_constraints.len())
            .map(VarType::Surplus)
            .collect();
        let artificial: Vec<VarType> = (0..self.ge_constraints.len() + self.eq_constraints.len())
            .map(VarType::Artificial)
            .collect();
        let slack: Vec<VarType> = (0..self.le_constraints.len()).map(VarType::Slack).collect();

        let row_count =
            self.le_constraints.len() + self.ge_constraints.len() + self.eq_constraints.len() + 1;
        let col_count = self.labels.len()
            + self.le_constraints.len()
            + self.eq_constraints.len()
            + self.ge_constraints.len() * 2
            + 1;
        let basis: Vec<VarType> = slack.iter().chain(&artificial).copied().collect();
        let col_names = (0..self.labels.len())
            .map(VarType::Real)
            .chain(slack.iter().copied())
            .chain(surplus.iter().copied())
            .chain(artificial.iter().copied())
            .collect();
        let mut grid: Vec<Vec<Rational64>> = vec![vec![0.into(); col_count]; row_count];

        //fill in rows with values from le_constraints.
        for (ix, (coeffs, rhs)) in self.le_constraints.iter().enumerate() {
            let r = &mut grid[ix];
            for (l_ix, c) in r.iter_mut().enumerate().take(self.labels.len()) {
                *c = coeffs.get(l_ix).copied().unwrap_or_default();
            }
            r[self.labels.len() + ix] = 1.into();
            r[col_count - 1] = *rhs;
        }
        //fill in later rows with values from eq_constraints
        let base = self.le_constraints.len();
        for (ix, (coeffs, rhs)) in self.eq_constraints.iter().enumerate() {
            let r = &mut grid[base + ix];
            for (l_ix, c) in r.iter_mut().enumerate().take(self.labels.len()) {
                *c = coeffs.get(l_ix).copied().unwrap_or_default();
            }
            r[surplus.len() + slack.len() + self.labels.len() + ix] = 1.into();
            r[col_count - 1] = *rhs;
        }
        //fill in later rows with values from ge_constraints
        let base = self.le_constraints.len() + self.eq_constraints.len();
        for (ix, (coeffs, rhs)) in self.ge_constraints.iter().enumerate() {
            let r = &mut grid[base + ix];
            for (l_ix, c) in r.iter_mut().enumerate().take(self.labels.len()) {
                *c = coeffs.get(l_ix).copied().unwrap_or_default();
            }
            r[slack.len() + self.labels.len() + ix] = Rational64::from_integer(-1);
            r[surplus.len() + slack.len() + self.labels.len() + ix] = 1.into();
            r[col_count - 1] = *rhs;
        }

        //fill in objective row.
        let final_row = &mut grid[row_count - 1];
        let w = final_row.len();
        for (ix, (args, rhs)) in self.ge_constraints.iter().enumerate() {
            for (ix, val) in args {
                final_row[ix] -= big_m * val;
            }
            final_row[w - 1] -= rhs * big_m;
            final_row[self.labels.len() + slack.len() + ix] += big_m;
        }
        for (args, rhs) in &self.eq_constraints {
            for (a_ix, val) in args {
                final_row[a_ix] -= big_m * val;
            }
            final_row[w - 1] -= rhs * big_m;
        }
        for (ix, c) in final_row.iter_mut().enumerate().take(self.labels.len()) {
            *c -= self.objective.get(ix).copied().unwrap_or_default();
        }
        SimplexTableau {
            prob: self,
            grid,
            basis,
            col_names,
        }
    }
}
impl<Label: Eq + Copy> SimplexProb<Label> {
    fn try_get_id(&self, label: Label) -> Option<usize> {
        self.labels.iter().position(|l| l == &label)
    }
    fn get_id(&mut self, label: Label) -> usize {
        if let Some(z) = self.try_get_id(label) {
            z
        } else {
            self.labels.push(label);
            self.labels.len() - 1
        }
    }
    fn convert_lhs<N: Into<Rational64> + Copy>(
        &mut self,
        lhs: &[(N, Label)],
    ) -> VecLookup<Rational64> {
        let mut l = VecLookup::default();
        for (coefficient, name) in lhs {
            let id = self.get_id(*name);
            let c = (*coefficient).into();
            *l.entry(id).or_default() += c;
            self.biggest_coefficient = self.biggest_coefficient.max(c);
        }
        l
    }
    pub fn declare_var(&mut self, l: Label) {
        let _ = self.get_id(l);
    }
    pub fn add_eq_constraint<N: Into<Rational64> + Copy, N2: Into<Rational64>>(
        &mut self,
        lhs: &[(N, Label)],
        rhs: N2,
    ) {
        let lhs = self.convert_lhs(lhs);
        self.eq_constraints.push((lhs, rhs.into()));
    }
    pub fn add_le_constraint<N: Into<Rational64> + Copy, N2: Into<Rational64>>(
        &mut self,
        lhs: &[(N, Label)],
        rhs: N2,
    ) {
        let lhs = self.convert_lhs(lhs);
        self.le_constraints.push((lhs, rhs.into()));
    }
    pub fn add_ge_constraint<N: Into<Rational64> + Copy, N2: Into<Rational64>>(
        &mut self,
        lhs: &[(N, Label)],
        rhs: N2,
    ) {
        let l = self.convert_lhs(lhs);
        self.ge_constraints.push((l, rhs.into()));
    }
    pub fn set_objective<N: Into<Rational64> + Copy>(&mut self, val: &[(N, Label)]) {
        self.objective = self.convert_lhs(val);
    }
}

impl<'prob, L: Display> SimplexTableau<'prob, L> {
    pub fn is_solved(&self) -> bool {
        self.select_pivot().is_none()
    }
    pub fn objective_value(&self) -> Rational64 {
        let val_col = self.col_names.len();
        self.grid[self.basis.len()][val_col]
    }

    pub fn get_assigned_values(&self) -> impl Iterator<Item = (&L, Rational64)> {
        let val_col = self.col_names.len();
        self.basis.iter().enumerate().filter_map(move |(ix, v)| {
            let n = match v {
                VarType::Real(r) => &self.prob.labels[*r],
                _ => return None,
            };
            Some((n, self.grid[ix][val_col]))
        })
    }
    pub fn solve(&mut self, debug: bool) {
        let mut c = 0;
        while let Some(p) = self.select_pivot() {
            c += 1;
            if debug {
                println!("Tableau:\n{self}");
            }
            if c > 100 {
                panic!(">100 tableau iterations");
            }
            self.execute_pivot(p);
        }
    }
    pub fn select_pivot(&self) -> Option<(usize, usize)> {
        //col is most negative in last row.
        let val_col = self.grid[0].len() - 1;
        let (&min, col_ix) = self.grid[self.grid.len() - 1]
            .iter()
            .zip(0..val_col)
            .min()?;
        if min >= 0.into() {
            //nothing negative, nothing to do.
            return None;
        }
        //now pick row. we want the smallest, positive value of theta, which is row value / value in col_ix.
        let (_theta, row_ix) = (0..self.grid.len() - 1)
            .map(|r| (self.grid[r][val_col], self.grid[r][col_ix], r))
            .filter(|x| x.1 > 0.into())
            .map(|(a, b, ix)| (a / b, ix))
            .filter(|x| x.0 >= 0.into())
            .min()?;

        Some((row_ix, col_ix))
    }
}

impl<Label: Display> Default for SimplexProb<Label> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Display> SimplexProb<Label> {
    fn render_coefficient(&self, coefficient: Rational64, label_ix: usize) -> String {
        let label = &self.labels[label_ix];
        if coefficient == Rational64::ONE {
            format!("{}", label)
        } else {
            format!("{}{}", coefficient, label)
        }
    }
    fn render_coefficients(&self, v: &VecLookup<Rational64>) -> String {
        v.iter()
            .map(|(a, b)| self.render_coefficient(*b, a))
            .join(" + ")
    }
}
impl<'prob, Label: Display> SimplexTableau<'prob, Label> {
    fn render_var(&self, v: &VarType) -> String {
        match v {
            VarType::Real(a) => self.prob.labels[*a].to_string(),
            VarType::Slack(a) => format!("s{a}"),
            VarType::Surplus(a) => format!("S{a}"),
            VarType::Artificial(a) => format!("a{a}"),
        }
    }
}
impl<'prob, Label: Display + Eq> SimplexTableau<'prob, Label> {
    pub fn check_all_constraints(&self, debug: bool) -> bool {
        let vals: Vec<(&Label, Rational64)> = self.get_assigned_values().collect();
        let mut ok = true;
        for (l, r) in &self.prob.ge_constraints {
            let lhs: Vec<Rational64> = l
                .iter()
                .map(|(label, v)| {
                    vals.iter()
                        .find(|(a, _)| *a == &self.prob.labels[label])
                        .map(|x| x.1)
                        .unwrap_or_default()
                        * v
                })
                .collect();
            let total: Rational64 = lhs.iter().sum();
            if total < *r {
                ok = false;
                if debug {
                    println!(
                        "Constraint violated:\n{} >= {}\n{:?} == {}",
                        self.prob.render_coefficients(l),
                        r,
                        lhs,
                        total
                    );
                } else {
                    return ok;
                }
            }
        }
        for (l, r) in &self.prob.le_constraints {
            let lhs: Vec<Rational64> = l
                .iter()
                .map(|(label, v)| {
                    vals.iter()
                        .find(|(a, _)| *a == &self.prob.labels[label])
                        .map(|x| x.1)
                        .unwrap_or_default()
                        * v
                })
                .collect();
            let total: Rational64 = lhs.iter().sum();
            if total > *r {
                ok = false;
                if debug {
                    println!(
                        "Constraint violated:\n{} <= {}\n{:?} == {}",
                        self.prob.render_coefficients(l),
                        r,
                        lhs,
                        total
                    );
                } else {
                    return ok;
                }
            }
        }
        for (l, r) in &self.prob.eq_constraints {
            let lhs: Vec<Rational64> = l
                .iter()
                .map(|(label, v)| {
                    vals.iter()
                        .find(|(a, _)| *a == &self.prob.labels[label])
                        .map(|x| x.1)
                        .unwrap_or_default()
                        * v
                })
                .collect();
            let total: Rational64 = lhs.iter().sum();
            if total != *r {
                ok = false;
                if debug {
                    println!(
                        "Constraint violated:\n{} == {}\n{:?} == {}",
                        self.prob.render_coefficients(l),
                        r,
                        lhs,
                        total
                    );
                } else {
                    return ok;
                }
            }
        }
        ok
    }
}
impl<'prob, Label> SimplexTableau<'prob, Label> {
    pub fn execute_pivot(&mut self, pivot: (usize, usize)) {
        let pivot_row = pivot.0;
        let pivot_col = pivot.1;
        self.basis[pivot_row] = self.col_names[pivot_col];
        let pivot_val = self.grid[pivot_row][pivot_col];
        //divide pivot row by pivot_val.
        for v in &mut self.grid[pivot_row] {
            *v /= pivot_val;
        }
        let width = self.grid[0].len();

        //subtract off multiples of pivot row from all other rows.
        for row_ix in 0..self.grid.len() {
            if row_ix != pivot_row {
                let row_pivot_col_val = self.grid[row_ix][pivot_col];
                if row_pivot_col_val != 0.into() {
                    let coeff = row_pivot_col_val / self.grid[pivot_row][pivot_col];
                    for ix in 0..width {
                        let pivot_row_val = self.grid[pivot_row][ix];
                        self.grid[row_ix][ix] -= coeff * pivot_row_val;
                    }
                }
            }
        }
    }
}

impl<L: Display> std::fmt::Display for SimplexProb<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obj = self.render_coefficients(&self.objective);
        let le_cons = self
            .le_constraints
            .iter()
            .map(|(a, b)| format!("{} <= {}", self.render_coefficients(a), b));
        let ge_cons = self
            .ge_constraints
            .iter()
            .map(|(a, b)| format!("{} >= {}", self.render_coefficients(a), b));
        let eq_cons = self
            .eq_constraints
            .iter()
            .map(|(a, b)| format!("{} = {}", self.render_coefficients(a), b));
        let cons = le_cons
            .into_iter()
            .chain(ge_cons)
            .chain(eq_cons)
            .join("\n  ");
        f.write_fmt(format_args!(
            "Simplex Problem:\nMaximise {obj}\nUnder constraints:\n  {cons}"
        ))?;
        Ok(())
    }
}
impl<'prob, L: Display> std::fmt::Display for SimplexTableau<'prob, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self.select_pivot();
        let p_row = p.map(|x| x.0 + 1);
        let p_col = p.map(|x| x.1 + 1);
        let w = self.grid[0].len() + 1;
        let mut full_grid = vec![vec![String::new(); w]; self.grid.len() + 1];
        //fill col titles.
        for (ix, v) in self.col_names.iter().enumerate() {
            full_grid[0][ix + 1] = self.render_var(v);
        }
        full_grid[0][w - 1] = "Value".to_string();
        full_grid[0][0] = "Basic".to_string();
        for (ix, v) in self.basis.iter().enumerate() {
            full_grid[ix + 1][0] = self.render_var(v);
        }
        for (row_ix, row) in self.grid.iter().enumerate() {
            for (col_ix, cell) in row.iter().enumerate() {
                full_grid[row_ix + 1][col_ix + 1] = cell.to_string();
            }
        }
        let mut col_widths = vec![0; w];

        for r in &full_grid {
            for (col_ix, c) in r.iter().enumerate() {
                col_widths[col_ix] = col_widths[col_ix].max(c.len());
            }
        }
        let total_wid = col_widths.iter().map(|x| x + 1).sum();

        // if let Some((row, col)) = self.select_pivot() {
        //     let green = ansiterm::Colour::Green;
        //     let white = ansiterm::Colour::White;
        //     let gow = white.on(green);
        //     for c in &mut full_grid[row+1] {
        //         let painted = gow.paint(&*c).to_string();
        //         *c = painted;
        //     }
        //     for r_ix in 0..full_grid.len() {
        //         let c = &mut full_grid[r_ix][col];
        //         *c = gow.paint(&*c).to_string();
        //     }
        // }

        let highlight = ansiterm::Colour::White.on(ansiterm::Colour::Green);
        for (row_ix, row) in full_grid.into_iter().enumerate() {
            for (col_ix, col) in row.iter().enumerate() {
                let width = col_widths[col_ix];
                if Some(row_ix) == p_row || Some(col_ix) == p_col {
                    let padded = format!("{col:^width$}");
                    f.write_fmt(format_args!("{}|", highlight.paint(padded)))?;
                } else {
                    f.write_fmt(format_args!("{col:^width$}|"))?;
                }
            }
            if row_ix == 0 || row_ix == self.grid.len() {
                let pad = "";
                f.write_fmt(format_args!("\n{pad:=^total_wid$}"))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::linear_programming::SimplexProb;

    #[test]
    fn example() {
        let mut p = SimplexProb::new();
        p.set_objective(&[(3, "x"), (4, "y"), (5, "z")]);
        p.add_le_constraint(&[(2, "x"), (1, "y")], 10);
        p.add_le_constraint(&[(2, "y"), (1, "z")], 20);
        p.add_le_constraint(&[(2, "z"), (1, "x")], 30);
        let as_str = p.to_string();
        assert_eq!(
            as_str,
            "Simplex Problem:
Maximise 3x + 4y + 5z
Under constraints:
  2x + y <= 10
  2y + z <= 20
  x + 2z <= 30"
        );
        let tab1 = p.build_tableau();
        println!("{tab1}");
        assert_eq!(tab1.select_pivot(), Some((2, 2)));
        let s = p.solve().unwrap();
        assert_eq!(s.0, 90.into());
    }
    #[test]
    fn example_with_ge() {
        let mut p = SimplexProb::new();
        p.set_objective(&[(2, "x"), (1, "y")]);
        p.add_le_constraint(&[(1, "x"), (-1, "y")], 11);
        p.add_ge_constraint(&[(1, "x"), (3, "y")], 15);
        let s = p.solve().unwrap();
        assert_eq!(s.0, 25.into());
    }
    #[test]
    fn another_ge_example() {
        let mut p = SimplexProb::new();
        p.set_objective(&[(-6, "x"), (-3, "y")]);
        p.add_ge_constraint(&[(1, "x"), (1, "y")], 1);
        p.add_ge_constraint(&[(2, "x"), (-1, "y")], 1);
        p.add_le_constraint(&[(3, "y")], 2);
        let s = p.solve().unwrap();
        assert_eq!(s.0, (-5).into());
    }
}
