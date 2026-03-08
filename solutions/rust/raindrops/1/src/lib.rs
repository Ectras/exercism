pub fn raindrops(n: u32) -> String {
    let div3 = n % 3 == 0;
    let div5 = n % 5 == 0;
    let div7 = n % 7 == 0;
    if div3 || div5 || div7 {
        let mut res = String::new();
        if div3 {
            res.push_str("Pling");
        }
        if div5 {
            res.push_str("Plang");
        }
        if div7 {
            res.push_str("Plong");
        }
        res
    } else {
        n.to_string()
    }
}
