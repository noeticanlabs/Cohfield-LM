use cohfield_lm::profiles::infrastructure::{
    InfrastructureExperience, InfrastructureModel, InfrastructureState,
};
use cohfield_lm::AdaptiveContinuationModel;

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

fn determinant_3x3(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn effective_h(theta: [f64; 3], psi: [[f64; 3]; 3], alpha: f64) -> [[f64; 3]; 3] {
    [
        [
            theta[0] + alpha * psi[0][0],
            alpha * psi[0][1],
            alpha * psi[0][2],
        ],
        [
            alpha * psi[1][0],
            theta[1] + alpha * psi[1][1],
            alpha * psi[1][2],
        ],
        [
            alpha * psi[2][0],
            alpha * psi[2][1],
            theta[2] + alpha * psi[2][2],
        ],
    ]
}

fn positive_definite_3x3(m: &[[f64; 3]; 3]) -> bool {
    let minor_1 = m[0][0];
    let minor_2 = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    minor_1 > 0.0 && minor_2 > 0.0 && determinant_3x3(m) > 0.0
}

#[test]
fn v006_reported_relational_configurations_preserve_positive_effective_coupling() {
    let theta = [1.8, 1.8, 1.8];
    let alpha = 1.9;
    let ha = effective_h(theta, PSI_A, alpha);
    let hb = effective_h(theta, PSI_B, alpha);

    assert!(positive_definite_3x3(&ha));
    assert!(positive_definite_3x3(&hb));
}

#[test]
fn coflow_adaptation_from_zero_configuration_produces_psd_rank_one_structure() {
    let model = InfrastructureModel::with_relational_coupling(1.9);
    let state = InfrastructureState {
        x: [0.0; 3],
        theta: [1.8; 3],
        psi: [[0.0; 3]; 3],
    };
    let experience = InfrastructureExperience {
        theta_delta: [0.0; 3],
        edge_signature: [0.8, -0.4, 0.2],
        psi_decay: 0.1,
        psi_gain: 0.5,
    };

    let adapted = model.adapt(&state, &experience).unwrap();
    assert_eq!(adapted.psi[0][1], adapted.psi[1][0]);
    assert_eq!(adapted.psi[0][2], adapted.psi[2][0]);
    assert_eq!(adapted.psi[1][2], adapted.psi[2][1]);

    let q = experience.edge_signature;
    let probes = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, -2.0, 0.5],
    ];
    for v in probes {
        let dot = v[0] * q[0] + v[1] * q[1] + v[2] * q[2];
        let expected = experience.psi_gain * dot * dot;
        let quadratic = v[0]
            * (adapted.psi[0][0] * v[0] + adapted.psi[0][1] * v[1] + adapted.psi[0][2] * v[2])
            + v[1]
                * (adapted.psi[1][0] * v[0] + adapted.psi[1][1] * v[1] + adapted.psi[1][2] * v[2])
            + v[2]
                * (adapted.psi[2][0] * v[0] + adapted.psi[2][1] * v[1] + adapted.psi[2][2] * v[2]);
        assert!((quadratic - expected).abs() < 1.0e-12);
        assert!(quadratic >= 0.0);
    }
}
