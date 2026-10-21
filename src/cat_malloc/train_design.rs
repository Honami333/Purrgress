use std::collections::HashMap;

use wibr::Make;

use crate::cat_depot::train_rule::PurrRule;
use crate::cat_depot::train_core::*;
use crate::cat_depot::train_error::*;

/// **PurrDesign** is a raw draft that stores blueprints.
/// 
/// It is intended for convenient (human-friendly) addition of stages and nested chains.
/// Each stage is bound to its corresponding condition and an optional chain
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Make, Default)]
pub struct PurrDesign<T: PurrStep, U: PurrRule> {
    /// blueprints is a storage for all drafts for specific carriages in a human-friendly format
    /// Direct interaction with the field removes verification for carriage overwrites and invalid coupling states of DesignBox
    /// Pub for raw interaction with blueprints and writing custom logic for PurrDesign
    #[Some(HashMap::with_capacity(DEFAULT_CAPACITY))] pub blueprints: HashMap<T, DesignBox<T, U>>
}

impl<T, U> PurrDesign<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    /// Raw method for adding single to blueprints
    /// 
    /// # Warning
    /// If the carriage is already in blueprints, the method will overwrite it
    pub fn single(&mut self, carriage: T, rule: U) {
        let design = DesignBox::new(rule);
        self.add_blueprints(carriage, design);
    }

    /// Raw method for adding chain to blueprints
    /// 
    /// # Warning
    /// If the carriage is already in blueprints, the method will overwrite it
    pub fn chain(&mut self, carriage: T, rule: U, coupling: Vec<T>) {
        let mut design = DesignBox::new(rule);
        design.set_coupling(coupling);
        self.add_blueprints(carriage, design);
    }

    /// The method attempts to add single to blueprints
    /// 
    /// # Errors
    /// * `TrainError::DuplicateCarriage { carriage: T }` - if the carriage is already in blueprints
    pub fn try_single(&mut self, carriage: T, rule: U) -> TrainResult<(), T> {
        if self.contains_carriage(carriage) { return Err(TrainError::DuplicateCarriage { carriage }); };
        self.single(carriage, rule);
        Ok(())
    }

    /// The method attempts to add chain to blueprints
    /// 
    /// # Errors
    /// * `TrainError::DuplicateCarriage { carriage: T }` - if the carriage is already in blueprints
    /// * `TrainError::EmptyCoupling` - if the passed coupling contains no carriage
    pub fn try_chain(&mut self, carriage: T, rule: U, coupling: Vec<T>) -> TrainResult<(), T> {
        if self.contains_carriage(carriage) { return Err(TrainError::DuplicateCarriage { carriage }); };
        let mut design = DesignBox::new(rule);
        design.validated_set_coupling(coupling)?;
        self.add_blueprints(carriage, design);
        Ok(())
    }

    /// Raw method for adding DesignBox to blueprints
    /// 
    /// # Warning
    /// If the carriage is already in blueprints, the method will overwrite it
    pub fn add_blueprints(&mut self, carriage: T, design: DesignBox<T, U>) { self.blueprints.insert(carriage, design); }

    /// Checks for the presence of the carriage in blueprints
    pub fn contains_carriage(&self, carriage: T) -> bool { self.blueprints.contains_key(&carriage) }

    /// Completely clears blueprints
    pub fn clear(&mut self) { self.blueprints.clear(); }
}

/// **DesignBox** - this is a container designed for a specific carriage inside PurrDesign that stores its rule and coupling
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::prelude::Component))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Make)]
pub struct DesignBox<T: PurrStep, U: PurrRule> {
    /// rule is a specific execution rule for a carriage within the line structure of PurrTrain
    /// Pub for convenient work with the raw rule and writing custom logic for DesignBox
    pub rule: U,
    /// coupling is an optional nesting and chain vector.
    /// It is required to trigger long chains or deep nesting; see PurrRoute.construct_schedule() for more details.
    /// Direct interaction with the field removes verification for overwrites and invalid coupling states - Some(Vec::new())
    /// Pub for convenient work with the raw coupling and writing custom logic for DesignBox
    #[None] pub coupling: Option<Vec<T>>
}

impl<T, U> DesignBox<T, U> 
where 
    T: PurrStep,
    U: PurrRule
{
    /// Raw method for setting coupling
    /// 
    /// # Warning
    /// If coupling is already set, the method will overwrite it
    /// If you pass an empty coupling, the method will write it as Some(Vec::new())
    pub fn set_coupling(&mut self, coupling: Vec<T>) { self.coupling = Some(coupling); }

    /// Unlike set_coupling, it validates and returns TrainResult<(), T>
    /// 
    /// # Errors
    /// * `TrainError::CouplingAlreadyDefined` - if coupling is already defined
    /// * `TrainError::EmptyCoupling` - if the passed coupling contains no carriage
    /// 
    /// # Warning
    /// In case the coupling is already defined and you pass a new empty coupling
    /// The method will return exactly TrainError::CouplingAlreadyDefined due to priority
    pub fn validated_set_coupling(&mut self, coupling: Vec<T>) -> TrainResult<(), T> {
        if self.coupling_is_some() { return Err(TrainError::CouplingAlreadyDefined); };
        if coupling.is_empty() { return Err(TrainError::EmptyCoupling); };
        self.set_coupling(coupling);
        Ok(())
    }

    /// Checks for the existence of the coupling
    pub fn coupling_is_some(&self) -> bool { self.coupling.is_some() }

    /// Checks for the absence of the coupling
    pub fn coupling_is_none(&self) -> bool { self.coupling.is_none() }
}

// A block of tests for PurrDesign
#[cfg(test)]
mod purr_design_test {
    use crate::cat_depot::train_rule::PurrRuleLite;
    use crate::cat_depot::train_error::TrainError;
    use crate::PurrStep;

    use super::*;

    // Test carriage enum for PurrDesign
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly,
        Pipeline
    }

    // Verification of Ok for the try_single method of PurrDesign when there is no duplication of single stages in blueprints
    #[test]
    fn try_single_ok_new_carriage() {
        let mut purr_design = PurrDesign::new();
        let result = purr_design.try_single(TestCarriage::Component, PurrRuleLite::instant());
        assert!(result.is_ok());
    }

    // Verification of Err ( TrainError::DuplicateCarriage { carriage: TestCarriage::Component } ) for the try_single method of PurrDesign
    // Intentionally duplicating a single stage in blueprints prior to calling try_single
    #[test]
    fn try_single_err_on_duplicate() {
        let mut purr_design = PurrDesign::new();
        // A duplicating single stage
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        let result = purr_design.try_single(TestCarriage::Component, PurrRuleLite::instant());
        assert_eq!(result, Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Component }));
    }

    // Verification of Ok for the try_chain method of PurrDesign when there is no duplication of chain stages in blueprints
    #[test]
    fn try_chain_ok_stores_coupling() {
        let mut purr_design = PurrDesign::new();
        // Coupling
        let coupling = prepare_coupling_pair(&mut purr_design);
        let result = purr_design.try_chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling.clone());
        assert!(result.is_ok());

        // Verification that DesignBox with coupling is actually saved
        let design_box = purr_design.blueprints.get(&TestCarriage::Pipeline);
        assert!(design_box.is_some());
        assert_eq!(design_box.unwrap().coupling, Some(coupling));
    }

    // Verification of Err ( TrainError::DuplicateCarriage { carriage: TestCarriage::Pipeline } ) for the try_chain method of PurrDesign
    // Intentionally duplicating a chain stage in blueprints prior to calling try_chain
    #[test]
    fn try_chain_err_on_duplicate() {
        let mut purr_design = PurrDesign::new();
        // Coupling
        let coupling = prepare_coupling_pair(&mut purr_design);
        // A duplicating chain stage
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling.clone());
        let result = purr_design.try_chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling);
        assert_eq!(result, Err(TrainError::DuplicateCarriage { carriage: TestCarriage::Pipeline }));
    }

    // Verification of Err ( TrainError::EmptyCoupling ) for the try_chain method of PurrDesign
    // Intentionally using an empty vector
    #[test]
    fn try_chain_err_on_empty_coupling_vec() {
        let mut purr_design = PurrDesign::new();
        let result = purr_design.try_chain(TestCarriage::Pipeline, PurrRuleLite::instant(), Vec::new());
        assert_eq!(result, Err(TrainError::EmptyCoupling));
        // Confirmation that the chain was not created for carriage TestCarriage::Pipeline
        assert!(!purr_design.contains_carriage(TestCarriage::Pipeline));
    }

    // Documentation that the single method of PurrDesign silently overwrites another identical single stage in blueprints
    // Left intentionally for control purposes
    #[test]
    fn single_overwrites_existing_silently() {
        let mut purr_design = PurrDesign::new();
        // The first call to single
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        // The second, overwriting call to single
        // Changes rule from PurrRuleLite::instant() to PurrRuleLite::flag()
        purr_design.single(TestCarriage::Component, PurrRuleLite::flag());
        // Retrieving the modified DesignBox directly from blueprints
        let design_box = purr_design.blueprints.get(&TestCarriage::Component);
        // Verification that it exists and that the rule has indeed changed
        assert!(design_box.is_some());
        assert_eq!(design_box.unwrap().rule, PurrRuleLite::flag());
    }

    // Documentation that the chain method of PurrDesign silently overwrites another identical chain of stages in blueprints
    // Left intentionally for control purposes
    #[test]
    fn chain_overwrites_existing_silently() {
        let mut purr_design = PurrDesign::new();
        // Coupling
        let coupling = prepare_coupling_pair(&mut purr_design);
        // The first call to chain
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::instant(), coupling.clone());

        // Creating a reverse copy of the vector and checking their difference
        let mut rev_coupling = coupling.clone();
        rev_coupling.reverse();
        assert_ne!(coupling, rev_coupling);

        // The second, overwriting call to chain
        // Changes rule from PurrRuleLite::instant() to PurrRuleLite::flag() and coupling to rev_coupling
        purr_design.chain(TestCarriage::Pipeline, PurrRuleLite::flag(), rev_coupling.clone());
        // Retrieving the modified DesignBox directly from blueprints
        let design_box = purr_design.blueprints.get(&TestCarriage::Pipeline);
        // Verification that it exists, that the rule has indeed changed, and that the vector inside is exactly equal to rev_coupling
        assert!(design_box.is_some());
        let design_box = design_box.cloned().unwrap();
        assert_eq!(design_box.rule, PurrRuleLite::flag());
        assert_eq!(design_box.coupling, Some(rev_coupling));
    }

    // Verification of the correct behavior of the contains_carriage method by scenarios
    #[test]
    fn purr_design_contains_carriage_scenarios_validation() {
        let mut purr_design = PurrDesign::new();
        // blueprints is empty
        assert!(!purr_design.contains_carriage(TestCarriage::Component));
        assert!(!purr_design.contains_carriage(TestCarriage::Assembly));
        assert!(!purr_design.contains_carriage(TestCarriage::Pipeline));

        // Coupling
        prepare_coupling_pair(&mut purr_design);
        // blueprints contains the required one
        assert!(purr_design.contains_carriage(TestCarriage::Component));
        assert!(purr_design.contains_carriage(TestCarriage::Assembly));
        // blueprints contains everything except the required one
        assert!(!purr_design.contains_carriage(TestCarriage::Pipeline));
    }

    // Verification of the clear method for full collection clearing
    #[test]
    fn clear_removes_all_carriages() {
        let mut purr_design = PurrDesign::new();
        // Coupling
        let _ = prepare_coupling_pair(&mut purr_design);

        // Verification that Component and Assembly have been written to blueprints
        assert!(purr_design.contains_carriage(TestCarriage::Component));
        assert!(purr_design.contains_carriage(TestCarriage::Assembly));

        purr_design.clear();

        // Verification that these stages no longer exist
        assert!(!purr_design.contains_carriage(TestCarriage::Component));
        assert!(!purr_design.contains_carriage(TestCarriage::Assembly));
    }

    // A function that adds 2 singles (TestCarriage::Component, TestCarriage::Assembly) to PurrDesign
    fn prepare_coupling_pair(purr_design: &mut PurrDesign<TestCarriage, PurrRuleLite>) -> Vec<TestCarriage> {
        purr_design.single(TestCarriage::Component, PurrRuleLite::instant());
        purr_design.single(TestCarriage::Assembly, PurrRuleLite::instant());
        // Combining (TestCarriage::Component, TestCarriage::Assembly) into a vector for a chain
        vec![TestCarriage::Component, TestCarriage::Assembly]
    }
}

#[cfg(test)]
mod design_box_test {
    use crate::cat_depot::train_rule::PurrRuleLite;
    use crate::cat_depot::train_error::TrainError;
    use crate::PurrStep;

    use super::*;

    // Test carriage enum for DesignBox
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
    enum TestCarriage {
        Component,
        Assembly
    }

    // Verification of whether detection of Some and None works using the coupling_is_some and coupling_is_none methods
    #[test]
    fn coupling_state_checks_agree_with_option() {
        let mut design_box = DesignBox::new(PurrRuleLite::instant());
        assert!(design_box.coupling_is_none());

        // Forcibly setting coupling to check for Some
        design_box.set_coupling(vec![TestCarriage::Component]);
        assert!(design_box.coupling_is_some())
    }

    // Setting coupling with 1 carriage via the validated_set_coupling method into a None field
    #[test]
    fn validated_set_coupling_ok_stores_value() {
        let mut design_box = DesignBox::new(PurrRuleLite::instant());
        let result = design_box.validated_set_coupling(vec![TestCarriage::Assembly]);
        assert!(result.is_ok());
    }

    // Pre-forcibly assigning coupling in order to get Err(TrainError::CouplingAlreadyDefined) when attempting to call validated_set_coupling
    #[test]
    fn validated_set_coupling_err_already_defined() {
        let mut design_box = DesignBox::new(PurrRuleLite::instant());
        let coupling = vec![TestCarriage::Component];
        design_box.set_coupling(coupling.clone());
        assert!(design_box.coupling_is_some());
        let result = design_box.validated_set_coupling(vec![TestCarriage::Assembly]);
        assert_eq!(result, Err(TrainError::CouplingAlreadyDefined));
        // Confirmation that the coupling was not overwritten
        assert_eq!(design_box.coupling, Some(coupling));
    }

    // An attempt to write an empty vector
    #[test]
    fn validated_set_coupling_err_on_empty_vec() {
        let mut design_box: DesignBox<TestCarriage, PurrRuleLite> = DesignBox::new(PurrRuleLite::instant());
        // Passing an empty vector when calling validated_set_coupling to get Err(TrainError::EmptyCoupling)
        let result = design_box.validated_set_coupling(Vec::new());
        assert!(design_box.coupling_is_none());
        assert_eq!(result, Err(TrainError::EmptyCoupling));
    }

    // An attempt to write an empty vector
    #[test]
    fn set_coupling_allows_empty_vec_unchecked() {
        let mut design_box: DesignBox<TestCarriage, PurrRuleLite> = DesignBox::new(PurrRuleLite::instant());
        // Verification that set_coupling writes an empty vector into coupling as Some
        design_box.set_coupling(Vec::new());
        assert!(design_box.coupling_is_some());
        assert_eq!(design_box.coupling, Some(Vec::new()));
    }

    // Verification of error priority
    #[test]
    fn validated_set_coupling_precedence_some_over_empty() {
        let mut design_box = DesignBox::new(PurrRuleLite::instant());
        // Writing a vector with TestCarriage::Component into coupling
        let coupling = vec![TestCarriage::Component];
        design_box.set_coupling(coupling.clone());

        // Passing an empty vector into validated_set_coupling
        let result = design_box.validated_set_coupling(Vec::new());
        // Ensuring that coupling is Some and its value is a vector with TestCarriage::Component
        assert!(design_box.coupling_is_some());
        assert_eq!(design_box.coupling, Some(coupling));

        // Ensuring that we receive Err(TrainError::CouplingAlreadyDefined) in the first place, rather than Err(TrainError::EmptyCoupling)
        assert_eq!(result, Err(TrainError::CouplingAlreadyDefined));
        assert_ne!(result, Err(TrainError::EmptyCoupling));
    }

    // Documenting that set_coupling silently overwrites coupling
    // Left intentionally for control purposes
    #[test]
    fn set_coupling_overwrites_existing_silently() {
        let mut design_box = DesignBox::new(PurrRuleLite::instant());
        // The first call to set_coupling
        let coupling = vec![TestCarriage::Component];
        design_box.set_coupling(coupling.clone());
        assert!(design_box.coupling_is_some());

        // The second call with a vector containing TestCarriage::Assembly instead of TestCarriage::Component
        let new_coupling = vec![TestCarriage::Assembly];
        design_box.set_coupling(new_coupling.clone());
        assert!(design_box.coupling_is_some());
        // Verifications of vector differences and that the new coupling contains exactly the second vector
        assert_ne!(coupling, new_coupling);
        assert_eq!(design_box.coupling.unwrap(), new_coupling);
    }
}