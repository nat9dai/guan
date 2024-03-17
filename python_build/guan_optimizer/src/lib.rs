//
// Auto-generated file by OptimizationEngine
// See https://alphaville.github.io/optimization-engine/
//
//


use optimization_engine::{constraints::*, panoc::*, alm::*, *};

// ---Private Constants----------------------------------------------------------------------------------

/// Tolerance of inner solver
const EPSILON_TOLERANCE: f64 = 5000.0;

/// Initial tolerance
const INITIAL_EPSILON_TOLERANCE: f64 = 5000.0;

/// Update factor for inner tolerance
const EPSILON_TOLERANCE_UPDATE_FACTOR: f64 = 0.1;

/// Delta tolerance
const DELTA_TOLERANCE: f64 = 0.0001;

/// LBFGS memory
const LBFGS_MEMORY: usize = 10;

/// Maximum number of iterations of the inner solver
const MAX_INNER_ITERATIONS: usize = 500;

/// Maximum number of outer iterations
const MAX_OUTER_ITERATIONS: usize = 10;

/// Maximum execution duration in microseconds
const MAX_DURATION_MICROS: u64 = 5000000;

/// Penalty update factor
const PENALTY_UPDATE_FACTOR: f64 = 5.0;

/// Initial penalty
const INITIAL_PENALTY_PARAMETER: Option<f64> = Some(0.1);

/// Sufficient decrease coefficient
const SUFFICIENT_INFEASIBILITY_DECREASE_COEFFICIENT: f64 = 0.1;

/// Whether preconditioning should be applied
const DO_PRECONDITIONING: bool = false;

// ---Public Constants-----------------------------------------------------------------------------------

/// Number of decision variables
pub const GUAN_OPTIMIZER_NUM_DECISION_VARIABLES: usize = 120;

/// Number of parameters
pub const GUAN_OPTIMIZER_NUM_PARAMETERS: usize = 9;

/// Number of parameters associated with augmented Lagrangian
pub const GUAN_OPTIMIZER_N1: usize = 300;

/// Number of penalty constraints
pub const GUAN_OPTIMIZER_N2: usize = 150;



// ---Parameters of the constraints----------------------------------------------------------------------






// ---Parameters of ALM-type constraints (Set C)---------------------------------------------------------
const SET_C_XMIN :Option<&[f64]> = Some(&[0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,0.0,0.0,0.2777777777777778,0.2,0.0,0.0,0.0,0.0,-254.0,-80.0,]);
const SET_C_XMAX :Option<&[f64]> = Some(&[9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,9999.0,9999.0,27.77777777777778,0.8,565.0,103.0,1.0,1.0,254.0,80.0,]);




// ---Parameters of ALM-type constraints (Set Y)---------------------------------------------------------
/// Y_min
const SET_Y_XMIN :Option<&[f64]> = Some(&[-1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0, -1000000000000.0]);

/// Y_max
const SET_Y_XMAX :Option<&[f64]> = Some(&[1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0, 1000000000000.0]);




// ---Internal private helper functions------------------------------------------------------------------

/// Make constraints U
fn make_constraints() -> impl Constraint {
    // - Cartesian product of constraints:
        let bounds = CartesianProduct::new();
        
        let idx_1 = 1;
        let xmin_1 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_1:Option<&[f64]> = Some(&[103.0,]);
        let set_1 = Rectangle::new(xmin_1, xmax_1);
        let bounds = bounds.add_constraint(idx_1, set_1);
        
        let idx_2 = 2;
        let data_2: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_2 = FiniteSet::new(data_2);
        let bounds = bounds.add_constraint(idx_2, set_2);
        
        let idx_3 = 3;
        let data_3: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_3 = FiniteSet::new(data_3);
        let bounds = bounds.add_constraint(idx_3, set_3);
        
        let idx_4 = 4;
        let xmin_4 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_4:Option<&[f64]> = Some(&[254.0,]);
        let set_4 = Rectangle::new(xmin_4, xmax_4);
        let bounds = bounds.add_constraint(idx_4, set_4);
        
        let idx_5 = 5;
        let xmin_5 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_5:Option<&[f64]> = Some(&[103.0,]);
        let set_5 = Rectangle::new(xmin_5, xmax_5);
        let bounds = bounds.add_constraint(idx_5, set_5);
        
        let idx_6 = 6;
        let data_6: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_6 = FiniteSet::new(data_6);
        let bounds = bounds.add_constraint(idx_6, set_6);
        
        let idx_7 = 7;
        let data_7: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_7 = FiniteSet::new(data_7);
        let bounds = bounds.add_constraint(idx_7, set_7);
        
        let idx_8 = 8;
        let xmin_8 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_8:Option<&[f64]> = Some(&[254.0,]);
        let set_8 = Rectangle::new(xmin_8, xmax_8);
        let bounds = bounds.add_constraint(idx_8, set_8);
        
        let idx_9 = 9;
        let xmin_9 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_9:Option<&[f64]> = Some(&[103.0,]);
        let set_9 = Rectangle::new(xmin_9, xmax_9);
        let bounds = bounds.add_constraint(idx_9, set_9);
        
        let idx_10 = 10;
        let data_10: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_10 = FiniteSet::new(data_10);
        let bounds = bounds.add_constraint(idx_10, set_10);
        
        let idx_11 = 11;
        let data_11: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_11 = FiniteSet::new(data_11);
        let bounds = bounds.add_constraint(idx_11, set_11);
        
        let idx_12 = 12;
        let xmin_12 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_12:Option<&[f64]> = Some(&[254.0,]);
        let set_12 = Rectangle::new(xmin_12, xmax_12);
        let bounds = bounds.add_constraint(idx_12, set_12);
        
        let idx_13 = 13;
        let xmin_13 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_13:Option<&[f64]> = Some(&[103.0,]);
        let set_13 = Rectangle::new(xmin_13, xmax_13);
        let bounds = bounds.add_constraint(idx_13, set_13);
        
        let idx_14 = 14;
        let data_14: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_14 = FiniteSet::new(data_14);
        let bounds = bounds.add_constraint(idx_14, set_14);
        
        let idx_15 = 15;
        let data_15: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_15 = FiniteSet::new(data_15);
        let bounds = bounds.add_constraint(idx_15, set_15);
        
        let idx_16 = 16;
        let xmin_16 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_16:Option<&[f64]> = Some(&[254.0,]);
        let set_16 = Rectangle::new(xmin_16, xmax_16);
        let bounds = bounds.add_constraint(idx_16, set_16);
        
        let idx_17 = 17;
        let xmin_17 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_17:Option<&[f64]> = Some(&[103.0,]);
        let set_17 = Rectangle::new(xmin_17, xmax_17);
        let bounds = bounds.add_constraint(idx_17, set_17);
        
        let idx_18 = 18;
        let data_18: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_18 = FiniteSet::new(data_18);
        let bounds = bounds.add_constraint(idx_18, set_18);
        
        let idx_19 = 19;
        let data_19: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_19 = FiniteSet::new(data_19);
        let bounds = bounds.add_constraint(idx_19, set_19);
        
        let idx_20 = 20;
        let xmin_20 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_20:Option<&[f64]> = Some(&[254.0,]);
        let set_20 = Rectangle::new(xmin_20, xmax_20);
        let bounds = bounds.add_constraint(idx_20, set_20);
        
        let idx_21 = 21;
        let xmin_21 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_21:Option<&[f64]> = Some(&[103.0,]);
        let set_21 = Rectangle::new(xmin_21, xmax_21);
        let bounds = bounds.add_constraint(idx_21, set_21);
        
        let idx_22 = 22;
        let data_22: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_22 = FiniteSet::new(data_22);
        let bounds = bounds.add_constraint(idx_22, set_22);
        
        let idx_23 = 23;
        let data_23: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_23 = FiniteSet::new(data_23);
        let bounds = bounds.add_constraint(idx_23, set_23);
        
        let idx_24 = 24;
        let xmin_24 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_24:Option<&[f64]> = Some(&[254.0,]);
        let set_24 = Rectangle::new(xmin_24, xmax_24);
        let bounds = bounds.add_constraint(idx_24, set_24);
        
        let idx_25 = 25;
        let xmin_25 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_25:Option<&[f64]> = Some(&[103.0,]);
        let set_25 = Rectangle::new(xmin_25, xmax_25);
        let bounds = bounds.add_constraint(idx_25, set_25);
        
        let idx_26 = 26;
        let data_26: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_26 = FiniteSet::new(data_26);
        let bounds = bounds.add_constraint(idx_26, set_26);
        
        let idx_27 = 27;
        let data_27: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_27 = FiniteSet::new(data_27);
        let bounds = bounds.add_constraint(idx_27, set_27);
        
        let idx_28 = 28;
        let xmin_28 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_28:Option<&[f64]> = Some(&[254.0,]);
        let set_28 = Rectangle::new(xmin_28, xmax_28);
        let bounds = bounds.add_constraint(idx_28, set_28);
        
        let idx_29 = 29;
        let xmin_29 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_29:Option<&[f64]> = Some(&[103.0,]);
        let set_29 = Rectangle::new(xmin_29, xmax_29);
        let bounds = bounds.add_constraint(idx_29, set_29);
        
        let idx_30 = 30;
        let data_30: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_30 = FiniteSet::new(data_30);
        let bounds = bounds.add_constraint(idx_30, set_30);
        
        let idx_31 = 31;
        let data_31: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_31 = FiniteSet::new(data_31);
        let bounds = bounds.add_constraint(idx_31, set_31);
        
        let idx_32 = 32;
        let xmin_32 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_32:Option<&[f64]> = Some(&[254.0,]);
        let set_32 = Rectangle::new(xmin_32, xmax_32);
        let bounds = bounds.add_constraint(idx_32, set_32);
        
        let idx_33 = 33;
        let xmin_33 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_33:Option<&[f64]> = Some(&[103.0,]);
        let set_33 = Rectangle::new(xmin_33, xmax_33);
        let bounds = bounds.add_constraint(idx_33, set_33);
        
        let idx_34 = 34;
        let data_34: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_34 = FiniteSet::new(data_34);
        let bounds = bounds.add_constraint(idx_34, set_34);
        
        let idx_35 = 35;
        let data_35: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_35 = FiniteSet::new(data_35);
        let bounds = bounds.add_constraint(idx_35, set_35);
        
        let idx_36 = 36;
        let xmin_36 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_36:Option<&[f64]> = Some(&[254.0,]);
        let set_36 = Rectangle::new(xmin_36, xmax_36);
        let bounds = bounds.add_constraint(idx_36, set_36);
        
        let idx_37 = 37;
        let xmin_37 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_37:Option<&[f64]> = Some(&[103.0,]);
        let set_37 = Rectangle::new(xmin_37, xmax_37);
        let bounds = bounds.add_constraint(idx_37, set_37);
        
        let idx_38 = 38;
        let data_38: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_38 = FiniteSet::new(data_38);
        let bounds = bounds.add_constraint(idx_38, set_38);
        
        let idx_39 = 39;
        let data_39: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_39 = FiniteSet::new(data_39);
        let bounds = bounds.add_constraint(idx_39, set_39);
        
        let idx_40 = 40;
        let xmin_40 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_40:Option<&[f64]> = Some(&[254.0,]);
        let set_40 = Rectangle::new(xmin_40, xmax_40);
        let bounds = bounds.add_constraint(idx_40, set_40);
        
        let idx_41 = 41;
        let xmin_41 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_41:Option<&[f64]> = Some(&[103.0,]);
        let set_41 = Rectangle::new(xmin_41, xmax_41);
        let bounds = bounds.add_constraint(idx_41, set_41);
        
        let idx_42 = 42;
        let data_42: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_42 = FiniteSet::new(data_42);
        let bounds = bounds.add_constraint(idx_42, set_42);
        
        let idx_43 = 43;
        let data_43: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_43 = FiniteSet::new(data_43);
        let bounds = bounds.add_constraint(idx_43, set_43);
        
        let idx_44 = 44;
        let xmin_44 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_44:Option<&[f64]> = Some(&[254.0,]);
        let set_44 = Rectangle::new(xmin_44, xmax_44);
        let bounds = bounds.add_constraint(idx_44, set_44);
        
        let idx_45 = 45;
        let xmin_45 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_45:Option<&[f64]> = Some(&[103.0,]);
        let set_45 = Rectangle::new(xmin_45, xmax_45);
        let bounds = bounds.add_constraint(idx_45, set_45);
        
        let idx_46 = 46;
        let data_46: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_46 = FiniteSet::new(data_46);
        let bounds = bounds.add_constraint(idx_46, set_46);
        
        let idx_47 = 47;
        let data_47: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_47 = FiniteSet::new(data_47);
        let bounds = bounds.add_constraint(idx_47, set_47);
        
        let idx_48 = 48;
        let xmin_48 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_48:Option<&[f64]> = Some(&[254.0,]);
        let set_48 = Rectangle::new(xmin_48, xmax_48);
        let bounds = bounds.add_constraint(idx_48, set_48);
        
        let idx_49 = 49;
        let xmin_49 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_49:Option<&[f64]> = Some(&[103.0,]);
        let set_49 = Rectangle::new(xmin_49, xmax_49);
        let bounds = bounds.add_constraint(idx_49, set_49);
        
        let idx_50 = 50;
        let data_50: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_50 = FiniteSet::new(data_50);
        let bounds = bounds.add_constraint(idx_50, set_50);
        
        let idx_51 = 51;
        let data_51: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_51 = FiniteSet::new(data_51);
        let bounds = bounds.add_constraint(idx_51, set_51);
        
        let idx_52 = 52;
        let xmin_52 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_52:Option<&[f64]> = Some(&[254.0,]);
        let set_52 = Rectangle::new(xmin_52, xmax_52);
        let bounds = bounds.add_constraint(idx_52, set_52);
        
        let idx_53 = 53;
        let xmin_53 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_53:Option<&[f64]> = Some(&[103.0,]);
        let set_53 = Rectangle::new(xmin_53, xmax_53);
        let bounds = bounds.add_constraint(idx_53, set_53);
        
        let idx_54 = 54;
        let data_54: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_54 = FiniteSet::new(data_54);
        let bounds = bounds.add_constraint(idx_54, set_54);
        
        let idx_55 = 55;
        let data_55: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_55 = FiniteSet::new(data_55);
        let bounds = bounds.add_constraint(idx_55, set_55);
        
        let idx_56 = 56;
        let xmin_56 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_56:Option<&[f64]> = Some(&[254.0,]);
        let set_56 = Rectangle::new(xmin_56, xmax_56);
        let bounds = bounds.add_constraint(idx_56, set_56);
        
        let idx_57 = 57;
        let xmin_57 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_57:Option<&[f64]> = Some(&[103.0,]);
        let set_57 = Rectangle::new(xmin_57, xmax_57);
        let bounds = bounds.add_constraint(idx_57, set_57);
        
        let idx_58 = 58;
        let data_58: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_58 = FiniteSet::new(data_58);
        let bounds = bounds.add_constraint(idx_58, set_58);
        
        let idx_59 = 59;
        let data_59: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_59 = FiniteSet::new(data_59);
        let bounds = bounds.add_constraint(idx_59, set_59);
        
        let idx_60 = 60;
        let xmin_60 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_60:Option<&[f64]> = Some(&[254.0,]);
        let set_60 = Rectangle::new(xmin_60, xmax_60);
        let bounds = bounds.add_constraint(idx_60, set_60);
        
        let idx_61 = 61;
        let xmin_61 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_61:Option<&[f64]> = Some(&[103.0,]);
        let set_61 = Rectangle::new(xmin_61, xmax_61);
        let bounds = bounds.add_constraint(idx_61, set_61);
        
        let idx_62 = 62;
        let data_62: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_62 = FiniteSet::new(data_62);
        let bounds = bounds.add_constraint(idx_62, set_62);
        
        let idx_63 = 63;
        let data_63: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_63 = FiniteSet::new(data_63);
        let bounds = bounds.add_constraint(idx_63, set_63);
        
        let idx_64 = 64;
        let xmin_64 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_64:Option<&[f64]> = Some(&[254.0,]);
        let set_64 = Rectangle::new(xmin_64, xmax_64);
        let bounds = bounds.add_constraint(idx_64, set_64);
        
        let idx_65 = 65;
        let xmin_65 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_65:Option<&[f64]> = Some(&[103.0,]);
        let set_65 = Rectangle::new(xmin_65, xmax_65);
        let bounds = bounds.add_constraint(idx_65, set_65);
        
        let idx_66 = 66;
        let data_66: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_66 = FiniteSet::new(data_66);
        let bounds = bounds.add_constraint(idx_66, set_66);
        
        let idx_67 = 67;
        let data_67: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_67 = FiniteSet::new(data_67);
        let bounds = bounds.add_constraint(idx_67, set_67);
        
        let idx_68 = 68;
        let xmin_68 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_68:Option<&[f64]> = Some(&[254.0,]);
        let set_68 = Rectangle::new(xmin_68, xmax_68);
        let bounds = bounds.add_constraint(idx_68, set_68);
        
        let idx_69 = 69;
        let xmin_69 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_69:Option<&[f64]> = Some(&[103.0,]);
        let set_69 = Rectangle::new(xmin_69, xmax_69);
        let bounds = bounds.add_constraint(idx_69, set_69);
        
        let idx_70 = 70;
        let data_70: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_70 = FiniteSet::new(data_70);
        let bounds = bounds.add_constraint(idx_70, set_70);
        
        let idx_71 = 71;
        let data_71: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_71 = FiniteSet::new(data_71);
        let bounds = bounds.add_constraint(idx_71, set_71);
        
        let idx_72 = 72;
        let xmin_72 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_72:Option<&[f64]> = Some(&[254.0,]);
        let set_72 = Rectangle::new(xmin_72, xmax_72);
        let bounds = bounds.add_constraint(idx_72, set_72);
        
        let idx_73 = 73;
        let xmin_73 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_73:Option<&[f64]> = Some(&[103.0,]);
        let set_73 = Rectangle::new(xmin_73, xmax_73);
        let bounds = bounds.add_constraint(idx_73, set_73);
        
        let idx_74 = 74;
        let data_74: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_74 = FiniteSet::new(data_74);
        let bounds = bounds.add_constraint(idx_74, set_74);
        
        let idx_75 = 75;
        let data_75: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_75 = FiniteSet::new(data_75);
        let bounds = bounds.add_constraint(idx_75, set_75);
        
        let idx_76 = 76;
        let xmin_76 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_76:Option<&[f64]> = Some(&[254.0,]);
        let set_76 = Rectangle::new(xmin_76, xmax_76);
        let bounds = bounds.add_constraint(idx_76, set_76);
        
        let idx_77 = 77;
        let xmin_77 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_77:Option<&[f64]> = Some(&[103.0,]);
        let set_77 = Rectangle::new(xmin_77, xmax_77);
        let bounds = bounds.add_constraint(idx_77, set_77);
        
        let idx_78 = 78;
        let data_78: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_78 = FiniteSet::new(data_78);
        let bounds = bounds.add_constraint(idx_78, set_78);
        
        let idx_79 = 79;
        let data_79: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_79 = FiniteSet::new(data_79);
        let bounds = bounds.add_constraint(idx_79, set_79);
        
        let idx_80 = 80;
        let xmin_80 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_80:Option<&[f64]> = Some(&[254.0,]);
        let set_80 = Rectangle::new(xmin_80, xmax_80);
        let bounds = bounds.add_constraint(idx_80, set_80);
        
        let idx_81 = 81;
        let xmin_81 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_81:Option<&[f64]> = Some(&[103.0,]);
        let set_81 = Rectangle::new(xmin_81, xmax_81);
        let bounds = bounds.add_constraint(idx_81, set_81);
        
        let idx_82 = 82;
        let data_82: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_82 = FiniteSet::new(data_82);
        let bounds = bounds.add_constraint(idx_82, set_82);
        
        let idx_83 = 83;
        let data_83: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_83 = FiniteSet::new(data_83);
        let bounds = bounds.add_constraint(idx_83, set_83);
        
        let idx_84 = 84;
        let xmin_84 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_84:Option<&[f64]> = Some(&[254.0,]);
        let set_84 = Rectangle::new(xmin_84, xmax_84);
        let bounds = bounds.add_constraint(idx_84, set_84);
        
        let idx_85 = 85;
        let xmin_85 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_85:Option<&[f64]> = Some(&[103.0,]);
        let set_85 = Rectangle::new(xmin_85, xmax_85);
        let bounds = bounds.add_constraint(idx_85, set_85);
        
        let idx_86 = 86;
        let data_86: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_86 = FiniteSet::new(data_86);
        let bounds = bounds.add_constraint(idx_86, set_86);
        
        let idx_87 = 87;
        let data_87: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_87 = FiniteSet::new(data_87);
        let bounds = bounds.add_constraint(idx_87, set_87);
        
        let idx_88 = 88;
        let xmin_88 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_88:Option<&[f64]> = Some(&[254.0,]);
        let set_88 = Rectangle::new(xmin_88, xmax_88);
        let bounds = bounds.add_constraint(idx_88, set_88);
        
        let idx_89 = 89;
        let xmin_89 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_89:Option<&[f64]> = Some(&[103.0,]);
        let set_89 = Rectangle::new(xmin_89, xmax_89);
        let bounds = bounds.add_constraint(idx_89, set_89);
        
        let idx_90 = 90;
        let data_90: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_90 = FiniteSet::new(data_90);
        let bounds = bounds.add_constraint(idx_90, set_90);
        
        let idx_91 = 91;
        let data_91: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_91 = FiniteSet::new(data_91);
        let bounds = bounds.add_constraint(idx_91, set_91);
        
        let idx_92 = 92;
        let xmin_92 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_92:Option<&[f64]> = Some(&[254.0,]);
        let set_92 = Rectangle::new(xmin_92, xmax_92);
        let bounds = bounds.add_constraint(idx_92, set_92);
        
        let idx_93 = 93;
        let xmin_93 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_93:Option<&[f64]> = Some(&[103.0,]);
        let set_93 = Rectangle::new(xmin_93, xmax_93);
        let bounds = bounds.add_constraint(idx_93, set_93);
        
        let idx_94 = 94;
        let data_94: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_94 = FiniteSet::new(data_94);
        let bounds = bounds.add_constraint(idx_94, set_94);
        
        let idx_95 = 95;
        let data_95: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_95 = FiniteSet::new(data_95);
        let bounds = bounds.add_constraint(idx_95, set_95);
        
        let idx_96 = 96;
        let xmin_96 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_96:Option<&[f64]> = Some(&[254.0,]);
        let set_96 = Rectangle::new(xmin_96, xmax_96);
        let bounds = bounds.add_constraint(idx_96, set_96);
        
        let idx_97 = 97;
        let xmin_97 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_97:Option<&[f64]> = Some(&[103.0,]);
        let set_97 = Rectangle::new(xmin_97, xmax_97);
        let bounds = bounds.add_constraint(idx_97, set_97);
        
        let idx_98 = 98;
        let data_98: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_98 = FiniteSet::new(data_98);
        let bounds = bounds.add_constraint(idx_98, set_98);
        
        let idx_99 = 99;
        let data_99: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_99 = FiniteSet::new(data_99);
        let bounds = bounds.add_constraint(idx_99, set_99);
        
        let idx_100 = 100;
        let xmin_100 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_100:Option<&[f64]> = Some(&[254.0,]);
        let set_100 = Rectangle::new(xmin_100, xmax_100);
        let bounds = bounds.add_constraint(idx_100, set_100);
        
        let idx_101 = 101;
        let xmin_101 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_101:Option<&[f64]> = Some(&[103.0,]);
        let set_101 = Rectangle::new(xmin_101, xmax_101);
        let bounds = bounds.add_constraint(idx_101, set_101);
        
        let idx_102 = 102;
        let data_102: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_102 = FiniteSet::new(data_102);
        let bounds = bounds.add_constraint(idx_102, set_102);
        
        let idx_103 = 103;
        let data_103: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_103 = FiniteSet::new(data_103);
        let bounds = bounds.add_constraint(idx_103, set_103);
        
        let idx_104 = 104;
        let xmin_104 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_104:Option<&[f64]> = Some(&[254.0,]);
        let set_104 = Rectangle::new(xmin_104, xmax_104);
        let bounds = bounds.add_constraint(idx_104, set_104);
        
        let idx_105 = 105;
        let xmin_105 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_105:Option<&[f64]> = Some(&[103.0,]);
        let set_105 = Rectangle::new(xmin_105, xmax_105);
        let bounds = bounds.add_constraint(idx_105, set_105);
        
        let idx_106 = 106;
        let data_106: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_106 = FiniteSet::new(data_106);
        let bounds = bounds.add_constraint(idx_106, set_106);
        
        let idx_107 = 107;
        let data_107: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_107 = FiniteSet::new(data_107);
        let bounds = bounds.add_constraint(idx_107, set_107);
        
        let idx_108 = 108;
        let xmin_108 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_108:Option<&[f64]> = Some(&[254.0,]);
        let set_108 = Rectangle::new(xmin_108, xmax_108);
        let bounds = bounds.add_constraint(idx_108, set_108);
        
        let idx_109 = 109;
        let xmin_109 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_109:Option<&[f64]> = Some(&[103.0,]);
        let set_109 = Rectangle::new(xmin_109, xmax_109);
        let bounds = bounds.add_constraint(idx_109, set_109);
        
        let idx_110 = 110;
        let data_110: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_110 = FiniteSet::new(data_110);
        let bounds = bounds.add_constraint(idx_110, set_110);
        
        let idx_111 = 111;
        let data_111: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_111 = FiniteSet::new(data_111);
        let bounds = bounds.add_constraint(idx_111, set_111);
        
        let idx_112 = 112;
        let xmin_112 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_112:Option<&[f64]> = Some(&[254.0,]);
        let set_112 = Rectangle::new(xmin_112, xmax_112);
        let bounds = bounds.add_constraint(idx_112, set_112);
        
        let idx_113 = 113;
        let xmin_113 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_113:Option<&[f64]> = Some(&[103.0,]);
        let set_113 = Rectangle::new(xmin_113, xmax_113);
        let bounds = bounds.add_constraint(idx_113, set_113);
        
        let idx_114 = 114;
        let data_114: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_114 = FiniteSet::new(data_114);
        let bounds = bounds.add_constraint(idx_114, set_114);
        
        let idx_115 = 115;
        let data_115: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_115 = FiniteSet::new(data_115);
        let bounds = bounds.add_constraint(idx_115, set_115);
        
        let idx_116 = 116;
        let xmin_116 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_116:Option<&[f64]> = Some(&[254.0,]);
        let set_116 = Rectangle::new(xmin_116, xmax_116);
        let bounds = bounds.add_constraint(idx_116, set_116);
        
        let idx_117 = 117;
        let xmin_117 :Option<&[f64]> = Some(&[0.0,]);
        let xmax_117:Option<&[f64]> = Some(&[103.0,]);
        let set_117 = Rectangle::new(xmin_117, xmax_117);
        let bounds = bounds.add_constraint(idx_117, set_117);
        
        let idx_118 = 118;
        let data_118: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_118 = FiniteSet::new(data_118);
        let bounds = bounds.add_constraint(idx_118, set_118);
        
        let idx_119 = 119;
        let data_119: &[&[f64]] = &[&[0.0],&[1.0],];
        let set_119 = FiniteSet::new(data_119);
        let bounds = bounds.add_constraint(idx_119, set_119);
        
        let idx_120 = 120;
        let xmin_120 :Option<&[f64]> = Some(&[-254.0,]);
        let xmax_120:Option<&[f64]> = Some(&[254.0,]);
        let set_120 = Rectangle::new(xmin_120, xmax_120);
        let bounds = bounds.add_constraint(idx_120, set_120);
        
    bounds
    }

/// Make set C
fn make_set_c() -> impl Constraint {
    Rectangle::new(SET_C_XMIN, SET_C_XMAX)
    }


/// Make set Y
fn make_set_y() -> impl Constraint {
    Rectangle::new(SET_Y_XMIN, SET_Y_XMAX)
    }


// ---Main public API functions--------------------------------------------------------------------------


/// Initialisation of the solver
pub fn initialize_solver() -> AlmCache {
    let panoc_cache = PANOCCache::new(GUAN_OPTIMIZER_NUM_DECISION_VARIABLES, EPSILON_TOLERANCE, LBFGS_MEMORY);
    AlmCache::new(panoc_cache, GUAN_OPTIMIZER_N1, GUAN_OPTIMIZER_N2)
}

/// If preconditioning has been applied, then at the end (after a solution has been obtained)
/// we need to undo the scaling and update the cost function
fn unscale_result(solver_status: &mut Result<AlmOptimizerStatus, SolverError>) {
    if let Ok(sss) = solver_status {
        let w_cost : f64 = icasadi_guan_optimizer::get_w_cost();
        sss.update_cost(sss.cost() / w_cost);
    }
}

/// Solver interface
///
/// ## Arguments
/// - `p`: static parameter vector of the optimization problem
/// - `alm_cache`: Instance of AlmCache
/// - `u`: Initial guess
/// - `y0` (optional) initial vector of Lagrange multipliers
/// - `c0` (optional) initial penalty
///
/// ## Returns
/// This function returns either an instance of AlmOptimizerStatus with information about the
/// solution, or a SolverError object if something goes wrong
pub fn solve(
    p: &[f64],
    alm_cache: &mut AlmCache,
    u: &mut [f64],
    y0: &Option<Vec<f64>>,
    c0: &Option<f64>,
) -> Result<AlmOptimizerStatus, SolverError> {

    assert_eq!(p.len(), GUAN_OPTIMIZER_NUM_PARAMETERS, "Wrong number of parameters (p)");
    assert_eq!(u.len(), GUAN_OPTIMIZER_NUM_DECISION_VARIABLES, "Wrong number of decision variables (u)");

    // Start by initialising the optimiser interface (e.g., set w=1)
    icasadi_guan_optimizer::init_guan_optimizer();

    let mut rho_init : f64 = 1.0;
    if DO_PRECONDITIONING {
        // Compute the preconditioning parameters (w's)
        // The scaling parameters will be stored internally in `interface.c`
        icasadi_guan_optimizer::precondition(u, p);

        // Compute initial penalty
        icasadi_guan_optimizer::initial_penalty(u, p, & mut rho_init);
    }

    let psi = |u: &[f64], xi: &[f64], cost: &mut f64| -> Result<(), SolverError> {
        icasadi_guan_optimizer::cost(u, xi, p, cost);
        Ok(())
    };
    let grad_psi = |u: &[f64], xi: &[f64], grad: &mut [f64]| -> Result<(), SolverError> {
        icasadi_guan_optimizer::grad(u, xi, p, grad);
        Ok(())
    };
    
    let f1 = |u: &[f64], res: &mut [f64]| -> Result<(), SolverError> {
        icasadi_guan_optimizer::mapping_f1(u, p, res);
        Ok(())
    };
    let f2 = |u: &[f64], res: &mut [f64]| -> Result<(), SolverError> {
        icasadi_guan_optimizer::mapping_f2(u, p, res);
        Ok(())
    };let bounds = make_constraints();

    let set_y = make_set_y();
    let set_c = make_set_c();
    let alm_problem = AlmProblem::new(
        bounds,
        Some(set_c),
        Some(set_y),
        psi,
        grad_psi,
        Some(f1),
        Some(f2),
        GUAN_OPTIMIZER_N1,
        GUAN_OPTIMIZER_N2,
    );

    let mut alm_optimizer = AlmOptimizer::new(alm_cache, alm_problem)
        .with_delta_tolerance(DELTA_TOLERANCE)
        .with_epsilon_tolerance(EPSILON_TOLERANCE)
        .with_initial_inner_tolerance(INITIAL_EPSILON_TOLERANCE)
        .with_inner_tolerance_update_factor(EPSILON_TOLERANCE_UPDATE_FACTOR)
        .with_max_duration(std::time::Duration::from_micros(MAX_DURATION_MICROS))
        .with_max_outer_iterations(MAX_OUTER_ITERATIONS)
        .with_max_inner_iterations(MAX_INNER_ITERATIONS)
        .with_initial_penalty(c0.unwrap_or(INITIAL_PENALTY_PARAMETER.unwrap_or(rho_init)))
        .with_penalty_update_factor(PENALTY_UPDATE_FACTOR)
        .with_sufficient_decrease_coefficient(SUFFICIENT_INFEASIBILITY_DECREASE_COEFFICIENT);

    // solve the problem using `u`, the initial condition `u`, and
    // initial vector of Lagrange multipliers, if provided;
    // returns the problem status (instance of `AlmOptimizerStatus`)
    if let Some(y0_) = y0 {
        let mut alm_optimizer = alm_optimizer.with_initial_lagrange_multipliers(y0_);
        let mut solution_status = alm_optimizer.solve(u);
        unscale_result(&mut solution_status);
        solution_status
    } else {
        let mut solution_status = alm_optimizer.solve(u);
        unscale_result(&mut solution_status);
        solution_status
    }

}