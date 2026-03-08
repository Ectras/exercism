pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with('?');
    let upper = message.to_uppercase();
    let is_shout = upper != message.to_lowercase() && upper == message;
    let is_silent = message.is_empty();
    match (is_question, is_shout, is_silent) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (false, false, true) => "Fine. Be that way!",
        _ => "Whatever."
    }
}
