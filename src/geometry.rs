/// Optional differential-response extension for a CF-ACP domain profile.
pub trait DifferentialResponse {
    type State;
    type ObservationProfile;
    type Error;

    /// Returns a response Jacobian with shape response_dim x coordinate_dim.
    fn response_jacobian(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Vec<Vec<f64>>, Self::Error>;
}

/// Compute the pullback metric G = J^T W J.
///
/// `jacobian` has shape m x n and `weight` has shape m x m. The result has
/// shape n x n. Returns `None` for inconsistent dimensions.
pub fn pullback_metric(jacobian: &[Vec<f64>], weight: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    if jacobian.is_empty() {
        return Some(Vec::new());
    }

    let m = jacobian.len();
    let n = jacobian[0].len();
    if jacobian.iter().any(|row| row.len() != n) {
        return None;
    }
    if weight.len() != m || weight.iter().any(|row| row.len() != m) {
        return None;
    }

    let mut wj = vec![vec![0.0; n]; m];
    for i in 0..m {
        for k in 0..m {
            for j in 0..n {
                wj[i][j] += weight[i][k] * jacobian[k][j];
            }
        }
    }

    let mut g = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..m {
                g[i][j] += jacobian[k][i] * wj[k][j];
            }
        }
    }
    Some(g)
}
