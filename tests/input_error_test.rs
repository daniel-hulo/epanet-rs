//! Integration tests for malformed but parseable INP files: the reader must return an error,
//! not panic later in the solver.

use epanet_rs::error::InputError;
use epanet_rs::simulation::Simulation;

/// A GPV without a curve id gives an error (before HH-4658 it panicked in the solver)
#[test]
fn test_gpv_without_curve_is_an_input_error() {
    let err = Simulation::from_file("tests/gpv-no-curve.inp")
        .err()
        .expect("expected an error for a GPV without a curve");

    assert!(matches!(err, InputError::Parse { .. }));
    assert!(
        err.to_string()
            .contains("GPV valve 'V1' requires a curve id")
    );
}

/// A GPV whose curve has only one point gives an error (before HH-4658 it panicked in the solver)
#[test]
fn test_gpv_with_one_point_curve_is_an_input_error() {
    let err = Simulation::from_file("tests/gpv-short-curve.inp")
        .err()
        .expect("expected an error for a GPV curve with one point");

    assert!(matches!(err, InputError::Parse { .. }));
    assert!(err.to_string().contains("must have at least 2 points"));
}
