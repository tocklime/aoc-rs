use std::path::PathBuf;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{parse::Parse, Token};
use walkdir::WalkDir;
pub struct FindLatestInput {
    dir: Literal,
    _comma: Token![,],
    filename_part: Literal,
}

impl Parse for FindLatestInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dir: input.parse()?,
            _comma: input.parse()?,
            filename_part: input.parse()?,
        })
    }
}
impl FindLatestInput {
    #[must_use]
    pub fn do_macro(&self) -> TokenStream {
        let dir_q = format!("{}", self.dir);
        let filename_part_q = format!("{}", self.filename_part);
        let filename_part = filename_part_q.trim_matches('"');
        let dir = dir_q.trim_matches('"');
        let path = PathBuf::from(dir).canonicalize().unwrap();
        let newest = WalkDir::new(&path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|x| {
                if x.file_type().is_file() {
                    let file_name = x.file_name().to_string_lossy();
                    file_name.ends_with(".rs") && file_name.contains(&filename_part)
                }else {
                    false
                }
            })
            .max_by_key(|x| std::fs::metadata(x.path()).unwrap().modified().unwrap())
            .expect("No rs files found!")
            .into_path()
            .canonicalize()
            .unwrap();
        let name = newest.to_str().unwrap();
        quote! {#name}
    }
}
