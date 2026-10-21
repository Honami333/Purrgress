 59 files changed, 18317 insertions(+), 4549 deletions(-)
 create mode 100644 PurrGress info/.obsidian/app.json
 create mode 100644 PurrGress info/.obsidian/appearance.json
 create mode 100644 PurrGress info/.obsidian/core-plugins.json
 create mode 100644 PurrGress info/.obsidian/graph.json
 create mode 100644 PurrGress info/.obsidian/themes/ITS Theme/manifest.json
 create mode 100644 PurrGress info/.obsidian/themes/ITS Theme/theme.css
 create mode 100644 PurrGress info/.obsidian/workspace.json
 create mode 100644 PurrGress info/PurrDesign.md
 delete mode 100644 examples/basic_test.rs
 delete mode 100644 examples/pandemonium_fps_test.rs
 delete mode 100644 examples/work_under_tets.rs
 rename purrgress-macros/src/{cat_stage_manager/step.rs => cat_malloc/purr_step.rs} (58%)
 delete mode 100644 purrgress-macros/src/cat_motion_blur/grimoire_binding.rs
 delete mode 100644 purrgress-macros/src/cat_motion_blur/pandemonium.rs
 delete mode 100644 purrgress-macros/src/cat_motion_blur/purgatory_line.rs
 delete mode 100644 purrgress-macros/src/cat_motion_blur/purr_engine_hellfire.rs
 delete mode 100644 purrgress-macros/src/cat_stage_manager/chain.rs
 delete mode 100644 purrgress-macros/src/cat_stage_manager/constructor.rs
 delete mode 100644 purrgress-macros/src/cat_stage_manager/octopurr.rs
 delete mode 100644 purrgress-macros/src/cat_stage_manager/pounce_drop.rs
 delete mode 100644 purrgress-macros/src/cat_stage_manager/purr_engine.rs
 delete mode 100644 purrgress-macros/src/cat_telegraph/dispatcher.rs
 create mode 100644 src/cat_depot/train_conditions.rs
 create mode 100644 src/cat_depot/train_conditions_spatial.rs
 create mode 100644 src/cat_depot/train_core.rs
 create mode 100644 src/cat_depot/train_cursor.rs
 create mode 100644 src/cat_depot/train_error.rs
 create mode 100644 src/cat_depot/train_route.rs
 create mode 100644 src/cat_depot/train_rule.rs
 create mode 100644 src/cat_depot/train_rule_spatial.rs
 create mode 100644 src/cat_depot/train_track.rs
 delete mode 100644 src/cat_malloc/purr_train.rs
 create mode 100644 src/cat_malloc/train.rs
 delete mode 100644 src/cat_malloc/train_error.rs
 create mode 100644 src/cat_malloc/train_route_single.rs
 delete mode 100644 src/cat_malloc/train_types.rs
 delete mode 100644 src/cat_motion_blur/memory_demonium.rs
 delete mode 100644 src/cat_motion_blur/pandemonium_types.rs
 delete mode 100644 src/cat_stage_manager/manager.rs
 delete mode 100644 src/cat_stage_manager/manager_conditions.rs
 delete mode 100644 src/cat_stage_manager/manager_macros.rs
 delete mode 100644 src/cat_stage_manager/manager_trail.rs
 delete mode 100644 src/condition.rs
 delete mode 100644 src/types.rs