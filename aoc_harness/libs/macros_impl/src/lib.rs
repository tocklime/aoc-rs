use quote::format_ident;
use quote::quote;

use proc_macro2::TokenStream;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;
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
struct GeneratorPartInput {
    _gen_token: kw::generator,
    gen_fn: Expr,
}
impl Parse for GeneratorPartInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            _gen_token: input.parse()?,
            gen_fn: input.parse()?,
        })
    }
}
struct PartInput {
    fns: Vec<Expr>,
    ans: Option<Expr>,
}
impl Parse for PartInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brackets = bracketed!(content in input);
        let fns: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse)?;
        let ans = match input.parse::<Token![=>]>() {
            Ok(_) => Some(input.parse::<Expr>()?),
            Err(_) => None,
        };
        Ok(Self {
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
    Gen(GeneratorPartInput),
    Part(PartInput),
    Bench(BenchPart),
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
        } else if lookahead.peek(token::Bracket) {
            Ok(Parts::Part(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
pub struct AocMainInput {
    day: DayInput,
    gen: Option<GeneratorPartInput>,
    p1: PartInput,
    p2: PartInput,
    bench: bool,
}
impl Parse for AocMainInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let punct: Punctuated<Parts, Token![,]> = input.parse_terminated(Parts::parse)?;
        let mut day = None;
        let mut gen = None;
        let mut parts = Vec::new();
        let mut bench = false;
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
                    parts.push(p);
                }
                Parts::Bench(_) => bench = true,
            }
        }
        if day.is_none() {
            return Err(input.error("No day given"));
        }
        match parts.len() {
            0 => {
                return Err(input.error("No parts given"));
            }
            1 => {
                parts.push(PartInput {
                    fns: Vec::new(),
                    ans: None,
                });
            }
            2 => {}
            _ => {
                return Err(input.error("More than 2 parts given"));
            }
        }
        assert_eq!(parts.len(), 2);
        let mut i = parts.into_iter();
        Ok(Self {
            day: day.unwrap(),
            gen,
            p1: i.next().unwrap(),
            p2: i.next().unwrap(),
            bench,
        })
    }
}
impl AocMainInput {
    fn add_part(&self, part_n: u8, part: &PartInput) -> TokenStream {
        let mut inner = TokenStream::new();
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
                        opts.log(||format!("Year {} Day {} Part {} expected result: {:?}",#year,#day,#part_n, expected));
                    });
                }
                true
            }
        };

        for f in &part.fns {
            inner.extend(quote! {
                let solver_name = stringify!(#f);
                let full_name = format!("Year {} Day {} Part {} via `{}`",#year,#day, #part_n, solver_name);
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
        let part1 = self.add_part(1, &self.p1);
        let part2 = self.add_part(2, &self.p2);
        let tests_name = format_ident!("test_year_{}_day_{}", (year as u32), day);
        if self.bench {
            quote! {
                pub fn part1(criterion : &mut criterion::Criterion) {
                    let test_mode : bool = true;
                    let opts = aoc_harness::Opts::for_test();
                    #setup;
                    #part1;
                }
                pub fn part2(criterion : &mut criterion::Criterion) {
                    let test_mode : bool = true;
                    let opts = aoc_harness::Opts::for_test();
                    #setup;
                    #part2;
                }
                pub fn benches() {
                    let mut criterion = criterion::Criterion::default();
                    part1(&mut criterion);
                    part2(&mut criterion);
                }
                fn main() {
                    benches();
                    criterion::Criterion::default().final_summary();
                }
            }
        } else {
            quote! {
                use structopt::StructOpt;
                #[cfg(test)]
                mod test {
                    #[test]
                    fn #tests_name() {
                        super::run_with_opts(aoc_harness::Opts::default(), true);
                    }
                }
                pub fn run_with_opts(opts: aoc_harness::Opts, test_mode : bool) {
                    #setup
                    #part1
                    #part2
                }
                pub fn main() {
                    let opts = aoc_harness::Opts::from_args();
                    run_with_opts(opts,false);
                }
            }
        }
    }
}
