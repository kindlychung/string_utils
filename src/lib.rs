/// Returns a &str in which a given suffix is hidden 
/// See https://stackoverflow.com/a/59330922/562222
pub fn withotu_suffix<'a>(src: &'a str, suffix: &str) -> &'a str {
    if src.ends_with(suffix) {
        &src[..src.len() - suffix.len()]
    } else {
        src
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn remove_suffix() {
        let a = "xxx.pdf";
        let b = super::remove_suffix(a, ".pdf");
        assert_eq!(b, "xxx")
    }
}
