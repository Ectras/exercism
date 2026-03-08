use std::{collections::HashSet, sync::Mutex};

use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};

static NAME_LIST: Lazy<Mutex<HashSet<u32>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    fn new_name() -> String {
        let mut rng = thread_rng();
        let mut lock = NAME_LIST.lock().unwrap();
        loop {
            let c1 = rng.gen_range(b'A'..b'Z');
            let c2 = rng.gen_range(b'A'..b'Z');
            let numb = rng.gen_range(0u16..=999);
            let id = ((c1 as u32) << 24) + ((c2 as u32) << 16) + (numb as u32);
            if !lock.contains(&id) {
                lock.insert(id);
                let c1 = char::from_u32(c1.into()).unwrap();
                let c2 = char::from_u32(c2.into()).unwrap();
                return format!("{c1}{c2}{numb}");
            }
        }
    }

    pub fn new() -> Self {
        Self {
            name: Self::new_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::new_name();
    }
}
