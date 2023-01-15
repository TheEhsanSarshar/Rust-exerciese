use std::collections::{BTreeMap, HashMap};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    roaster: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roaster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roaster
            .entry(grade)
            .and_modify(|list| list.push(student.to_string()))
            .or_insert(vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut final_list: Vec<u32> = vec![];
        for (key, _) in &self.roaster {
            final_list.push(*key)
        }
        final_list
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let tree_map = self.roaster.get(&grade);

        if let Some(result) = tree_map {
            let mut result = result.clone();
            result.sort();
            result
        } else {
            vec![]
        }
    }
}
