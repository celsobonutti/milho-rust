pub const RESERVED_KEYWORDS: [&str; 3] = ["if", "def", "defn"];

pub fn is_reserved_keyword(value: &str) -> bool {
    RESERVED_KEYWORDS.contains(&value)
}
