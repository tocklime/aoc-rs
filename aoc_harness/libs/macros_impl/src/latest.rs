use std::path::PathBuf;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::parse::Parse;
use walkdir::WalkDir;
pub struct FindLatestInput {
    dir: Literal,
}

impl Parse for FindLatestInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dir: input.parse()?,
        })
    }
}
impl FindLatestInput {
    #[must_use]
    pub fn do_macro(&self) -> TokenStream {
        let dir_q = format!("{}", self.dir);
        let dir = dir_q.trim_matches('"');
        let path = PathBuf::from(dir).canonicalize().unwrap();
        let newest = WalkDir::new(&path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|x| x.file_type().is_file() && x.file_name().to_string_lossy().ends_with(".rs"))
            .max_by_key(|x| std::fs::metadata(x.path()).unwrap().modified().unwrap())
            .expect("No rs files found!")
            .into_path()
            .canonicalize()
            .unwrap();
        let name = newest.to_str().unwrap();
        quote! {#name}
    }
}
