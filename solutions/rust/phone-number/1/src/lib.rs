use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let regex = Regex::new(
        r#"(?x)^
        \s*
        (?:\+?1\s*)?    # Country code
        [(]?
        ([2-9]\d{2})    # Area code
        [)]?\s*[.]?\s*
        ([2-9]\d{2})    # Exchange code
        \s*[.-]?\s*
        (\d{4})         # Subscriber number
        \s*$"#,
    )
    .unwrap();

    let caps = regex.captures(user_number)?;
    let area_code = caps.get(1).unwrap().as_str();
    let exchange_code = caps.get(2).unwrap().as_str();
    let subscriber_number = caps.get(3).unwrap().as_str();
    Some(String::from(area_code) + exchange_code + subscriber_number)
}
