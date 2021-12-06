use std::collections::HashMap;

pub struct Fish {
    pub remaining : u64
}

struct FishChildrenCounter {
    child_count_cache : HashMap<u64, u64>
}

impl FishChildrenCounter {
    fn init() -> FishChildrenCounter {
        FishChildrenCounter { child_count_cache : HashMap::new() }
    }

    fn number_children(&mut self, fish : &Fish, day : u64) -> u64 {
        if fish.remaining > day {
            0
        } else {
            let spawn_day = day - fish.remaining;
            if let Some(count) = self.child_count_cache.get(&spawn_day) {
                *count
            } else {
                // One higher to include this day
                let new_fish = Fish { remaining : 9 };
                let reset_fish = Fish {remaining : 7};
                let children = 1 + self.number_children(&new_fish, spawn_day)
                                     + self.number_children(&reset_fish, spawn_day);
                self.child_count_cache.insert(spawn_day, children);
                children
            }
        }
    }
}

pub fn count_after(fish : &Vec<Fish>, day : u64) -> u64 {
    let mut fish_children_counter = FishChildrenCounter::init();
    fish.iter().map(
        |fish| 1 + fish_children_counter.number_children(fish, day)
    ).sum()
}