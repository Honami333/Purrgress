use super::manager_types::*;
use super::manager::*;


impl<T> StageManager<T> 
where 
    T: PurrStep 
{
    pub fn register_sub_manager(&mut self, sub_manager: StageManager<T>) -> usize {
        let mut new_id = 0;

        if !self.sub_managers.is_empty() {
            loop {
                if !self.sub_managers.contains_key(&new_id) {
                    break;
                };

                new_id += 1;
            };
        }

        self.sub_managers.insert(new_id, Box::new(sub_manager));

        new_id
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_sub_manager(&self, sub_manager_index: usize) -> Option<&Box<StageManager<T>>> {
        self.sub_managers.get(&sub_manager_index)
    }

    pub fn get_sub_manager_mut(&mut self, sub_manager_index: usize) -> Option<&mut Box<StageManager<T>>> {
        self.sub_managers.get_mut(&sub_manager_index)
    }

    pub fn sub_manager_is_first(&self, first_index: usize, sub_manager_index: usize) -> bool {
        if self.vector_stage.is_empty() { return false; };

        if first_index != sub_manager_index { return  false ;};

        true
    }
}

