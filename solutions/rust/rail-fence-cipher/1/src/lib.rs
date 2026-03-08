use std::collections::VecDeque;

pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        assert!(rails > 1);
        Self(rails)
    }

    /// Pingpongs the value such that it is always going back and forth in the range
    /// `0..=limit`.
    fn pingpong(val: u32, limit: u32) -> u32 {
        let val = val % (2 * limit);
        let a = val;
        let b = 2 * limit - val;
        a.min(b)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![Vec::new(); self.0 as usize];
        for (i, c) in text.chars().enumerate() {
            let rail = Self::pingpong(i as u32, self.0 - 1);
            rails[rail as usize].push(c);
        }

        rails.into_iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail_lengths = vec![0; self.0 as usize];
        for i in 0..cipher.chars().count() {
            let rail = Self::pingpong(i as u32, self.0 - 1);
            rail_lengths[rail as usize] += 1;
        }

        let mut chars = cipher.chars();
        let mut rails = Vec::with_capacity(self.0 as usize);
        for count in rail_lengths {
            rails.push(chars.by_ref().take(count).collect::<VecDeque<_>>());
        }

        let mut out = String::new();
        for i in 0..cipher.chars().count() {
            let rail = Self::pingpong(i as u32, self.0 - 1);
            out.push(rails[rail as usize].pop_front().unwrap());
        }

        out
    }
}

#[test]
fn it_works() {
    let r = RailFence::new(4);
    r.decode("AGBFHCEIKDJ");
}
