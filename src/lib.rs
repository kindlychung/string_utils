pub fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
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
