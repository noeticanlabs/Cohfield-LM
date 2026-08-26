use cohfield_lm::profiles::infrastructure::{
    InfrastructureModel, InfrastructureObservationProfile, InfrastructureResponse,
    InfrastructureState,
};
use cohfield_lm::{pullback_metric, AdaptiveContinuationModel, DifferentialResponse};

const THETA_A: [f64; 3] = [2.7727, 1.6468, 1.6468];
const THETA_B: [f64; 3] = [1.6468, 1.6468, 2.7727];

const PSI_A: [[f64; 3]; 3] = [
    [1.655, 0.635, 0.682],
    [0.635, 0.248, 0.265],
    [0.682, 0.265, 0.283],
];

const PSI_B: [[f64; 3]; 3] = [
    [0.364, 0.364, 0.729],
    [0.364, 0.364, 0.729],
    [0.729, 0.729, 1.458],
];

fn zero_psi() -> [[f64; 3]; 3] {
    [[0.0; 3]; 3]
}

fn state(theta: [f64; 3], psi: [[f64; 3]; 3]) -> InfrastructureState {
    InfrastructureState {
        x: [0.0; 3],
        theta,
        psi,
    }
}

fn identity(n: usize) -> Vec<Vec<f64>> {
    let mut w = vec![vec![0.0; n]; n];
    for (i, row) in w.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    w
}

fn determinant_3x3(m: &[Vec<f64>]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn frobenius_distance(a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
    a.iter()
        .zip(b.iter())
        .flat_map(|(ra, rb)| ra.iter().zip(rb.iter()))
        .map(|(x, y)| (x - y) * (x - y))
        .sum::<f64>()
        .sqrt()
}

fn frobenius_norm(a: &[Vec<f64>]) -> f64 {
    a.iter()
        .flat_map(|row| row.iter())
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt()
}

fn metric(
    model: &InfrastructureModel,
    state: &InfrastructureState,
    profile: &InfrastructureObservationProfile,
) -> Vec<Vec<f64>> {
    let j = model.response_jacobian(state, profile).unwrap();
    pullback_metric(&j, &identity(j.len())).unwrap()
}

fn quadratic_form(delta: &[f64; 3], g: &[Vec<f64>]) -> f64 {
    let mut total = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            total += delta[i] * g[i][j] * delta[j];
        }
    }
    total
}

fn interpolate(a: &[f64; 3], b: &[f64; 3], t: f64) -> [f64; 3] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
    ]
}

fn metric_path_length(
    model: &InfrastructureModel,
    base_state: &InfrastructureState,
    profile: &InfrastructureObservationProfile,
    points: &[[f64; 3]],
) -> f64 {
    let mut total = 0.0;
    for pair in points.windows(2) {
        let delta = [
            pair[1][0] - pair[0][0],
            pair[1][1] - pair[0][1],
            pair[1][2] - pair[0][2],
        ];
        let midpoint = interpolate(&pair[0], &pair[1], 0.5);
        let mut local_state = base_state.clone();
        local_state.theta = midpoint;
        let g = metric(model, &local_state, profile);
        total += quadratic_form(&delta, &g).sqrt();
    }
    total
}

#[test]
fn v001_horizon_sweep_reproduces_reported_shape_and_scale() {
    let model = InfrastructureModel::default();
    let a = state(THETA_A, zero_psi());
    let b = state(THETA_B, zero_psi());

    let reported = [
        (0.02, 0.01478),
        (0.05, 0.03258),
        (0.10, 0.05311),
        (0.25, 0.0748336),
        (0.50, 0.06577),
    ];

    let mut measured = Vec::new();
    for (horizon, target) in reported {
        let profile = InfrastructureObservationProfile::balanced(horizon);
        let ra = model.observe(&a, &profile).unwrap();
        let rb = model.observe(&b, &profile).unwrap();
        let d = InfrastructureModel::mean_probe_distance(&ra, &rb).unwrap();
        measured.push(d);
        assert!(
            (d - target).abs() < 0.0025,
            "horizon {horizon}: measured {d}, reported {target}"
        );
    }

    assert!(measured[0] < measured[1]);
    assert!(measured[1] < measured[2]);
    assert!(measured[2] < measured[3]);
    assert!(measured[4] < measured[3]);
}

#[test]
fn v001_direct_theta_intervention_collapses_response_difference() {
    let model = InfrastructureModel::default();
    let profile = InfrastructureObservationProfile::balanced(0.25);
    let a = state(THETA_A, zero_psi());
    let b = state(THETA_B, zero_psi());

    let before = InfrastructureModel::mean_probe_distance(
        &model.observe(&a, &profile).unwrap(),
        &model.observe(&b, &profile).unwrap(),
    )
    .unwrap();
    assert!(before > 0.07);

    let mut intervened = a.clone();
    intervened.theta = b.theta;
    let after = InfrastructureModel::mean_probe_distance(
        &model.observe(&intervened, &profile).unwrap(),
        &model.observe(&b, &profile).unwrap(),
    )
    .unwrap();
    assert!(after < 1.0e-12);
}

#[test]
fn v003_pullback_metric_is_positive_definite_and_locally_predictive() {
    let model = InfrastructureModel::default();
    let profile = InfrastructureObservationProfile::balanced(0.25);
    let midpoint = interpolate(&THETA_A, &THETA_B, 0.5);
    let base = state(midpoint, zero_psi());
    let g = metric(&model, &base, &profile);

    assert!(g[0][0] > 0.0);
    assert!(g[0][0] * g[1][1] - g[0][1] * g[1][0] > 0.0);
    assert!(determinant_3x3(&g) > 0.0);

    let delta = [0.01, -0.007, 0.005];
    let predicted = quadratic_form(&delta, &g).sqrt();
    let mut displaced = base.clone();
    for (theta, &d) in displaced.theta.iter_mut().zip(delta.iter()) {
        *theta += d;
    }

    let actual = InfrastructureModel::response_l2(
        &model.observe(&base, &profile).unwrap(),
        &model.observe(&displaced, &profile).unwrap(),
    )
    .unwrap();
    let relative_error = (predicted - actual).abs() / actual;
    assert!(relative_error < 0.01, "relative error {relative_error}");
}

#[test]
fn v003_metric_state_dependence_reproduces_reported_relative_changes() {
    let model = InfrastructureModel::default();
    let profile = InfrastructureObservationProfile::balanced(0.25);
    let midpoint = interpolate(&THETA_A, &THETA_B, 0.5);
    let base = state(midpoint, zero_psi());
    let g0 = metric(&model, &base, &profile);
    let base_norm = frobenius_norm(&g0);
    let direction = [
        THETA_A[0] - THETA_B[0],
        THETA_A[1] - THETA_B[1],
        THETA_A[2] - THETA_B[2],
    ];

    let expected = [(0.25, 0.114), (0.50, 0.234), (0.75, 0.364)];
    for (gamma, target) in expected {
        let mut moved = base.clone();
        for (theta, &d) in moved.theta.iter_mut().zip(direction.iter()) {
            *theta += gamma * d;
        }
        let g = metric(&model, &moved, &profile);
        let relative = frobenius_distance(&g, &g0) / base_norm;
        assert!(
            (relative - target).abs() < 0.015,
            "gamma {gamma}: measured {relative}, reported approximately {target}"
        );
    }
}

#[test]
fn v004_same_endpoints_support_distinct_metric_path_lengths() {
    let model = InfrastructureModel::default();
    let profile = InfrastructureObservationProfile::balanced(0.25);
    let base = state(THETA_A, zero_psi());

    let steps = 24;
    let mut straight = Vec::with_capacity(steps + 1);
    let mut bowed = Vec::with_capacity(steps + 1);
    for k in 0..=steps {
        let t = k as f64 / steps as f64;
        let p = interpolate(&THETA_A, &THETA_B, t);
        straight.push(p);

        let bow = 0.30 * (std::f64::consts::PI * t).sin();
        bowed.push([p[0], p[1] + bow, p[2]]);
    }

    assert_eq!(straight.first(), bowed.first());
    assert_eq!(straight.last(), bowed.last());

    let l_straight = metric_path_length(&model, &base, &profile, &straight);
    let l_bowed = metric_path_length(&model, &base, &profile, &bowed);
    assert!(l_bowed > l_straight);
}

#[test]
fn v006_same_theta_relational_configuration_changes_response_jacobian_and_metric() {
    let model = InfrastructureModel::with_relational_coupling(1.9);
    let profile = InfrastructureObservationProfile::balanced(0.25);
    let theta = [1.8, 1.8, 1.8];
    let a = state(theta, PSI_A);
    let b = state(theta, PSI_B);

    let ra = model.observe(&a, &profile).unwrap();
    let rb = model.observe(&b, &profile).unwrap();
    let response_difference = InfrastructureModel::response_l2(&ra, &rb).unwrap();
    assert!(response_difference > 0.15);

    let ja = model.response_jacobian(&a, &profile).unwrap();
    let jb = model.response_jacobian(&b, &profile).unwrap();
    let jacobian_difference = frobenius_distance(&ja, &jb);
    assert!(jacobian_difference > 0.01);

    let ga = pullback_metric(&ja, &identity(ja.len())).unwrap();
    let gb = pullback_metric(&jb, &identity(jb.len())).unwrap();
    let metric_difference = frobenius_distance(&ga, &gb);
    assert!(metric_difference > 0.001);

    let mut intervened = a.clone();
    intervened.psi = b.psi;
    let ri = model.observe(&intervened, &profile).unwrap();
    assert!(InfrastructureModel::response_l2(&ri, &rb).unwrap() < 1.0e-12);

    let ji = model.response_jacobian(&intervened, &profile).unwrap();
    assert!(frobenius_distance(&ji, &jb) < 1.0e-10);
}

#[test]
fn infrastructure_probe_observation_is_deterministic_from_cloned_state() {
    let model = InfrastructureModel::default();
    let profile = InfrastructureObservationProfile::balanced(0.25);
    let s = state(THETA_A, zero_psi());

    let r1: InfrastructureResponse = model.observe(&s, &profile).unwrap();
    let r2: InfrastructureResponse = model.observe(&s.clone(), &profile).unwrap();
    assert_eq!(r1, r2);
    assert_eq!(s.x, [0.0; 3]);
}
