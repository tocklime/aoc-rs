use quote::format_ident;
use quote::quote;
use std::path::PathBuf;
use structopt::StructOpt;

use proc_macro2::Ident;
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
}
struct GeneratorPartInput {
    _gen_token: kw::generator,
    gen_fn: Ident,
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
    fns: Vec<Ident>,
    ans: Option<Expr>,
}
impl Parse for PartInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brackets = bracketed!(content in input);
        let fns: Punctuated<Ident, Token![,]> = content.parse_terminated(Ident::parse)?;
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
}
impl Parse for Parts {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::generator) {
            Ok(Parts::Gen(input.parse()?))
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
}
impl Parse for AocMainInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let punct: Punctuated<Parts, Token![,]> = input.parse_terminated(Parts::parse)?;
        let mut day = None;
        let mut gen = None;
        let mut parts = Vec::new();
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
        })
    }
}
impl AocMainInput {
    fn add_part(&self, inner: &mut TokenStream, part_n: u8, part: &PartInput) {
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
                        if !test_mode {
                            println!("Year {} Day {} Part {} expected result: {}",#year,#day,#part_n, expected);
                        }
                    });
                }
                true
            }
        };

        for f in &part.fns {
            inner.extend(quote! {
                let solver_name = stringify!(#f);
                let (t, a) = opts.time_fn(|| #f(&generated));
            });
            if !do_ans_check || is_single_solution {
                inner.extend(quote! {
                    if !test_mode {
                        println!("Year {} Day {} Part {} via `{}` solved in {}: {}",#year, #day, #part_n, solver_name, aoc_harness::render_duration(t), a);
                    }
                })
            } else {
                inner.extend(quote! {
                    if !test_mode {
                        println!("Year {} Day {} Part {} via `{}` solved in {}",#year, #day, #part_n, solver_name, aoc_harness::render_duration(t));
                    }
                })
            }
            if do_ans_check {
                if is_single_solution {
                    inner.extend(quote! {
                        if test_mode {
                            assert_eq!(a,expected);
                        } else {
                            if a != expected {
                                println!("!!! Answer does not match expected");
                            }
                        }
                    });
                } else {
                    inner.extend(quote! {
                        if test_mode {
                            assert_eq!(a,expected);
                        } else {
                            if a != expected {
                                println!("!!! Answer does not match expected: {}", a);
                            }
                        }
                    });
                }
            } else {
                //if !is_multi_solution || !do_ans_check
            }
        }
    }
    pub fn do_macro(&self) -> TokenStream {
        let day = self.day.day;
        let year = self.day.year;
        let mut inner = quote! {
            let s : String = opts.get_input(#year, #day);
        };
        match self.gen.as_ref().map(|z| &z.gen_fn) {
            Some(g) => inner.extend(quote! {
                let (t, generated) = opts.time_fn(||#g(&s));
                println!("Year {} Day {} generated in {}", #year, #day, aoc_harness::render_duration(t));
            }),
            None => inner.extend(quote! {
                let generated = s;
            }),
        }
        self.add_part(&mut inner, 1, &self.p1);
        self.add_part(&mut inner, 2, &self.p2);
        let tests_name = format_ident!("test_year_{}_day_{}", (year as u32), day);
        quote! {
            use structopt::StructOpt;
            #[test]
            fn #tests_name() {
                run_with_opts(aoc_harness::Opts::default(), true);
            }
            pub fn run_with_opts(opts: aoc_harness::Opts, test_mode : bool) {
                #inner
            }
            pub fn main() {
                let opts = aoc_harness::Opts::from_args();
                run_with_opts(opts,false);
            }
        }
    }
}
pub fn generator_id_func(input: &str) -> &str {
    input
}

#[derive(StructOpt, Debug, Default)]
pub struct Opts {
    /// Do full benchmarks for each part
    #[structopt(short, long)]
    bench: bool,
    /// Override the input with the contents of this file
    #[structopt(short, long)]
    input: Option<PathBuf>,
}

impl Opts {
    pub fn get_input(&self, year: i32, day: u8) -> String {
        match &self.input {
            None => {
                let mut aoc = aocf::Aoc::new()
                    .year(Some(year))
                    .day(Some(day.into()))
                    .parse_cli(false)
                    .init()
                    .unwrap();
                aoc.get_input(false)
                    .expect("Couldn't get input for day from adventofcode.com.")
            }
            Some(f) => std::fs::read_to_string(f).expect("Couldn't read file"),
        }
    }
    pub fn time_fn<O, F>(&self, f: F) -> (time::Duration, O)
    where
        F: FnOnce() -> O,
    {
        if self.bench {
            unimplemented!()
        } else {
            time::Duration::time_fn(f)
        }
    }
}

pub fn render_duration(d: time::Duration) -> String {
    let (n, unit) = if d.whole_seconds() > 0 {
        (d.whole_seconds(), "s")
    } else if d.subsec_milliseconds() > 0 {
        (d.subsec_milliseconds().into(), "ms")
    } else if d.subsec_microseconds() > 0 {
        (d.subsec_microseconds().into(), "µs")
    } else {
        (d.subsec_nanoseconds().into(), "ns")
    };
    format!("{} {}", n, unit)
}
