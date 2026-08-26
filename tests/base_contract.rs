use cohfield_lm::{
    binary_survival, mean_recovery_margin, pullback_metric, recovery_margin,
    AdaptiveContinuationModel, StateRoles,
};

#[derive(Clone, Debug, PartialEq)]
struct ToyState {
    fast: f64,
    theta: f64,
    psi: f64,
}

#[derive(Clone, Debug)]
struct ToyObservation {
    probe: f64,
    horizon: f64,
}

struct ToyDomain;

impl AdaptiveContinuationModel for ToyDomain {
    type State = ToyState;
    type Fast = f64;
    type LocalCondition = f64;
    type RelationalConfiguration = f64;
    type Input = f64;
    type Experience = f64;
    type ObservationProfile = ToyObservation;
    type Response = f64;
    type Error = &'static str;

    fn roles(&self, state: &Self::State) -> StateRoles<Self::Fast, Self::LocalCondition, Self::RelationalConfiguration> {
        StateRoles {
            fast: state.fast,
            local_condition: state.theta,
            relational_configuration: state.psi,
        }
    }

    fn evolve(
        &self,
        state: &Self::State,
        input: &Self::Input,
        horizon: f64,
    ) -> Result<Self::State, Self::Error> {
        if !horizon.is_finite() || horizon < 0.0 {
            return Err("invalid horizon");
        }

        let mut next = state.clone();
        let rate = input - state.theta * state.fast + state.psi;
        next.fast += horizon * rate;
        Ok(next)
    }

    fn adapt(
        &self,
        state: &Self::State,
        experience: &Self::Experience,
    ) -> Result<Self::State, Self::Error> {
        if !experience.is_finite() {
            return Err("non-finite experience");
        }

        let mut next = state.clone();
        next.psi += experience;
        Ok(next)
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        Ok(self.evolve(state, &profile.probe, profile.horizon)?.fast)
    }
}

#[test]
fn signed_margin_refines_binary_survival() {
    let near = recovery_margin(0.19, 0.20).unwrap();
    let far = recovery_margin(0.05, 0.20).unwrap();

    assert!(binary_survival(near));
    assert!(binary_survival(far));
    assert_ne!(
        near, far,
        "the margin preserves information the threshold discards"
    );
}

#[test]
fn mean_margin_matches_v010_definition() {
    let margins = [0.4, 0.2, -0.1, 0.5];
    let q = mean_recovery_margin(&margins).unwrap();
    assert!((q - 0.25).abs() < 1e-12);
}

#[test]
fn invalid_margin_inputs_fail_closed() {
    assert_eq!(recovery_margin(0.1, 0.0), None);
    assert_eq!(recovery_margin(0.1, -1.0), None);
    assert_eq!(recovery_margin(f64::NAN, 0.2), None);
    assert_eq!(mean_recovery_margin(&[]), None);
    assert_eq!(mean_recovery_margin(&[0.1, f64::NAN]), None);
}

#[test]
fn identity_weight_pullback_is_j_transpose_j() {
    let j = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let w = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let g = pullback_metric(&j, &w).unwrap();

    assert_eq!(g, vec![vec![10.0, 14.0], vec![14.0, 20.0]]);
}

#[test]
fn inconsistent_pullback_dimensions_fail_closed() {
    let ragged_j = vec![vec![1.0, 2.0], vec![3.0]];
    let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    assert_eq!(pullback_metric(&ragged_j, &identity), None);

    let j = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let wrong_weight = vec![vec![1.0]];
    assert_eq!(pullback_metric(&j, &wrong_weight), None);
}

#[test]
fn toy_domain_implements_base_without_infrastructure_types() {
    let model = ToyDomain;
    let state = ToyState {
        fast: 0.0,
        theta: 0.5,
        psi: 0.25,
    };
    let roles = model.roles(&state);

    assert_eq!(roles.fast, 0.0);
    assert_eq!(roles.local_condition, 0.5);
    assert_eq!(roles.relational_configuration, 0.25);

    let adapted = model.adapt(&state, &0.1).unwrap();
    assert_eq!(adapted.psi, 0.35);
    assert_eq!(state.psi, 0.25, "adaptation returns a new State");
}

#[test]
fn deterministic_identical_state_observation_has_zero_repeat_floor() {
    let model = ToyDomain;
    let state = ToyState {
        fast: 0.2,
        theta: 0.5,
        psi: 0.25,
    };
    let profile = ToyObservation {
        probe: 0.7,
        horizon: 0.25,
    };

    let a = model.observe(&state.clone(), &profile).unwrap();
    let b = model.observe(&state.clone(), &profile).unwrap();
    assert_eq!(a, b);
}

#[test]
fn persistent_relational_configuration_can_causally_change_continuation_at_fixed_fast_state() {
    let model = ToyDomain;
    let profile = ToyObservation {
        probe: 1.0,
        horizon: 0.25,
    };

    let a = ToyState {
        fast: 0.0,
        theta: 1.0,
        psi: 0.25,
    };
    let b = ToyState {
        fast: 0.0,
        theta: 1.0,
        psi: 0.75,
    };

    assert_eq!(model.roles(&a).fast, model.roles(&b).fast);
    let response_a = model.observe(&a, &profile).unwrap();
    let response_b = model.observe(&b, &profile).unwrap();
    assert_ne!(response_a, response_b);

    let intervened = ToyState {
        psi: b.psi,
        ..a.clone()
    };
    let response_intervened = model.observe(&intervened, &profile).unwrap();
    assert_eq!(response_intervened, response_b);
}

#[test]
fn invalid_horizon_fails_closed_in_toy_domain() {
    let model = ToyDomain;
    let state = ToyState {
        fast: 0.0,
        theta: 1.0,
        psi: 0.0,
    };
    assert!(model.evolve(&state, &1.0, -0.1).is_err());
    assert!(model.evolve(&state, &1.0, f64::NAN).is_err());
}
