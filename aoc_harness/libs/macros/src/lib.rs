use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn aoc_main(input: TokenStream) -> TokenStream {
    (parse_macro_input!(input as aoc_harness_macros_impl::AocMainInput))
        .do_macro()
        .into()
}

#[proc_macro]
pub fn aoc_all_main(input: TokenStream) -> TokenStream {
    (parse_macro_input!(input as aoc_harness_macros_impl::all::AocAllMainInput))
        .do_macro()
        .into()
}
