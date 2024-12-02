use std::fmt::Display;

use quote::quote;

use proc_macro2::TokenStream;
use syn::bracketed;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::ExprLit;
use syn::LitInt;
use syn::Result;
use syn::Token;

pub mod all;
pub mod latest;

mod kw {
    syn::custom_keyword!(generator);
    syn::custom_keyword!(day);
    syn::custom_keyword!(bench);
    syn::custom_keyword!(example);
    syn::custom_keyword!(part1);
    syn::custom_keyword!(part2);
    syn::custom_keyword!(both);
}
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum PartNum {
    Part1,
    Part2,
    Both,
}
enum ExpectedResult {
    None,
    Single(Expr),
    Double(Expr, Expr),
}

impl Display for PartNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartNum::Part1 => f.write_str("Part 1"),
            PartNum::Part2 => f.write_str("Part 2"),
            PartNum::Both => f.write_str("Both parts"),
        }
    }
}

impl Parse for PartNum {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::part1) {
            let _: kw::part1 = input.parse()?;
            Ok(PartNum::Part1)
        } else if lookahead.peek(kw::part2) {
            let _: kw::part2 = input.parse()?;
            Ok(PartNum::Part2)
        } else if lookahead.peek(kw::both) {
            let _: kw::both = input.parse()?;
            Ok(PartNum::Both)
        } else {
            Err(lookahead.error())
        }
    }
}

struct ExamplePart {
    part_num: PartNum,
    str_input: Expr,
    ans: ExpectedResult,
}
impl Parse for ExamplePart {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ex_token: kw::example = input.parse()?;
        let part_num: PartNum = input.parse()?;
        let str_input: Expr = input.parse()?;
        let _goes_to = input.parse::<Token![=>]>()?;
        let ans = match part_num {
            PartNum::Part1 | PartNum::Part2 => ExpectedResult::Single(input.parse()?),
            PartNum::Both => {
                let content;
                let _parens = parenthesized!(content in input);
                let a = content.parse::<Expr>()?;
                let _comma = content.parse::<Token![,]>()?;
                let b = content.parse::<Expr>()?;
                ExpectedResult::Double(a, b)
            }
        };
        Ok(Self {
            part_num,
            str_input,
            ans,
        })
    }
}
struct BenchPart {
    _bench_token: kw::bench,
}
impl Parse for BenchPart {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            _bench_token: input.parse()?,
        })
    }
}
struct GeneratorPart {
    _gen_token: kw::generator,
    gen_fn: Expr,
}
impl Parse for GeneratorPart {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            _gen_token: input.parse()?,
            gen_fn: input.parse()?,
        })
    }
}
struct SolutionPart {
    part_num: PartNum,
    fns: Vec<Expr>,
    ans: ExpectedResult,
}
impl Parse for SolutionPart {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let part_num = input.parse()?;
        let _brackets = bracketed!(content in input);
        let functions: Punctuated<Expr, Token![,]> =
            content.parse_terminated(Expr::parse, Token![,])?;
        let ans = if input.parse::<Token![=>]>().is_ok() {
            match part_num {
                PartNum::Part1 | PartNum::Part2 => ExpectedResult::Single(input.parse::<Expr>()?),
                PartNum::Both => {
                    let content;
                    let _parens = parenthesized!(content in input);
                    let a = content.parse::<Expr>()?;
                    let _comma = content.parse::<Token![,]>()?;
                    let b = content.parse::<Expr>()?;
                    ExpectedResult::Double(a, b)
                }
            }
        } else {
            ExpectedResult::None
        };
        Ok(Self {
            part_num,
            fns: functions.into_iter().collect(),
            ans,
        })
    }
}
struct DayInput {
    year: i32,
    _day_token: kw::day,
    day: u8,
}
impl Parse for DayInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            year: input.parse::<LitInt>()?.base10_parse()?,
            _day_token: input.parse::<kw::day>()?,
            day: input.parse::<LitInt>()?.base10_parse()?,
        })
    }
}
#[allow(clippy::large_enum_variant)]
enum Parts {
    Day(DayInput),
    Gen(GeneratorPart),
    Part(SolutionPart),
    #[allow(dead_code)]
    Bench(BenchPart),
    Example(ExamplePart),
}
impl Parse for Parts {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::generator) {
            Ok(Parts::Gen(input.parse()?))
        } else if lookahead.peek(kw::bench) {
            Ok(Parts::Bench(input.parse()?))
        } else if lookahead.peek(LitInt) {
            Ok(Parts::Day(input.parse()?))
        } else if lookahead.peek(kw::part1) || lookahead.peek(kw::part2) || lookahead.peek(kw::both)
        {
            Ok(Parts::Part(input.parse()?))
        } else if lookahead.peek(kw::example) {
            Ok(Parts::Example(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
pub struct AocMainInput {
    day: DayInput,
    gen: Option<GeneratorPart>,
    solutions: Vec<SolutionPart>,
    bench: bool,
    examples: Vec<ExamplePart>,
}

impl Parse for AocMainInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let punct: Punctuated<Parts, Token![,]> =
            input.parse_terminated(Parts::parse, Token![,])?;
        let mut day = None;
        let mut gen = None;
        let mut solutions = Vec::new();
        let mut bench = false;
        let mut examples = Vec::new();
        for p in punct {
            match p {
                Parts::Day(d) => {
                    if day.is_none() {
                        day = Some(d);
                    } else {
                        return Err(input.error("Multiple day definitions"));
                    }
                }
                Parts::Gen(g) => {
                    if gen.is_none() {
                        gen = Some(g);
                    } else {
                        return Err(input.error("Multiple generators given"));
                    }
                }
                Parts::Part(p) => {
                    solutions.push(p);
                }
                Parts::Bench(_) => bench = true,
                Parts::Example(e) => examples.push(e),
            }
        }
        if day.is_none() {
            return Err(input.error("No day given"));
        }
        Ok(Self {
            day: day.unwrap(),
            gen,
            solutions,
            bench,
            examples,
        })
    }
}

impl AocMainInput {
    fn add_solution(&self, part: &SolutionPart) -> TokenStream {
        let mut inner = TokenStream::new();
        let part_num = format!("{}", part.part_num);
        let is_single_solution = part.fns.len() == 1;
        let mut do_ans_check = true;
        match (part.part_num, &part.ans) {
            (_, ExpectedResult::None) => {
                do_ans_check = false;
            }
            (PartNum::Both, ExpectedResult::Single(_)) => {
                unreachable!("both example with only one solution??")
            }
            (PartNum::Both, ExpectedResult::Double(p1, p2)) => {
                inner.extend(quote! {
                    let expected_p1 = #p1;
                    let expected_p2 = #p2;
                });
                if let Some(f) = &part.fns.get(0) {
                    inner.extend(quote!{
                        ::aoc_harness::type_hint_pair_has_values_in_func_return(&expected_p1, &expected_p2, &#f);
                    });
                }
                if !is_single_solution {
                    inner.extend(quote! {
                        opts.log(||format!("{desc} both parts expected result: ({expected_p1:?}, {expected_p2:?})"));
                    });
                }
                inner.extend(quote! {
                    results.expect_p1(expected_p1);
                    results.expect_p2(expected_p2);
                });
            }
            (_, ExpectedResult::Double(_, _)) => {
                unreachable!("single example with double solution??")
            }
            (p, ExpectedResult::Single(exp)) => {
                inner.extend(quote! { let expected = #exp; });
                if let Some(f) = &part.fns.get(0) {
                    inner.extend(quote! {
                        ::aoc_harness::type_hint_value_has_same_type_as_func_return(&expected, &#f);
                    });
                }
                if !is_single_solution {
                    inner.extend(quote! {
                        opts.log(||format!("{} {} expected result: {:?}",desc,#part_num, expected));
                    });
                }
                match p {
                    PartNum::Part1 => inner.extend(quote! { results.expect_p1(expected); }),
                    PartNum::Part2 => inner.extend(quote! { results.expect_p2(expected); }),
                    PartNum::Both => unreachable!(),
                }
            }
        }

        for f in &part.fns {
            inner.extend(quote! {
                let solver_name = stringify!(#f);
                let full_name = format!("{} {} via `{}`",&desc, #part_num, solver_name);
            });
            if self.bench {
                inner.extend(quote! {
                    criterion.bench_function(&full_name, |b| b.iter(||#f(&generated)));
                });
            } else {
                inner.extend(quote! {
                    let (t, a) = opts.time_fn(|| #f(&generated));
                    let a = match aoc_harness::answertype::AnswerType::to_option(a) {
                        Some(x) => x,
                        None => panic!("{} failed to produce an answer", &full_name)
                    };
                });
                match part.part_num {
                    PartNum::Part1 => {
                        inner.extend(quote! { results.record_p1(Some(a.clone()),t);});
                    }
                    PartNum::Part2 => {
                        inner.extend(quote! { results.record_p2(Some(a.clone()),t);});
                    }
                    PartNum::Both => {
                        inner.extend(quote! { results.record_both(Some(a.clone()),t);});
                    }
                }
                if !do_ans_check || is_single_solution {
                    inner.extend(quote! {
                        opts.log(||format!("{} solved in {}: {:?}",&full_name, aoc_harness::render_duration(t), a));
                    });
                } else {
                    inner.extend(quote! {
                        opts.log(||format!("{} solved in {}",&full_name, aoc_harness::render_duration(t)));
                    });
                }
            }
        }
        inner
    }

    #[must_use]
    pub fn example(
        &self,
        part_num: &str,
        eg_num: usize,
        expected: &Expr,
        input: &Expr,
        func: &Expr,
        func_select_result: Option<&ExprLit>,
    ) -> TokenStream {
        let mut ans = TokenStream::new();
        if let Some(g) = self.gen.as_ref().map(|x| &x.gen_fn) {
            ans.extend(quote! { let found = #func(&#g(#input)); });
        } else {
            ans.extend(quote! { let found = #func(#input); });
        }
        ans.extend(quote! {
            let as_opt = aoc_harness::answertype::AnswerType::to_option(found);
        });
        if let Some(e) = func_select_result {
            ans.extend(quote! {
                let as_opt = as_opt.map(|v| v.#e);
            });
        }
        ans.extend(quote!{
            match as_opt {
                Some(x) => { assert_eq!(x, #expected, "Example failure: {} example {} with fn {}\nInput:\n{}",#part_num, stringify!(#input), stringify!(#func), #input); }
                None => { assert!(false, "Example failure: {} example {} with fn {} failed", #part_num, #eg_num, stringify!(#func)); }
            };
        });
        ans
    }
    #[must_use]
    pub fn examples(&self) -> TokenStream {
        let mut out = TokenStream::new();
        for (e, eg_num) in self.examples.iter().zip(1..) {
            let part_num = format!("{}", e.part_num);
            match (e.part_num, &e.ans) {
                (_, ExpectedResult::None) => {}
                (PartNum::Both, ExpectedResult::Single(_)) => {
                    unreachable!("both eg with single answer")
                }
                (PartNum::Both, ExpectedResult::Double(a, b)) => {
                    for s in &self.solutions {
                        for f in &s.fns {
                            match s.part_num {
                                PartNum::Part1 => out.extend(self.example(
                                    &part_num,
                                    eg_num,
                                    a,
                                    &e.str_input,
                                    f,
                                    None,
                                )),
                                PartNum::Part2 => out.extend(self.example(
                                    &part_num,
                                    eg_num,
                                    b,
                                    &e.str_input,
                                    f,
                                    None,
                                )),
                                PartNum::Both => {
                                    let get_p1: ExprLit = parse_quote!(0);
                                    let get_p2: ExprLit = parse_quote!(1);
                                    out.extend(self.example(
                                        "both/part1",
                                        eg_num,
                                        a,
                                        &e.str_input,
                                        f,
                                        Some(&get_p1),
                                    ));
                                    out.extend(self.example(
                                        "both/part2",
                                        eg_num,
                                        b,
                                        &e.str_input,
                                        f,
                                        Some(&get_p2),
                                    ));
                                }
                            }
                        }
                    }
                }
                (_, ExpectedResult::Double(_, _)) => {
                    unreachable!("single eg with double answer")
                }
                (_, ExpectedResult::Single(expected)) => {
                    for s in &self.solutions {
                        match s.part_num {
                            PartNum::Part1 | PartNum::Part2 => {
                                if e.part_num == s.part_num {
                                    for f in &s.fns {
                                        out.extend(self.example(
                                            &part_num,
                                            eg_num,
                                            expected,
                                            &e.str_input,
                                            f,
                                            None,
                                        ));
                                    }
                                }
                            }
                            PartNum::Both => {
                                for f in &s.fns {
                                    if e.part_num == PartNum::Part1 {
                                        let get_p1: ExprLit = parse_quote!(0);
                                        out.extend(self.example(
                                            "both/part1",
                                            eg_num,
                                            expected,
                                            &e.str_input,
                                            f,
                                            Some(&get_p1),
                                        ));
                                    }

                                    if e.part_num == PartNum::Part2 {
                                        let get_p2: ExprLit = parse_quote!(1);
                                        out.extend(self.example(
                                            "both/part2",
                                            eg_num,
                                            expected,
                                            &e.str_input,
                                            f,
                                            Some(&get_p2),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        out
    }
    #[must_use]
    pub fn do_macro(&self) -> TokenStream {
        let day = self.day.day;
        let year = self.day.year;
        let mut setup = quote! {
            let basename = std::path::Path::new(file!()).file_name().unwrap();
            let desc = format!("Year {} Day {:02} in {:?}",#year,#day, basename);
            let s : String = match opts.get_input(#year, #day) {
                Ok(s) => s,
                Err(i) => {
                    if i != aoc_harness::InputFetchFailure::PuzzleNotReleasedYet {
                        opts.log(||format!("Missing input for {}: {:?}", &desc, i));
                    }
                    return;
                }
            };
        };
        match self.gen.as_ref().map(|z| &z.gen_fn) {
            Some(g) => setup.extend(quote! {
                let (t, generated) = opts.time_fn(||#g(&s));
                results.record_generator(t);
                opts.log(||format!("{} generated in {}", &desc, aoc_harness::render_duration(t)));
            }),
            None => setup.extend(quote! {
                let generated = s;
            }),
        }
        let solutions: TokenStream = self
            .solutions
            .iter()
            .map(|x| self.add_solution(x))
            .collect();
        let examples = if self.examples.is_empty() {
            quote! {fn check_examples() {}}
        } else {
            let inner = self.examples();
            quote! {
                pub fn check_examples() {
                    #inner
                }
            }
        };
        if self.bench {
            unimplemented!();
        } else {
            quote! {
                #[cfg(test)]
                mod autotests {
                    #[test]
                    fn full_solution() {
                        let mut results = aoc_harness::dayresult::DayResult::new(#year,#day);
                        super::run_with_opts(&mut results, &mut aoc_harness::Opts::for_test());
                    }
                    #[test]
                    pub fn examples() {
                        super::check_examples();
                    }
                }
                pub fn run_with_opts(results: &mut aoc_harness::dayresult::DayResult, opts: &mut aoc_harness::Opts) {
                    #setup
                    #solutions
                    opts.answers.record_dayresult(results).expect("Mismatched results");
                }
                #examples

                #[allow(dead_code)]
                pub fn run_main() -> aoc_harness::dayresult::DayResult {
                    use clap::Parser;
                    let mut opts = aoc_harness::Opts::parse();
                    check_examples();
                    let mut results = aoc_harness::dayresult::DayResult::new(#year,#day);
                    for _ in 0..opts.repeats {
                        run_with_opts(&mut results, &mut opts);
                    }
                    results
                }
            }
        }
    }
}
