pub fn egg_count(mut display_value: u32) -> usize {
    let mut bit_count = 0;
    while display_value > 0 {
        // Add 1 if the last bit is set
        bit_count += (display_value % 2) as usize;

        // Then shift by one to the right
        display_value >>= 1;
    }
    bit_count
}
