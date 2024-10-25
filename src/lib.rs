use crate::trnsys::param::TrnsysValue;
use crate::trnsys_type::TrnSysState;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use trnsys::*;

mod storage;
mod trnsys;
mod trnsys_type;

include!(concat!(env!("OUT_DIR"), "/generated_entrance.rs"));

static TRNSYS_STATE_DICT: LazyLock<RwLock<HashMap<i32, Arc<RwLock<TrnSysState>>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

fn get_current_state() -> Arc<RwLock<TrnSysState>> {
    let mut dict = (&TRNSYS_STATE_DICT).write().unwrap();
    let unit = get_current_unit();

    dict.entry(unit)
        .or_insert(Arc::new(RwLock::new(TrnSysState::new())))
        .clone()
}

#[no_mangle]
fn entrance() {
    // create type instance

    let state_lock = get_current_state();
    let mut state = state_lock.write().unwrap();

    if is_version_signing_time() {
        set_type_version(state.trnsys_standard_version);
        return;
    } else if is_first_call_of_simulation() {
        // Tell the TRNSYS Engine How This Type Works
        set_number_of_parameters(state.num_params);
        set_number_of_inputs(state.num_inputs);
        set_number_of_derivatives(state.num_derivatives);
        set_number_of_outputs(state.num_outputs);
        set_iteration_mode(state.iteration_mode.into());
        // set_number_stored_variables(state.num_stored_variables.0, state.num_stored_variables.1);

        state.first_call_of_simulation();
        return;
    }

    read_parameters(&mut state);
    // read_storage(&mut state);

    if is_last_call_of_simulation() {
        state.simulation_ends();
        return;
    } else if is_end_of_timestep() {
        state.end_of_timestep();
        return;
    } else if is_start_time() {
        // Read in the Values of the Parameters from the Input File
        read_parameters(&mut state);

        // validate parameters
        state.validate_parameters();
        if error_found() {
            return;
        }
        // initialize outputs
        state.initialize_outputs();

        state.simulation_starts();
        return;
    }

    if is_reread_parameters() {
        read_parameters(&mut state);
        // read_storage(&mut state);
    }
    // Perform All the Calculations Here
    let simulation_results = state.iterate();
    // set output
    simulation_results.iter().enumerate().for_each(|(i, val)| {
        // attention: TRNSYS/Fortran is 1-indexed
        set_output_value(i as i32 + 1, val.value);
    });
}

fn read_parameters(state: &mut TrnSysState) {
    let num_params = get_number_of_parameters();
    state.params = (1..num_params + 1)
        .map(|i| TrnsysValue {
            value: get_parameter_value(i),
        })
        .collect();
    let num_inputs = get_number_of_inputs();
    state.inputs = (1..num_inputs + 1)
        .map(|i| TrnsysValue {
            value: get_input_value(i),
        })
        .collect();
    let num_outputs = get_number_of_outputs();
    state.outputs = (1..num_outputs + 1)
        .map(|i| TrnsysValue {
            value: get_output_value(i),
        })
        .collect();
}

// fn read_storage(
//     state: &mut TrnSysState
// ) {
//     let num_static_store: i32 = state.num_stored_variables.0;
//     let num_dynamic_store: i32 = state.num_stored_variables.1;
//
//     state.static_store = (1..num_static_store + 1)
//         .map(|i| TrnsysValue {
//             value: get_static_array_value(i)
//         })
//         .collect();
//
//     state.variable_store = (1..num_dynamic_store + 1)
//         .map(|i| TrnsysValue {
//             value: get_dynamic_array_value_last_timestep(i)
//         })
//         .collect();
// }
