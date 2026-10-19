// Insert before value_to_query_selector, retaining its existing documentation.
fn selector_literal_pattern(text: &str) -> String {
    let mut pattern = String::with_capacity(text.len());
    for character in text.chars() {
        if "\\.+*?()|[]{}^$#&-~".contains(character) {
            pattern.push('\\');
        }
        pattern.push(character);
    }
    pattern
}

// Replace only the external escape call with:
// crate::entities::regex::Regex::new(&selector_literal_pattern(text.as_str()))
