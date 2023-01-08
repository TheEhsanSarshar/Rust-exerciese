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
        if self.health == 0 {
            Some(Player {
                health: 100,
                mana: if self.level >= 10 {Some(100)} else {None},
                level: self.level
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
           None => {
            let new_value = self.health as i32 - mana_cost as i32;
            if new_value < 0 {
                self.health = 0;
            } else {
                self.health = new_value as u32;
            }
            return 0
           },
           Some(x) if x < mana_cost => {
            return 0
           },
           Some(x) => {
            self.mana = Some(x - mana_cost);
            return 2 * mana_cost;
           }
        }
    }
}
