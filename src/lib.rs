extern crate self as purrgress;

#[cfg(feature = "animator")]
pub mod cat_motion_blur {
    pub mod memory_demonium;
    pub mod pandemonium_types;
}

pub mod cat_stage_manager {
    pub mod condition;
    mod manager_conditions;
    mod manager_macros;
    mod manager_trail;
    pub mod manager_types;
    pub mod manager;
}
