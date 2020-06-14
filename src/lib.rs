pub mod distance {

    use cached::proc_macro::cached;
    use cached::SizedCache;

    #[cached(
        type = "SizedCache<String, i32>",
        create = "{ SizedCache::with_size(100) }",
        convert = r#"{ format!("{}_{}", a, b) }"#
    )]
    pub fn levenstein_recursive(a: &str, b: &str) -> i32 {
        if a.is_empty() || b.is_empty() {
            return std::cmp::max(a.chars().count(), b.chars().count()) as i32;
        }

        let cost = if a.chars().next().unwrap() == b.chars().next().unwrap() {
            0
        } else {
            1
        };

        let range_a = a.chars().skip(1).collect::<String>();
        let range_b = b.chars().skip(1).collect::<String>();
        let d1 = levenstein_recursive(&range_a, b) + 1;
        let d2 = levenstein_recursive(a, &range_b) + 1;
        let d3 = levenstein_recursive(&range_a, &range_b) + cost;

        let distance: i32 = *(vec![d1, d2, d3].iter().min().unwrap());

        distance
    }
}

#[cfg(test)]
mod tests {

    use crate::distance;

    #[test]
    fn test_levenshtein_recursive() {
        assert_eq!(distance::levenstein_recursive("egg", "egg"), 0);
        assert_eq!(distance::levenstein_recursive("egg", "eggs"), 1);
        assert_eq!(distance::levenstein_recursive("egg", "book"), 4);
    }
}
