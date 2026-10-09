#![warn(missing_docs)]


extern crate self as purrgress;

#[cfg(feature = "rkyv")]
pub use rkyv;
#[cfg(feature = "serde")]
pub use serde;

#[cfg(feature = "train")]
pub use purrgress_macros::{PurrStep, PurrRule};


#[cfg(feature = "dispatcher")]
pub mod cat_telegraph {
    pub mod dispatcher_condition;
    pub mod dispatcher_types;
    pub mod dispatcher;
    pub mod station_link;
}

#[cfg(feature = "train")]
pub mod cat_depot {
    #[cfg(feature = "spatial")]
    pub mod train_conditions_spatial;
    pub mod train_conditions;
    pub mod train_core;
    #[cfg(feature = "cursor")]
    pub mod train_cursor;
    pub mod train_error;
    pub mod train_route;
    #[cfg(feature = "spatial")]
    pub mod train_rule_spatial;
    pub mod train_rule;
    pub mod train_track;
}

#[cfg(feature = "train")]
pub mod cat_malloc {
    pub mod train;
    pub mod train_design;
    #[cfg(feature = "single")]
    pub mod train_route_single;
    pub mod train_route;
    pub mod train_siding;
}



