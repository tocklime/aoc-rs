use std::fmt::Display;

use quote::quote;

use proc_macro2::TokenStream;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token;
use syn::Expr;
use syn::LitInt;
use syn::Result;
use syn::Token;

pub mod all;

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
    expected_ans: Expr,
}
impl Parse for ExamplePart {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ex_token: kw::example = input.parse()?;
        let part_num: PartNum = input.parse()?;
        let str_input: Expr = input.parse()?;
        let _goes_to = input.parse::<Token![=>]>()?;
        let expected_ans: Expr = input.parse()?;
        Ok(Self {
            part_num,
            str_input,
            expected_ans,
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
    ans: Option<Expr>,
}
impl Parse for SolutionPart {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let part_num = input.parse()?;
        let _brackets = bracketed!(content in input);
        let fns: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse)?;
        let ans = match input.parse::<Token![=>]>() {
            Ok(_) => Some(input.parse::<Expr>()?),
            Err(_) => None,
        };
        Ok(Self {
            part_num,
            fns: fns.into_iter().collect(),
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
enum Parts {
    Day(DayInput),
    Gen(GeneratorPart),
    Part(SolutionPart),
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
        } else if lookahead.peek(kw::part1) || lookahead.peek(kw::part2)  || lookahead.peek(kw::both) {
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
        let punct: Punctuated<Parts, Token![,]> = input.parse_terminated(Parts::parse)?;
        let mut day = None;
        let mut gen = None;
        let mut solutions = Vec::new();
        let mut bench = false;
        let mut examples = Vec::new();
        for p in punct.into_iter() {
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
        let year = self.day.year;
        let day = self.day.day;
        let is_single_solution = part.fns.len() == 1;
        let do_ans_check = match part.ans.as_ref() {
            None => false,
            Some(a) => {
                inner.extend(quote! {
                    let expected = #a;
                });
                if !is_single_solution {
                    inner.extend(quote! {
                        opts.log(||format!("Year {} Day {} {} expected result: {:?}",#year,#day,#part_num, expected));
                    });
                }
                true
            }
        };

        for f in &part.fns {
            inner.extend(quote! {
                let solver_name = stringify!(#f);
                let full_name = format!("Year {} Day {} {} via `{}`",#year,#day, #part_num, solver_name);
            });
            if self.bench {
                inner.extend(quote! {
                    criterion.bench_function(&full_name, |b| b.iter(||#f(&generated)));
                });
            } else {
                inner.extend(quote! {
                    let (t, a) = opts.time_fn(|| #f(&generated));
                });
                if !do_ans_check || is_single_solution {
                    inner.extend(quote! {
                    opts.log(||format!("{} solved in {}: {:?}",&full_name, aoc_harness::render_duration(t), a));
                })
                } else {
                    inner.extend(quote! {
                        opts.log(||format!("{} solved in {}",&full_name, aoc_harness::render_duration(t)));
                    });
                }
                if do_ans_check {
                    inner.extend(quote! {
                        opts.assert_eq(a,expected);
                    });
                }
            }
        }
        inner
    }
    pub fn example(
        &self,
        part_num: &str,
        eg_num: usize,
        expected: &Expr,
        input: &Expr,
        func: &Expr,
    ) -> TokenStream {
        match self.gen.as_ref().map(|x| &x.gen_fn) {
            Some(g) => quote! {
                assert_eq!(#func(&#g(#input)), #expected, "Example failure: {} example {} with fn {}",#part_num, #eg_num, stringify!(#func));
            },
            None => quote! {
                assert_eq!(#func(#input), #expected, "Example failure: {} example {} with fn {}",#part_num, #eg_num, stringify!(#func));
            },
        }
    }
    pub fn examples(&self) -> TokenStream {
        let mut out = TokenStream::new();
        for (e, eg_num) in self.examples.iter().zip(1..) {
            let part_num = format!("{}", e.part_num);
            let ans = &e.expected_ans;
            if e.part_num == PartNum::Both {
                let p1_ans = parse_quote! { #ans.0 };
                let p2_ans = parse_quote! { #ans.1 };
                for s in &self.solutions {
                    for f in &s.fns {
                        match s.part_num {
                            PartNum::Part1 => out.extend(self.example(
                                &part_num,
                                eg_num,
                                &p1_ans,
                                &e.str_input,
                                f,
                            )),
                            PartNum::Part2 => out.extend(self.example(
                                &part_num,
                                eg_num,
                                &p2_ans,
                                &e.str_input,
                                f,
                            )),
                            PartNum::Both => {
                                out.extend(self.example(&part_num, eg_num, ans, &e.str_input, f))
                            }
                        }
                    }
                }
            } else {
                for s in &self.solutions {
                    if s.part_num == PartNum::Both {
                        unimplemented!("Please give both-style examples for both-style solutions");
                    } else if s.part_num == e.part_num {
                        for f in &s.fns {
                            out.extend(self.example(&part_num, eg_num, &ans, &e.str_input, f));
                        }
                    }
                }
            }
        }
        out
    }
    pub fn do_macro(&self) -> TokenStream {
        let day = self.day.day;
        let year = self.day.year;
        let mut setup = quote! {
            let s : String = opts.get_input(#year, #day);
        };
        match self.gen.as_ref().map(|z| &z.gen_fn) {
            Some(g) => setup.extend(quote! {
                let (t, generated) = opts.time_fn(||#g(&s));
                opts.log(||format!("Year {} Day {} generated in {}", #year, #day, aoc_harness::render_duration(t)));
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
                use structopt::StructOpt;
                #[cfg(test)]
                mod test {
                    #[test]
                    fn full_solution() {
                        super::run_with_opts(&aoc_harness::Opts::default(), true);
                    }
                    #[test]
                    pub fn examples() {
                        super::check_examples();
                    }
                }
                pub fn run_with_opts(opts: &aoc_harness::Opts, test_mode : bool) {
                    #setup
                    #solutions
                }
                #examples
                pub fn main() {
                    let opts = aoc_harness::Opts::from_args();
                    check_examples();
                    for _ in 0..opts.repeats {
                        run_with_opts(&opts,false);
                    }
                }
            }
        }
    }
}
