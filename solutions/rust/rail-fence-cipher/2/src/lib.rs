/// Returns an iterator that pingpongs forever in the range `0..=limit`.
fn pingpong(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}

pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![Vec::new(); self.0 as usize];
        for (c, i) in text.chars().zip(pingpong(self.0)) {
            rails[i].push(c);
        }

        rails.into_iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        // Mostly taken from the solution of k-hamada.
        // However, fixed to work with non-ascii chars

        let mut indices = pingpong(self.0)
            .zip(1..)
            .take(cipher.chars().count())
            .collect::<Vec<_>>();
        indices.sort();

        let mut char_with_index = cipher
            .chars()
            .zip(indices)
            .map(|(c, (_, i))| (i, c))
            .collect::<Vec<_>>();

        char_with_index.sort();
        char_with_index.iter().map(|(_, c)| c).collect()
    }
}
