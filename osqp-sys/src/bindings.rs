/* automatically generated by rust-bindgen 0.55.1 */

use {c_float, c_int};

pub const OSQP_SOLVED: osqp_status_type = 1;
pub const OSQP_SOLVED_INACCURATE: osqp_status_type = 2;
pub const OSQP_PRIMAL_INFEASIBLE: osqp_status_type = 3;
pub const OSQP_PRIMAL_INFEASIBLE_INACCURATE: osqp_status_type = 4;
pub const OSQP_DUAL_INFEASIBLE: osqp_status_type = 5;
pub const OSQP_DUAL_INFEASIBLE_INACCURATE: osqp_status_type = 6;
pub const OSQP_MAX_ITER_REACHED: osqp_status_type = 7;
pub const OSQP_TIME_LIMIT_REACHED: osqp_status_type = 8;
pub const OSQP_NON_CVX: osqp_status_type = 9;
pub const OSQP_SIGINT: osqp_status_type = 10;
pub const OSQP_UNSOLVED: osqp_status_type = 11;
#[doc = " Solver Status  *"]
pub type osqp_status_type = ::std::os::raw::c_uint;
pub const OSQP_UNKNOWN_SOLVER: osqp_linsys_solver_type = 0;
pub const OSQP_DIRECT_SOLVER: osqp_linsys_solver_type = 1;
pub const OSQP_INDIRECT_SOLVER: osqp_linsys_solver_type = 2;
#[doc = " Linear System Solvers *"]
pub type osqp_linsys_solver_type = ::std::os::raw::c_uint;
pub const OSQP_DATA_VALIDATION_ERROR: osqp_error_type = 1;
pub const OSQP_SETTINGS_VALIDATION_ERROR: osqp_error_type = 2;
pub const OSQP_LINSYS_SOLVER_INIT_ERROR: osqp_error_type = 3;
pub const OSQP_NONCVX_ERROR: osqp_error_type = 4;
pub const OSQP_MEM_ALLOC_ERROR: osqp_error_type = 5;
pub const OSQP_WORKSPACE_NOT_INIT_ERROR: osqp_error_type = 6;
pub const OSQP_ALGEBRA_LOAD_ERROR: osqp_error_type = 7;
#[doc = " Solver Errors  *"]
pub type osqp_error_type = ::std::os::raw::c_uint;
#[doc = " User settings"]
#[repr(C)]
pub struct OSQPSettings {
    #[doc = "< device identifier; currently used for CUDA devices"]
    pub device: c_int,
    #[doc = "< linear system solver to use"]
    pub linsys_solver: osqp_linsys_solver_type,
    #[doc = "< boolean; write out progress"]
    pub verbose: c_int,
    #[doc = "< boolean; warm start"]
    pub warm_starting: c_int,
    #[doc = "< data scaling iterations; if 0, then disabled"]
    pub scaling: c_int,
    #[doc = "< boolean; polish ADMM solution"]
    pub polishing: c_int,
    #[doc = "< ADMM penalty parameter"]
    pub rho: c_float,
    #[doc = "< boolean; is rho scalar or vector?"]
    pub rho_is_vec: c_int,
    #[doc = "< ADMM penalty parameter"]
    pub sigma: c_float,
    #[doc = "< ADMM relaxation parameter"]
    pub alpha: c_float,
    #[doc = "< maximum number of CG iterations per solve"]
    pub cg_max_iter: c_int,
    #[doc = "< number of consecutive zero CG iterations before the tolerance gets halved"]
    pub cg_tol_reduction: c_int,
    #[doc = "< CG tolerance (fraction of ADMM residuals)"]
    pub cg_tol_fraction: c_float,
    #[doc = "< boolean, is rho step size adaptive?"]
    pub adaptive_rho: c_int,
    #[doc = "< number of iterations between rho adaptations; if 0, then it is timing-based"]
    pub adaptive_rho_interval: c_int,
    #[doc = "< time interval for adapting rho (fraction of the setup time)"]
    pub adaptive_rho_fraction: c_float,
    #[doc = "< tolerance X for adapting rho; new rho must be X times larger or smaller than the current one to change it"]
    pub adaptive_rho_tolerance: c_float,
    #[doc = "< maximum number of iterations"]
    pub max_iter: c_int,
    #[doc = "< absolute solution tolerance"]
    pub eps_abs: c_float,
    #[doc = "< relative solution tolerance"]
    pub eps_rel: c_float,
    #[doc = "< primal infeasibility tolerance"]
    pub eps_prim_inf: c_float,
    #[doc = "< dual infeasibility tolerance"]
    pub eps_dual_inf: c_float,
    #[doc = "< boolean; use scaled termination criteria"]
    pub scaled_termination: c_int,
    #[doc = "< integer, check termination interval; if 0, checking is disabled"]
    pub check_termination: c_int,
    #[doc = "< maximum time to solve the problem (seconds)"]
    pub time_limit: c_float,
    #[doc = "< regularization parameter for polishing"]
    pub delta: c_float,
    #[doc = "< number of iterative refinement steps in polishing"]
    pub polish_refine_iter: c_int,
}
#[doc = " Solver return information"]
#[repr(C)]
pub struct OSQPInfo {
    #[doc = "< status string, e.g. 'solved'"]
    pub status: [::std::os::raw::c_char; 32usize],
    #[doc = "< status as c_int, defined in osqp_api_constants.h"]
    pub status_val: c_int,
    #[doc = "< polishing status: successful (1), unperformed (0), unsuccessful (-1)"]
    pub status_polish: c_int,
    #[doc = "< primal objective"]
    pub obj_val: c_float,
    #[doc = "< norm of primal residual"]
    pub prim_res: c_float,
    #[doc = "< norm of dual residual"]
    pub dual_res: c_float,
    #[doc = "< number of iterations taken"]
    pub iter: c_int,
    #[doc = "< number of rho updates"]
    pub rho_updates: c_int,
    #[doc = "< best rho estimate so far from residuals"]
    pub rho_estimate: c_float,
    #[doc = "< setup  phase time (seconds)"]
    pub setup_time: c_float,
    #[doc = "< solve  phase time (seconds)"]
    pub solve_time: c_float,
    #[doc = "< update phase time (seconds)"]
    pub update_time: c_float,
    #[doc = "< polish phase time (seconds)"]
    pub polish_time: c_float,
    #[doc = "< total time  (seconds)"]
    pub run_time: c_float,
}
#[doc = " Solution structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OSQPSolution {
    #[doc = "< primal solution"]
    pub x: *mut c_float,
    #[doc = "< Lagrange multiplier associated with \\f$l \\le Ax \\le u\\f$"]
    pub y: *mut c_float,
    #[doc = "< primal infeasibility certificate"]
    pub prim_inf_cert: *mut c_float,
    #[doc = "< dual infeasibility certificate"]
    pub dual_inf_cert: *mut c_float,
}
pub type OSQPWorkspace = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OSQPSolver {
    #[doc = "< problem settings"]
    pub settings: *mut OSQPSettings,
    #[doc = "< problem solution"]
    pub solution: *mut OSQPSolution,
    #[doc = "< solver information"]
    pub info: *mut OSQPInfo,
    #[doc = "< solver internal workspace"]
    pub work: *mut OSQPWorkspace,
}
#[doc = "  Matrix in compressed-column form."]
#[doc = "  The structure is used internally to store matrices in the triplet form as well,"]
#[doc = "  but the API requires that the matrices are in the CSC format."]
#[repr(C)]
pub struct csc {
    #[doc = "< number of rows"]
    pub m: c_int,
    #[doc = "< number of columns"]
    pub n: c_int,
    #[doc = "< column pointers (size n+1); col indices (size nzmax) starting from 0 for triplet format"]
    pub p: *mut c_int,
    #[doc = "< row indices, size nzmax starting from 0"]
    pub i: *mut c_int,
    #[doc = "< numerical values, size nzmax"]
    pub x: *mut c_float,
    #[doc = "< maximum number of entries"]
    pub nzmax: c_int,
    #[doc = "< number of entries in triplet matrix, -1 for csc"]
    pub nz: c_int,
}
extern "C" {
    #[doc = " Set default settings from osqp_api_constants.h file."]
    #[doc = " Assumes settings already allocated in memory."]
    #[doc = " @param settings OSQPSettings structure"]
    pub fn osqp_set_default_settings(settings: *mut OSQPSettings);
}
extern "C" {
    #[doc = " Initialize OSQP solver allocating memory."]
    #[doc = ""]
    #[doc = " It performs:"]
    #[doc = " - data and settings validation"]
    #[doc = " - problem data scaling"]
    #[doc = " - setup linear system solver:"]
    #[doc = "      - direct solver: KKT matrix factorization is performed here"]
    #[doc = "      - indirect solver: reduced KKT matrix preconditioning is performed here"]
    #[doc = ""]
    #[doc = " NB: This is the only function that allocates dynamic memory and is not used"]
    #[doc = " during code generation"]
    #[doc = ""]
    #[doc = " @param  solverp   Solver pointer"]
    #[doc = " @param  P         Problem data (upper triangular part of quadratic cost term, csc format)"]
    #[doc = " @param  q         Problem data (linear cost term)"]
    #[doc = " @param  A         Problem data (constraint matrix, csc format)"]
    #[doc = " @param  l         Problem data (constraint lower bound)"]
    #[doc = " @param  u         Problem data (constraint upper bound)"]
    #[doc = " @param  m         Problem data (number of constraints)"]
    #[doc = " @param  n         Problem data (number of variables)"]
    #[doc = " @param  settings  Solver settings"]
    #[doc = " @return           Exitflag for errors (0 if no errors)"]
    pub fn osqp_setup(
        solverp: *mut *mut OSQPSolver,
        P: *const csc,
        q: *const c_float,
        A: *const csc,
        l: *const c_float,
        u: *const c_float,
        m: c_int,
        n: c_int,
        settings: *const OSQPSettings,
    ) -> c_int;
}
extern "C" {
    #[doc = " Solve quadratic program"]
    #[doc = ""]
    #[doc = " The final solver information is stored in the  \\a solver->info  structure"]
    #[doc = ""]
    #[doc = " The solution is stored in the  \\a solver->solution  structure"]
    #[doc = ""]
    #[doc = " If the problem is primal infeasible, the certificate is stored"]
    #[doc = " in \\a solver->delta_y"]
    #[doc = ""]
    #[doc = " If the problem is dual infeasible, the certificate is stored in \\a"]
    #[doc = " solver->delta_x"]
    #[doc = ""]
    #[doc = " @param  solver Solver"]
    #[doc = " @return        Exitflag for errors (0 if no errors)"]
    pub fn osqp_solve(solver: *mut OSQPSolver) -> c_int;
}
extern "C" {
    #[doc = " Cleanup workspace by deallocating memory"]
    #[doc = ""]
    #[doc = " This function is not used in code generation"]
    #[doc = " @param  solver Solver"]
    #[doc = " @return        Exitflag for errors (0 if no errors)"]
    pub fn osqp_cleanup(solver: *mut OSQPSolver) -> c_int;
}
extern "C" {
    #[doc = " Warm start primal and dual variables"]
    #[doc = " @param  solver Solver"]
    #[doc = " @param  x      Primal variable, NULL if none"]
    #[doc = " @param  y      Dual variable, NULL if none"]
    #[doc = " @return        Exitflag for errors (0 if no errors)"]
    pub fn osqp_warm_start(solver: *mut OSQPSolver, x: *const c_float, y: *const c_float) -> c_int;
}
extern "C" {
    #[doc = " Update problem data vectors"]
    #[doc = " @param  solver  Solver"]
    #[doc = " @param  q_new   New linear cost, NULL if none"]
    #[doc = " @param  l_new   New lower bound, NULL if none"]
    #[doc = " @param  u_new   New upper bound, NULL if none"]
    #[doc = " @return         Exitflag for errors (0 if no errors)"]
    pub fn osqp_update_data_vec(
        solver: *mut OSQPSolver,
        q_new: *const c_float,
        l_new: *const c_float,
        u_new: *const c_float,
    ) -> c_int;
}
extern "C" {
    #[doc = " Update elements of matrices P (upper triangular) and A by preserving"]
    #[doc = " their sparsity structures."]
    #[doc = ""]
    #[doc = " If Px_new_idx (Ax_new_idx) is OSQP_NULL, Px_new (Ax_new) is assumed"]
    #[doc = " to be as long as P->x (A->x) and the whole P->x (A->x) is replaced."]
    #[doc = ""]
    #[doc = " @param  solver     Solver"]
    #[doc = " @param  Px_new     Vector of new elements in P->x (upper triangular), NULL if none"]
    #[doc = " @param  Px_new_idx Index mapping new elements to positions in P->x"]
    #[doc = " @param  P_new_n    Number of new elements to be changed"]
    #[doc = " @param  Ax_new     Vector of new elements in A->x, NULL if none"]
    #[doc = " @param  Ax_new_idx Index mapping new elements to positions in A->x"]
    #[doc = " @param  A_new_n    Number of new elements to be changed"]
    #[doc = " @return            output flag:  0: OK"]
    #[doc = "                                  1: P_new_n > nnzP"]
    #[doc = "                                  2: A_new_n > nnzA"]
    #[doc = "                                 <0: error in the update"]
    pub fn osqp_update_data_mat(
        solver: *mut OSQPSolver,
        Px_new: *const c_float,
        Px_new_idx: *const c_int,
        P_new_n: c_int,
        Ax_new: *const c_float,
        Ax_new_idx: *const c_int,
        A_new_n: c_int,
    ) -> c_int;
}
extern "C" {
    #[doc = " Update settings. The following settings can only be set using"]
    #[doc = " osqp_setup and are ignored in this function:"]
    #[doc = "  - scaling"]
    #[doc = "  - rho_is_vec"]
    #[doc = "  - sigma"]
    #[doc = "  - adaptive_rho"]
    #[doc = "  - adaptive_rho_interval"]
    #[doc = "  - adaptive_rho_fraction"]
    #[doc = "  - adaptive_rho_tolerance"]
    #[doc = " Also, rho can be updated using osqp_update_rho and is ignored"]
    #[doc = " in this function."]
    #[doc = " @param  solver       Solver"]
    #[doc = " @param  new_settings Solver settings"]
    #[doc = " @return              Exitflag for errors (0 if no errors)"]
    pub fn osqp_update_settings(
        solver: *mut OSQPSolver,
        new_settings: *const OSQPSettings,
    ) -> c_int;
}
extern "C" {
    #[doc = " Update the ADMM parameter rho."]
    #[doc = " Limit it between OSQP_RHO_MIN and OSQP_RHO_MAX."]
    #[doc = " @param  solver  Solver"]
    #[doc = " @param  rho_new New rho setting"]
    #[doc = " @return         Exitflag for errors (0 if no errors)"]
    pub fn osqp_update_rho(solver: *mut OSQPSolver, rho_new: c_float) -> c_int;
}
