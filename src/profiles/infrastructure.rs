use crate::{AdaptiveContinuationModel, DifferentialResponse, StateRoles};

#[derive(Clone, Debug, PartialEq)]
pub struct InfrastructureState {
    pub x: [f64; 3],
    pub theta: [f64; 3],
    pub psi: [[f64; 3]; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfrastructureInput {
    pub node: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfrastructureExperience {
    pub theta_delta: [f64; 3],
    pub edge_signature: [f64; 3],
    pub psi_decay: f64,
    pub psi_gain: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfrastructureObservationProfile {
    pub probes: Vec<InfrastructureInput>,
    pub horizon: f64,
    pub integration_step: f64,
    pub jacobian_step: f64,
}

impl InfrastructureObservationProfile {
    pub fn balanced(horizon: f64) -> Self {
        Self {
            probes: vec![
                InfrastructureInput {
                    node: [0.6, -0.6, 0.0],
                },
                InfrastructureInput {
                    node: [-0.6, 0.6, 0.0],
                },
                InfrastructureInput {
                    node: [0.6, 0.0, -0.6],
                },
                InfrastructureInput {
                    node: [-0.6, 0.0, 0.6],
                },
            ],
            horizon,
            integration_step: 0.001,
            jacobian_step: 1.0e-5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfrastructureResponse {
    pub vectors: Vec<[f64; 3]>,
}

impl InfrastructureResponse {
    pub fn flattened(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.vectors.len() * 3);
        for vector in &self.vectors {
            out.extend_from_slice(vector);
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InfrastructureError {
    InvalidHorizon,
    InvalidIntegrationStep,
    InvalidJacobianStep,
    InvalidState,
    InvalidAdaptation,
    EmptyProbeFamily,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfrastructureModel {
    pub lambda: f64,
    pub alpha_psi: f64,
}

impl Default for InfrastructureModel {
    fn default() -> Self {
        Self {
            lambda: 0.5,
            alpha_psi: 0.0,
        }
    }
}

impl InfrastructureModel {
    pub fn with_relational_coupling(alpha_psi: f64) -> Self {
        Self {
            lambda: 0.5,
            alpha_psi,
        }
    }

    fn validate_state(&self, state: &InfrastructureState) -> bool {
        state.x.iter().all(|v| v.is_finite())
            && state.theta.iter().all(|v| v.is_finite() && *v > 0.0)
            && state
                .psi
                .iter()
                .flat_map(|row| row.iter())
                .all(|v| v.is_finite())
            && self.lambda.is_finite()
            && self.lambda > 0.0
            && self.alpha_psi.is_finite()
            && self.alpha_psi >= 0.0
    }

    fn edge_differences(x: &[f64; 3]) -> [f64; 3] {
        [x[0] - x[1], x[1] - x[2], x[0] - x[2]]
    }

    fn effective_edge_response(&self, state: &InfrastructureState) -> [[f64; 3]; 3] {
        let mut h = [[0.0; 3]; 3];
        for (i, row) in h.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value =
                    self.alpha_psi * state.psi[i][j] + if i == j { state.theta[i] } else { 0.0 };
            }
        }
        h
    }

    fn edge_flows(&self, state: &InfrastructureState, x: &[f64; 3]) -> [f64; 3] {
        let d = Self::edge_differences(x);
        let h = self.effective_edge_response(state);
        let mut flows = [0.0; 3];
        for i in 0..3 {
            for j in 0..3 {
                flows[i] += h[i][j] * d[j];
            }
        }
        flows
    }

    fn derivative(
        &self,
        state: &InfrastructureState,
        x: &[f64; 3],
        input: &InfrastructureInput,
    ) -> [f64; 3] {
        let flows = self.edge_flows(state, x);
        let node_outflow = [
            flows[0] + flows[2],
            -flows[0] + flows[1],
            -flows[1] - flows[2],
        ];

        [
            input.node[0] - node_outflow[0] - self.lambda * x[0],
            input.node[1] - node_outflow[1] - self.lambda * x[1],
            input.node[2] - node_outflow[2] - self.lambda * x[2],
        ]
    }

    fn add_scaled(x: &[f64; 3], dx: &[f64; 3], scale: f64) -> [f64; 3] {
        [
            x[0] + scale * dx[0],
            x[1] + scale * dx[1],
            x[2] + scale * dx[2],
        ]
    }

    fn rk4_step(
        &self,
        state: &InfrastructureState,
        x: &[f64; 3],
        input: &InfrastructureInput,
        dt: f64,
    ) -> [f64; 3] {
        let k1 = self.derivative(state, x, input);
        let x2 = Self::add_scaled(x, &k1, 0.5 * dt);
        let k2 = self.derivative(state, &x2, input);
        let x3 = Self::add_scaled(x, &k2, 0.5 * dt);
        let k3 = self.derivative(state, &x3, input);
        let x4 = Self::add_scaled(x, &k3, dt);
        let k4 = self.derivative(state, &x4, input);

        [
            x[0] + dt * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]) / 6.0,
            x[1] + dt * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]) / 6.0,
            x[2] + dt * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]) / 6.0,
        ]
    }

    pub fn mean_probe_distance(
        left: &InfrastructureResponse,
        right: &InfrastructureResponse,
    ) -> Option<f64> {
        if left.vectors.is_empty() || left.vectors.len() != right.vectors.len() {
            return None;
        }

        let mut total = 0.0;
        for (a, b) in left.vectors.iter().zip(&right.vectors) {
            let dx = a[0] - b[0];
            let dy = a[1] - b[1];
            let dz = a[2] - b[2];
            total += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        Some(total / left.vectors.len() as f64)
    }

    pub fn response_l2(
        left: &InfrastructureResponse,
        right: &InfrastructureResponse,
    ) -> Option<f64> {
        let a = left.flattened();
        let b = right.flattened();
        if a.is_empty() || a.len() != b.len() {
            return None;
        }
        Some(
            a.iter()
                .zip(b.iter())
                .map(|(x, y)| (x - y) * (x - y))
                .sum::<f64>()
                .sqrt(),
        )
    }
}

impl AdaptiveContinuationModel for InfrastructureModel {
    type State = InfrastructureState;
    type Fast = [f64; 3];
    type LocalCondition = [f64; 3];
    type RelationalConfiguration = [[f64; 3]; 3];
    type Input = InfrastructureInput;
    type Experience = InfrastructureExperience;
    type ObservationProfile = InfrastructureObservationProfile;
    type Response = InfrastructureResponse;
    type Error = InfrastructureError;

    fn roles(
        &self,
        state: &Self::State,
    ) -> StateRoles<Self::Fast, Self::LocalCondition, Self::RelationalConfiguration> {
        StateRoles {
            fast: state.x,
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
        if !self.validate_state(state) || input.node.iter().any(|v| !v.is_finite()) {
            return Err(InfrastructureError::InvalidState);
        }
        if !horizon.is_finite() || horizon < 0.0 {
            return Err(InfrastructureError::InvalidHorizon);
        }
        if horizon == 0.0 {
            return Ok(state.clone());
        }

        let default_step = 0.001_f64.min(horizon);
        let steps = (horizon / default_step).ceil() as usize;
        let dt = horizon / steps as f64;
        let mut x = state.x;
        for _ in 0..steps {
            x = self.rk4_step(state, &x, input, dt);
        }

        let mut next = state.clone();
        next.x = x;
        Ok(next)
    }

    fn adapt(
        &self,
        state: &Self::State,
        experience: &Self::Experience,
    ) -> Result<Self::State, Self::Error> {
        if !self.validate_state(state)
            || experience.theta_delta.iter().any(|v| !v.is_finite())
            || experience.edge_signature.iter().any(|v| !v.is_finite())
            || !experience.psi_decay.is_finite()
            || !experience.psi_gain.is_finite()
            || !(0.0..=1.0).contains(&experience.psi_decay)
            || experience.psi_gain < 0.0
        {
            return Err(InfrastructureError::InvalidAdaptation);
        }

        let mut next = state.clone();
        for i in 0..3 {
            next.theta[i] += experience.theta_delta[i];
            if next.theta[i] <= 0.0 || !next.theta[i].is_finite() {
                return Err(InfrastructureError::InvalidAdaptation);
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                next.psi[i][j] = (1.0 - experience.psi_decay) * state.psi[i][j]
                    + experience.psi_gain
                        * experience.edge_signature[i]
                        * experience.edge_signature[j];
            }
        }
        Ok(next)
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.validate_state(state) {
            return Err(InfrastructureError::InvalidState);
        }
        if profile.probes.is_empty() {
            return Err(InfrastructureError::EmptyProbeFamily);
        }
        if !profile.horizon.is_finite() || profile.horizon <= 0.0 {
            return Err(InfrastructureError::InvalidHorizon);
        }
        if !profile.integration_step.is_finite() || profile.integration_step <= 0.0 {
            return Err(InfrastructureError::InvalidIntegrationStep);
        }

        let steps = (profile.horizon / profile.integration_step).ceil() as usize;
        let dt = profile.horizon / steps as f64;
        let mut vectors = Vec::with_capacity(profile.probes.len());

        for probe in &profile.probes {
            if probe.node.iter().any(|v| !v.is_finite()) {
                return Err(InfrastructureError::InvalidState);
            }
            let mut x = state.x;
            for _ in 0..steps {
                x = self.rk4_step(state, &x, probe, dt);
            }
            vectors.push([
                (x[0] - state.x[0]) / profile.horizon,
                (x[1] - state.x[1]) / profile.horizon,
                (x[2] - state.x[2]) / profile.horizon,
            ]);
        }

        Ok(InfrastructureResponse { vectors })
    }
}

impl DifferentialResponse for InfrastructureModel {
    type State = InfrastructureState;
    type ObservationProfile = InfrastructureObservationProfile;
    type Error = InfrastructureError;

    fn response_jacobian(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Vec<Vec<f64>>, Self::Error> {
        if !profile.jacobian_step.is_finite() || profile.jacobian_step <= 0.0 {
            return Err(InfrastructureError::InvalidJacobianStep);
        }

        let eps = profile.jacobian_step;
        let response_dim = profile.probes.len() * 3;
        let mut jacobian = vec![vec![0.0; 3]; response_dim];

        for (coordinate, _) in state.theta.iter().enumerate() {
            let mut plus = state.clone();
            let mut minus = state.clone();
            plus.theta[coordinate] += eps;
            minus.theta[coordinate] -= eps;
            if minus.theta[coordinate] <= 0.0 {
                return Err(InfrastructureError::InvalidJacobianStep);
            }

            let r_plus = self.observe(&plus, profile)?.flattened();
            let r_minus = self.observe(&minus, profile)?.flattened();
            for (row, column) in jacobian.iter_mut().enumerate() {
                column[coordinate] = (r_plus[row] - r_minus[row]) / (2.0 * eps);
            }
        }

        Ok(jacobian)
    }
}
