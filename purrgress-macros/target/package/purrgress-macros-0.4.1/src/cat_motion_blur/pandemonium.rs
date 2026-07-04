use syn::{Expr, Result, Token, parse::{Parse, ParseStream}};
use syn::punctuated::Punctuated;
use proc_macro;
use quote;


pub(crate) struct PandemoniumInput {
    pub(crate) all_hall_stage: InfernalStage
}

pub(crate) struct InfernalStage {
    pub(crate) _comma1: Token![!],
    pub(crate) _comma2: Token![!],
    pub(crate) stage: Expr,
    pub(crate) _comma3: Token![:],
    pub(crate) _comma4: Token![<],
    pub(crate) sub_stages: Option<Punctuated<AbyssalSubStage, Token![=>]>>,
    pub(crate) _comma5: Token![>],
}

pub(crate) struct AbyssalSubStage {
    pub(crate) sub_stage: Expr,
    pub(crate) _comma1: Token![,],
    pub(crate) frame: Expr,
    pub(crate) fps: Expr,
    pub(crate) duration: Option<Expr>,
}

impl Parse for AbyssalSubStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let sub_stage = input.parse()?;

        let _comma1 = input.parse()?;

        let content;

        let _brackets = syn::bracketed!(content in input);

        let frame =  content.parse()?;

        let _: Token![,] = content.parse()?;

        let fps =  content.parse()?;

        let duration = if content.peek( Token![,]) {
            let _: Token![,] = content.parse()?;
            Some(content.parse()?)
        } else {
            None
        };

        Ok(AbyssalSubStage {
            sub_stage,
            _comma1,
            frame,
            fps,
            duration,
        })
    }
}

impl Parse for InfernalStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let _comma1 = input.parse()?;
        let _comma2 = input.parse()?;
        let stage = input.parse()?;
        let _comma3 = input.parse()?;

        let _comma4 = input.parse()?;

        let sub_stages = if input.peek(Token![>]) {
            None
        } else {
            Some(Punctuated::parse_separated_nonempty(input)?)
        };

        let _comma5 = input.parse()?;

        Ok(InfernalStage {
            _comma1,
            _comma2,
            stage,
            _comma3,
            _comma4,
            sub_stages,
            _comma5,
        })
    }
}

impl Parse for PandemoniumInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PandemoniumInput {
            all_hall_stage: input.parse()?,
        })
    }
}

// A heavy macro for creating a nested chain inside a sub-manager
//  Writing metadata
//  Completely creating a frame chain for each user substage
pub fn purr_pandemonium_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as PandemoniumInput);

    // Calling manager creation on the library's built-in enum
    let create_manager = quote::quote! {
        purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage::meowphosis_manager();
    };

    let stages = &input.all_hall_stage;

    let name_stage = &stages.stage;

    let mut purr_flow_meta_data = Vec::new();
    
    // Working with each user substage
    if let Some(infernal_stage) = &stages.sub_stages {
        for abyssal in infernal_stage.iter() {
            // Retrieving frame, FPS, and duration data from the user
            let infernal_stage = fps_redaction(&abyssal.frame, &abyssal.fps, &abyssal.duration);

            if let Some(meta_data) = infernal_stage {
                let frame = meta_data.0;
                let fps =  meta_data.1;
                let duration = meta_data.2;

                // Calculating the total number of frames in the animation over its full duration
                let max_frame = frame * (duration / (frame / fps));
                let max_frame = max_frame.round() as usize;

                // Calculating the duration of a single frame
                let mata_data_step = 1.0 / fps;

                let mut frame_vec = Vec::new();

                for i in 0..max_frame {
                    let frame_list = quote::quote! {
                        // Creating a frame vector for the substage by setting the condition as the frame duration
                        PurrFrameStage::Frame(#i) : purrgress::condition::PurrTimer::new(#mata_data_step)
                    };

                    frame_vec.push(frame_list);
                };

                let sab_stage = &abyssal.sub_stage;

                purr_flow_meta_data.push(quote::quote! {
                    {
                        // Creating a nested chain using the already familiar macro
                        let flow_stage_chain_index = purrgress_macros::new_purr_chain!(
                            cat_manager,
                            PurrFrameStage,

                            // Flattening the vector to get the frame dependency chain
                            #( #frame_vec )=>*
                        );

                        // Writing complete metadata for the current substage
                        let flow_data = purrgress::cat_motion_blur::memory_demonium::PurrFlowMetaData::new(
                            flow_stage_chain_index,
                            #max_frame - 1,

                            #frame,
                            #fps,
                            #duration,
                        );

                        (#sab_stage, flow_data)
                    }
                });
            } else {
                return syn::Error::new_spanned(
                    &abyssal.sub_stage,
                    format!("Error in specifying crop metadata")
                ).to_compile_error().into();
            };
        };
    };

    let expanded = quote::quote! {
        {
            let mut cat_manager = #create_manager;

            // Compiling and returning all metadata about the current stage to the user
            let meta_data = vec![ #( #purr_flow_meta_data ),* ];

            let ani_data = purrgress::cat_motion_blur::memory_demonium::PurrAnimateMetaData::new(meta_data);

            (#name_stage, cat_manager, ani_data)
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// Retrieving data and dynamically changing FPS
fn fps_redaction(frame: &Expr, fps: &Expr, duration: &Option<Expr>) -> Option<(f32, f32, f32)> {
    // Extracting the user's raw numbers
    let raw_frame = match frame {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit), ..}) => 
            lit.base10_parse::<usize>().unwrap_or(1),
                        
        _ => 0
    };

    let raw_fps = match fps {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit), ..}) => 
            lit.base10_parse::<usize>().unwrap_or(1),
                        
         _ =>  0
    };


    if matches!((raw_frame, raw_fps), (0, _) | (_, 0)) { return None; };

    let raw_frame = raw_frame as f32;
    let raw_fps = raw_fps as f32;

    // Retrieving the total duration after enum conversion
    let duration_seconds: f32 = if let Some(dur) = duration {
        match dur {
            syn::Expr::Call(expr_call) => {
                if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(lit), ..})) = expr_call.args.first() {
                    let val = lit.base10_parse::<f32>().unwrap_or(raw_frame / raw_fps);

                    let func_expr = &expr_call.func;
                    let expr_call_str = quote::quote! { #func_expr }.to_string();

                    if expr_call_str.contains("Millis") {
                        val / 1000.0
                    } else if expr_call_str.contains("Minutes") {
                        val * 60.0
                    } else {
                        val
                    }
                } else {
                    raw_frame / raw_fps
                }
            },
            _ => raw_frame / raw_fps
        }
    } else {
        raw_frame / raw_fps
    };
    // In case of failure, setting the duration of a single cycle to frame / fps

    let perfect_time = raw_frame / raw_fps;

    let count = (duration_seconds / perfect_time).max(1.0);

    // Dynamically changing the FPS to fit the animation into the specified time
    let new_fps = if count.fract() < 0.0001 {
        raw_fps
    } else {
        let max_frame = count.round() * raw_frame;
        max_frame / duration_seconds
    };

    Some((raw_frame, new_fps, duration_seconds))
}