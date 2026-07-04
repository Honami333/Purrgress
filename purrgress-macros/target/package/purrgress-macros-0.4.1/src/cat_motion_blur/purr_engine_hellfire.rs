use syn::{Result, Token, Ident, parse::{Parse, ParseStream}};
use proc_macro;
use quote;


pub(crate) struct BrimstoneRumble {
    pub(crate) _comma1: Token![!],
    pub(crate) _comma2: Token![!],
    pub(crate) _comma3: Token![!],
    pub(crate) animated_meta_data: Ident,
}

impl Parse for BrimstoneRumble {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(BrimstoneRumble {
            _comma1: input.parse()?,
            _comma2: input.parse()?,
            _comma3: input.parse()?,
            animated_meta_data: input.parse()?,
        })
    }
}

// The final stage of the animation
// An updater macro for all internal frames
pub fn purr_rumble_brimstone_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as BrimstoneRumble);

    let animated_meta_data = &input.animated_meta_data;

    let expanded = quote::quote! {
        'macro_scope_update: {
            let animated_meta_data = &mut #animated_meta_data 
                as *mut purrgress::cat_motion_blur::memory_demonium::PurrAnimator<PurrFrameStage, _, _>;
            
            // Pre-saving all required stages to return the matched results
            let mut global_stage = None;

            unsafe {
                let animator = (*animated_meta_data).get_animator_mut();

                // Retrieving the first element in the current animator queue
                // To conserve resources
                let purr_stage = animator.first_vec_query().copied();

                // Pre-saving all required stages to return the matched results
                let mut current_frame_id = None;

                let mut current_sub_stage = None;

                let mut current_stage = None;

                // Reading all metadata
                let Some(purr_stage) = purr_stage else { break 'macro_scope_update (current_stage, current_sub_stage, current_frame_id) };

                let Some(animated_stages) = (*animated_meta_data).get_animated_stages_no_key(purr_stage)
                    else { break 'macro_scope_update (current_stage, current_sub_stage, current_frame_id) };
                
                global_stage = Some(*animated_stages.0);

                let purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::PurrChain(index) = purr_stage
                    else { break 'macro_scope_update (current_stage, current_sub_stage, current_frame_id) };
                
                let Some(sub_stage_manager) = animator.get_sub_manager_mut(index)
                    else { break 'macro_scope_update (current_stage, current_sub_stage, current_frame_id) };

                let mut frame_stage = None;

                // Iterating through all substages
                for (frame_stage_index, flow_meta_data) in animated_stages.1.1.get_flow_stages() {
                    let last_frame_index = flow_meta_data.get_last_frame_index();
                    let get_flow_stage_chain_index = flow_meta_data.get_flow_stage_chain_index();

                    let frame = if let cur_frame = flow_meta_data.get_frame() as usize { cur_frame } else { 1 };

                    // Calling update on the library's custom frame update function
                    if let purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::Frame(index) = last_frame_index {
                        let frame_manager_procces = purrgress_macros::purr_rumble!(
                            sub_stage_manager : get_flow_stage_chain_index,
                            PurrFrameStage,
                            purrgress::cat_motion_blur::memory_demonium::flow_stage_chain : delta, index
                        );

                        // Processing the update to return data to the user
                        let Some(procces_stage) = frame_manager_procces
                            else { continue; };

                        match procces_stage {
                            purrgress::types::PurrEvent::Idle => (),
                            purrgress::types::PurrEvent::Running(stage) => {
                                if let purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::Frame(index) = stage {
                                    current_frame_id = Some(index % frame);
                                    frame_stage = Some(frame_stage_index);
                                };
                            },
                            purrgress::types::PurrEvent::Transition { .. } => ()
                        };
                    };
                };

                // Calling update inside the second-level sub-manager and immediately processing the update to return data to the user
                match sub_stage_manager.update() {
                    purrgress::types::PurrEvent::Idle => {
                        if let Some(sub_manager_flag) = animator
                            .get_condition_mut::<purrgress::condition::PurrFlag>(purr_stage) {

                            sub_manager_flag.set_flag(true);
                        };
                    },
                    purrgress::types::PurrEvent::Running(_) => {
                        current_sub_stage = frame_stage;
                    },
                    purrgress::types::PurrEvent::Transition { .. } => ()
                };

                // Calling the final update of the main manager
                match animator.update() {
                    purrgress::types::PurrEvent::Idle => (),
                    purrgress::types::PurrEvent::Running(_) => {
                        current_stage = global_stage;
                    },
                    purrgress::types::PurrEvent::Transition { .. } => ()
                };

                // Returning the updated metadata and animation state to the user
                (current_stage, current_sub_stage, current_frame_id)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}