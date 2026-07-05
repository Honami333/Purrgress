extern crate self as purrgress;

#[cfg(feature = "train")]
pub mod cat_malloc {
    pub mod purr_train;
    pub mod train_design;
    pub mod train_route;
    pub mod train_siding;
    pub mod train_types;
}

#[cfg(feature = "animator")]
pub mod cat_motion_blur {
    pub mod memory_demonium;
    pub mod pandemonium_types;
}

#[cfg(feature = "scrap")]
pub mod cat_stage_manager {
    mod manager_conditions;
    mod manager_macros;
    mod manager_trail;
    pub mod manager;
}

pub mod condition;
pub mod types;