use std::str::FromStr;

pub fn parse_input_from_str_sep_by<T>(input: &str, sep: &str) -> Vec<T>
where
    T: FromStr,
{
    input.split(sep).map(|x| x.parse().ok().unwrap()).collect()
}
