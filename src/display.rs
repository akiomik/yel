pub fn display_option_bool(o: &Option<bool>) -> String {
    match o {
        Some(s) => format!("{s}"),
        None => "-".to_string(),
    }
}
