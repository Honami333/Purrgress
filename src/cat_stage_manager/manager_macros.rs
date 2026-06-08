use super::manager_types::*;
use super::manager::*;


// A file for macro operations
// Not useful in your development unless nesting exceeds a level of one, which the macros provide

// WARNING: NESTING LEVELS GREATER THAN ONE ARE NOT RECOMMENDED IN YOUR CODE
// DUE TO MORE COMPLEX AND SOMETIMES CONFUSING OPERATION AND MAINTENANCE

// However, it is technically absolutely operational

impl<T> StageManager<T> 
where 
    T: PurrStep 
{
    pub fn register_sub_manager(&mut self, sub_manager: StageManager<T>) -> usize {
        // Create manager index
        let mut new_id = 0;

        if !self.sub_managers.is_empty() {
            // We begin an endless cycle of searching for a free key.
            loop {
                if !self.sub_managers.contains_key(&new_id) {
                    break;
                };

                new_id += 1;
            };
        }

        // Registering a manager in the global manager pool hash map as a Box
        self.sub_managers.insert(new_id, Box::new(sub_manager));

        new_id
    }

    // Getting a reference to a sub-manager by index
    #[allow(clippy::borrowed_box)]
    pub fn get_sub_manager(&self, sub_manager_index: usize) -> Option<&Box<StageManager<T>>> {
        self.sub_managers.get(&sub_manager_index)
    }

    // Getting a mutable reference to a sub-manager by index
    pub fn get_sub_manager_mut(&mut self, sub_manager_index: usize) -> Option<&mut Box<StageManager<T>>> {
        self.sub_managers.get_mut(&sub_manager_index)
    }

    // Checking if the sub-manager is the first one in the current queue
    pub fn sub_manager_is_first(&self, first_index: usize, sub_manager_index: usize) -> bool {
        if self.vector_stage.is_empty() { return false; };

        if first_index != sub_manager_index { return  false ;};

        true
    }

    // Completely removing a sub-manager and returning the removal result
    pub fn remove_sub_manager(&mut self, sub_manager_index: usize) -> bool {
        let removed_manager = self.sub_managers.remove(&sub_manager_index);

        removed_manager.is_some()
    }

    // Completely extracting a sub-manager
    pub fn remove_sub_manager_get(&mut self, sub_manager_index: usize) -> Option<Box<StageManager<T>>> {
        self.sub_managers.remove(&sub_manager_index)
    }
}

