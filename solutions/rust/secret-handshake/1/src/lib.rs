pub fn actions(n: u8) -> Vec<&'static str> {
    let mut out = vec![];
    if n & 0b00001 > 0 {
        out.push("wink");
    }
    if n & 0b00010 > 0 {
        out.push("double blink");
    }
    if n & 0b00100 > 0 {
        out.push("close your eyes");
    }
    if n & 0b01000 > 0 {
        out.push("jump");
    }
    if n & 0b10000 > 0 {
        out.reverse();
    }
    out
}
