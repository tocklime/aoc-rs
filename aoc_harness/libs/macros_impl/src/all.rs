use std::path::PathBuf;

use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::{parse::Parse, Token};

pub struct AocAllMainInput {
    dir: Literal,
    _comma: Token![,],
    prefix: Literal,
}

impl Parse for AocAllMainInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dir: input.parse()?,
            _comma: input.parse()?,
            prefix: input.parse()?,
        })
    }
}
impl AocAllMainInput {
    #[must_use]
    pub fn do_macro(&self) -> TokenStream {
        let dir_q = format!("{}", self.dir);
        let prefix_q = format!("{}", self.prefix);
        let dir = dir_q.trim_matches('"');
        let prefix = prefix_q.trim_matches('"');
        let path = PathBuf::from(dir);
        let mut mods = TokenStream::new();
        let mut inner = TokenStream::new();
        let fs = path
            .read_dir()
            .expect("Can't read dir")
            .map(|x| x.unwrap().file_name().to_str().unwrap().to_owned())
            .filter(|x| x.starts_with(prefix))
            .sorted();
        for f in fs {
            let short = format_ident!("{}", &f[..f.len() - 3]);
            mods.extend(quote! {
                mod #short;
            });
            inner.extend(quote! {
                println!("{}", #f);
                times.push(#short::run_main());
            });
        }

        quote! {
            use dotenv;
            #mods
            pub fn main() {
                dotenv::dotenv().ok();
                let mut times = Vec::new();
                #inner

                let total_time = times.iter().map(|x| x.total_time()).sum();
                // for t in times {
                //     println!("{}", t.output_line())
                // }
                println!("Total time: {}", aoc_harness::render_duration(total_time));
            }
        }
    }
}
