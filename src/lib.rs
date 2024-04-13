use trnsys::*;

use crate::trnsys::param::TrnsysValue;

mod trnsys;
mod trnsys_type;




#[no_mangle]
pub extern "C" fn TYPE256() {
    // create type instance

    let mut trnsys_type = trnsys_type::TrnsysType::new();

    if is_version_signing_time() {
        set_type_version(trnsys_type.trnsys_standard_version);
        return;
    } else if is_first_call_of_simulation() {
        // Tell the TRNSYS Engine How This Type Works
        set_number_of_parameters(trnsys_type.num_params);
        set_number_of_inputs(trnsys_type.num_inputs);
        set_number_of_derivatives(trnsys_type.num_derivatives);
        set_number_of_outputs(trnsys_type.num_outputs);
        set_iteration_mode(trnsys_type.iteration_mode.into());
        set_number_stored_variables(trnsys_type.num_stored_variables.0, trnsys_type.num_stored_variables.1);

        trnsys_type.first_call_of_simulation();
        return;
    }


    read_parameters(&mut trnsys_type.params, &mut trnsys_type.inputs, &mut trnsys_type.outputs);
    read_storage(
        &mut trnsys_type.static_store,
        &mut trnsys_type.variable_store,
        trnsys_type.num_stored_variables.0,
        trnsys_type.num_stored_variables.1,
    );

    if is_last_call_of_simulation() {
        trnsys_type.simulation_ends();
        return;
    } else if is_end_of_timestep() {
        trnsys_type.end_of_timestep();
        return;
    } else if is_start_time() {
        // Read in the Values of the Parameters from the Input File
        read_parameters(&mut trnsys_type.params, &mut trnsys_type.inputs, &mut trnsys_type.outputs);

        // validate parameters
        trnsys_type.validate_parameters();
        if error_found() {
            return;
        }
        // initialize outputs
        trnsys_type.initialize_outputs();

        trnsys_type.simulation_starts();
        return;
    }

    if is_reread_parameters() {
        read_parameters(&mut trnsys_type.params, &mut trnsys_type.inputs, &mut trnsys_type.outputs);
        read_storage(
            &mut trnsys_type.static_store,
            &mut trnsys_type.variable_store,
            trnsys_type.num_stored_variables.0,
            trnsys_type.num_stored_variables.1,
        );
    }
    // Perform All the Calculations Here
    let simulation_results = trnsys_type.iterate();
    // set output
    simulation_results.iter().enumerate().for_each(|(i, val)| {
        // attention: TRNSYS/Fortran is 1-indexed
        set_output_value(i as i32 + 1, val.value);
    });
}


fn read_parameters(
    params: &mut Vec<TrnsysValue>,
    inputs: &mut Vec<TrnsysValue>,
    outputs: &mut Vec<TrnsysValue>) {
    let num_params = get_number_of_parameters();
    *params = (1..num_params + 1)
        .map(|i| TrnsysValue {
            value: get_parameter_value(i)
        })
        .collect();
    let num_inputs = get_number_of_inputs();
    *inputs = (1..num_inputs + 1)
        .map(|i| TrnsysValue {
            value: get_input_value(i)
        })
        .collect();
    let num_outputs = get_number_of_outputs();
    *outputs = (1..num_outputs + 1)
        .map(|i| TrnsysValue {
            value: get_output_value(i)
        })
        .collect();
}

fn read_storage(
    static_store: &mut Vec<TrnsysValue>,
    dynamic_store: &mut Vec<TrnsysValue>,
    num_static_store: i32,
    num_dynamic_store: i32,
) {
    *static_store = (1..num_static_store + 1)
        .map(|i| TrnsysValue {
            value: get_static_array_value(i)
        })
        .collect();

    *dynamic_store = (1..num_dynamic_store + 1)
        .map(|i| TrnsysValue {
            value: get_dynamic_array_value_last_timestep(i)
        })
        .collect();
}