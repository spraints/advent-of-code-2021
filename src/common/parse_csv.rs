use std::str::FromStr;

pub fn parse_csv<F: FromStr>(s: &str) -> Vec<Result<F, <F as FromStr>::Err>> {
    s.split(',').map(|x| x.parse()).collect()
}
