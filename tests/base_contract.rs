use cohfield_lm::{binary_survival, mean_recovery_margin, pullback_metric, recovery_margin};

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
fn identity_weight_pullback_is_j_transpose_j() {
    let j = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let w = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let g = pullback_metric(&j, &w).unwrap();

    assert_eq!(g, vec![vec![10.0, 14.0], vec![14.0, 20.0]]);
}

#[test]
fn invalid_recovery_boundary_fails_closed() {
    assert_eq!(recovery_margin(0.1, 0.0), None);
    assert_eq!(recovery_margin(0.1, -1.0), None);
}
