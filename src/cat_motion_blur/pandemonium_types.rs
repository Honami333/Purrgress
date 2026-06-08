use purrgress_macros::{meowphosis, PurrStep};


// Animator data file

// An enum created in accordance with the custom enum creation instructions for the animation manager
#[meowphosis]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum PurrFrameStage {
    Frame(usize),
    PurrChain(usize)
}

/// # Example of AbyssalDuration usage
/// 
/// ```rust
/// use purrgress_macros::{meowphosis, PurrStep};
/// use purrgress::cat_stage_manager::*;
/// use purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage;
/// 
/// // Creating a custom stage enum
/// #[meowphosis]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
/// pub enum MyStage {
///     Idle,
///     Walk,
///     Run,
///     PurrChain(usize)
/// }
/// 
/// // Creating a custom substage enum
/// #[derive(Debug, Clone, Copy)]
/// pub enum MyFrameStage {
///     Start,
///     Run,
///     End,
///     Pause,
/// }
/// 
/// // Setting all necessary data for the animation
/// let idle_ani_manager = purrgress_macros::purr_pandemonium!(
///     !!MyStage::Idle : <
///         MyFrameStage::Start, [3, 10] =>
/// 
///         // Specifying the animation duration for any condition in milliseconds, seconds, or minutes
///         MyFrameStage::Run, [3, 12, pandemonium_types::AbyssalDuration::Seconds(1.0)] => 
///         MyFrameStage::End, [3, 10]
///     >
/// );
/// ```
pub enum AbyssalDuration {
    Millis(f32),
    Seconds(f32),
    Minutes(f32),
}