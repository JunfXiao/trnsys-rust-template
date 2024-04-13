use crate::trnsys::param::TrnsysValue;
use crate::trnsys::iteration_mode::IterationMode;
use crate::trnsys::*;

pub struct TrnsysType {
    pub(crate) trnsys_standard_version: i32,
    pub(crate) num_params: i32,
    pub(crate) params: Vec<TrnsysValue>,
    pub(crate) num_inputs: i32,
    pub(crate) inputs: Vec<TrnsysValue>,
    pub(crate) num_derivatives: i32,
    pub(crate) num_outputs: i32,
    pub(crate) default_output_values: Vec<TrnsysValue>,
    pub(crate) outputs: Vec<TrnsysValue>,
    pub(crate) iteration_mode: IterationMode,
    /// The number of stored variables （static, dynamic）
    pub(crate) num_stored_variables: (i32, i32),
    pub(crate) static_store: Vec<TrnsysValue>,
    pub(crate) variable_store: Vec<TrnsysValue>,

}

impl TrnsysType {
    /// set up parameters for the TRNSYS type
    pub fn new() -> Self {
        TrnsysType {
            trnsys_standard_version: 17,
            iteration_mode: IterationMode::default(),

            num_params: 0,
            params: vec![],

            num_inputs: 0,
            inputs: vec![],

            num_derivatives: 0,

            num_outputs: 0,
            outputs: vec![],
            default_output_values: vec![],

            num_stored_variables: (0, 0),
            static_store: vec![],
            variable_store: vec![],
        }
    }

    /// The very first call of the simulation.
    /// At this time, **no parameters, inputs, or outputs are available**.
    pub fn first_call_of_simulation(&self) {
        // All the "Very First Call of the Simulation Manipulations"
    }

    /// Validate the input parameters.
    /// If not valid, call `found_bad_input` or `found_bad_parameter` to stop the simulation.
    pub fn validate_parameters(&self) {
        // Validate the parameters
    }
    /// This function is called at the beginning of each simulation.
    /// Do start calculations here and store the results in the static store
    pub fn simulation_starts(&self) {

    }


    /// Whether the simulation ends correctly or ends in error, each Type is recalled by the TRNSYS
    /// kernel before the simulation shuts down.
    pub fn simulation_ends(&self) {
        // Do All of the Last Call Manipulations Here
    }

    /// The TRNSYS kernel calls this function at each time step. \
    /// This function will be called one or more times at each time step. \
    /// This function should return the values of the outputs for the current time step. \
    /// TrnSys will take care of the convergence of the simulation.
    pub fn iterate(&mut self) -> Vec<TrnsysValue> {
        let time = get_simulation_time();
        let timestep = get_simulation_time_step();
        let current_unit = get_current_unit();
        let current_type = get_current_type();

        vec![]
    }

    /// At the end of each time step, each Type in a simulation is recalled.
    /// If necessary, store the values of the outputs for the current time step
    /// in the dynamic storage
    pub fn end_of_timestep(&self) {
        // Perform Any "End of Timestep" Manipulations That May Be Required

    }




    pub fn initialize_outputs(&self) {
        // initialize output values
        self.default_output_values.iter().enumerate().for_each(|(i, val)| {
            // attention: TRNSYS/Fortran is 1-indexed
            set_output_value(i as i32 + 1, val.value);
        });
    }
}