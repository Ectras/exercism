pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }
    let size: usize = size.try_into().unwrap();
    let mut out = vec![vec![0; size]; size];
    let mut pos = [0, -1];
    let mut counter = 1;
    let mut steps = size;
    'outer: loop {
        for direction in 0..4 {
            if direction == 1 || direction == 3 {
                if steps == 1 {
                    break 'outer;
                }
                steps -= 1;
            }
            for _ in 0..steps {
                let x = [1, 0, -1, 0][direction];
                let y = [0, 1, 0, -1][direction];
                pos[1] += x;
                pos[0] += y;
                out[pos[0] as usize][pos[1] as usize] = counter;
                counter += 1;
            }
        }
    }
    out
}
