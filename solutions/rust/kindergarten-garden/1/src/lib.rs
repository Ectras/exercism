fn map(plant: u8) -> &'static str {
    match plant {
        b'V' => "violets",
        b'G' => "grass",
        b'C' => "clover",
        b'R' => "radishes",
        _ => unreachable!(),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let offset = 2 * (student.as_bytes().first().unwrap() - b'A');
    let newline = diagram.find('\n').unwrap() as u8;
    let diagram = diagram.as_bytes();
    [
        offset,
        offset + 1,
        newline + offset + 1,
        newline + offset + 2,
    ]
    .iter()
    .map(|&idx| diagram[idx as usize])
    .map(map)
    .collect()
}
