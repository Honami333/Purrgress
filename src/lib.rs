extern crate self as purrgress;

#[cfg(feature = "macros")]
pub use purrgress_macros::PurrStep;
#[cfg(feature = "train")]
pub use purrgress_macros::PurrRule;
#[cfg(feature = "dispatcher")]
pub use purrgress_macros::PurrKey;
#[cfg(feature = "scrap")]
pub use purrgress_macros::{purr_pounce, purr_tentacle, purr_rumble, new_purr_chain, meowphosis};
#[cfg(feature = "animator")]
pub use purrgress_macros::{purr_pandemonium, abyssal_grimoire, purr_rumble_brimstone, abyssal_march};


#[cfg(feature = "dispatcher")]
pub mod cat_telegraph {
    pub mod dispatcher_condition;
    pub mod dispatcher_types;
    pub mod dispatcher;
    pub mod station_link;
}

#[cfg(feature = "train")]
pub mod cat_malloc {
    pub mod purr_train;
    pub mod train_design;
    pub mod train_error;
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