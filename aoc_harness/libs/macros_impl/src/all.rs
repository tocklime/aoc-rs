use std::path::PathBuf;

use proc_macro2::{Literal, TokenStream};
use quote::{quote, format_ident};
use syn::parse::Parse;
use walkdir::WalkDir;

pub struct AocAllMainInput {
    dir: Literal,
}

impl Parse for AocAllMainInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dir: input.parse()?,
        })
    }
}
fn find_num(s: &str) -> Option<usize> {
    let x: String = s
        .chars()
        .skip_while(|c| !c.is_numeric())
        .take_while(|c| c.is_numeric())
        .collect();
    // .split(|c: char| !c.is_numeric()).collect();
    x.parse().ok()
}
impl AocAllMainInput {
    #[must_use]
    pub fn do_macro(&self) -> TokenStream {
        let dir_q = format!("{}", self.dir);
        let dir = dir_q.trim_matches('"');
        let path = PathBuf::from(dir)
            .canonicalize()
            .expect("cannot find directory");
        let files: Vec<_> = WalkDir::new(&path)
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(|x| {
                if x.file_type().is_file() {
                    let file_name = x.file_name().to_string_lossy();
                    if let Some(stripped) = file_name.strip_suffix(".rs") {
                        let parent_dir_name = x
                            .path()
                            .components()
                            .nth_back(1)
                            .unwrap()
                            .as_os_str()
                            .to_string_lossy();
                        return match (find_num(&parent_dir_name), find_num(&file_name)) {
                            (Some(y), Some(d)) => Some((
                                i32::try_from(y).expect("bad year"),
                                u8::try_from(d).expect("day too big"),
                                format_ident!("{}",parent_dir_name),
                                format_ident!("{}",stripped)
                            )),
                            _ => None,
                        };
                    }
                }
                None
            })
            .collect();
        let mut adds = TokenStream::new();
        let len = files.len();
        for (y, d, dir, file) in files {
            adds.extend(quote! {
                ans.push(((#y, #d), aoc::solutions::#dir::#file::run_with_opts));
            })
        }
        quote!{
            fn make_all() -> Vec<Day>{
                let mut ans : Vec<Day> = Vec::with_capacity(#len);
                #adds
                ans
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::find_num;

    #[test]
    fn test_find_num() {
        assert_eq!(find_num("42"), Some(42));
        assert_eq!(find_num("y2022"), Some(2022));
        assert_eq!(find_num("y2022d21"), Some(2022));
        assert_eq!(find_num("d02"), Some(2));
        assert_eq!(find_num("d25"), Some(25));
        assert_eq!(find_num("25rsten"), Some(25));
        assert_eq!(find_num("foo"), None);
        assert_eq!(find_num(""), None);
    }
}
