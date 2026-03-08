// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mana = if self.level >= 10 { Some(100) } else { None };
        match self.health {
            0 => Some(Player { health: 100, mana, level: self.level }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);
                return 2 * mana_cost;
            } else {
                return 0;
            }
        } else {
            self.health -= self.health.min(mana_cost);
            return 0;
        }
    }
}
