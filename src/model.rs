/// Typed projections of one complete domain state into the three experimentally
/// distinguished adaptive-continuation roles.
#[derive(Clone, Debug, PartialEq)]
pub struct StateRoles<X, Theta, Psi> {
    pub fast: X,
    pub local_condition: Theta,
    pub relational_configuration: Psi,
}

/// CF-ACP-000 minimum executable contract.
///
/// Implementations provide domain mathematics. The trait does not prescribe a
/// graph, ODE, learning law, neural architecture, tokenizer, or language model.
pub trait AdaptiveContinuationModel {
    type State;
    type Fast;
    type LocalCondition;
    type RelationalConfiguration;
    type Input;
    type Experience;
    type ObservationProfile;
    type Response;
    type Error;

    /// Project the complete state into the semantically distinct X/Theta/Psi roles.
    fn roles(
        &self,
        state: &Self::State,
    ) -> StateRoles<Self::Fast, Self::LocalCondition, Self::RelationalConfiguration>;

    /// Domain evolution Phi_tau : Z x U -> Z.
    fn evolve(
        &self,
        state: &Self::State,
        input: &Self::Input,
        horizon: f64,
    ) -> Result<Self::State, Self::Error>;

    /// Experience adaptation A : Z x E -> Z.
    fn adapt(
        &self,
        state: &Self::State,
        experience: &Self::Experience,
    ) -> Result<Self::State, Self::Error>;

    /// Continuation response R_O : Z -> Y_O.
    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error>;
}
