
#![allow(unused)]
mod ext_c;
mod util;
pub(super) mod param;
pub(super) mod iteration_mode;

use util::c_bool;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ffi::{CStr, CString};


// This file declares all the global functions available to C / C++ TRNSYS Types

pub fn found_bad_input(input: &mut i32, severity: &mut [u8], message: &mut [u8]) {
    unsafe {
        ext_c::FOUNDBADINPUT(input, severity.as_mut_ptr() as *mut c_char, message.as_mut_ptr() as *mut c_char, severity.len(), message.len());
    }
}

pub fn found_bad_parameter(param: &mut i32, severity: &mut [u8], message: &mut [u8]) {
    unsafe {
        ext_c::FOUNDBADPARAMETER(param, severity.as_mut_ptr() as *mut c_char, message.as_mut_ptr() as *mut c_char, severity.len(), message.len());
    }
}


pub fn init_report_integral(index: &mut i32, int_name: &str, inst_unit: &str, int_unit: &str) {
    unsafe {
        let int_name = CString::new(int_name).unwrap();
        let inst_unit = CString::new(inst_unit).unwrap();
        let int_unit = CString::new(int_unit).unwrap();
        ext_c::INITREPORTINTEGRAL(index, int_name.as_ptr() as *mut c_char, inst_unit.as_ptr() as *mut c_char, int_unit.as_ptr() as *mut c_char, int_name.to_bytes().len(), inst_unit.to_bytes().len(), int_unit.to_bytes().len());
    }
}


pub fn init_report_min_max(index: &mut i32, minmax_name: &str, minmax_unit: &str) {
    unsafe {
        let minmax_name = CString::new(minmax_name).unwrap();
        let minmax_unit = CString::new(minmax_unit).unwrap();
        ext_c::INITREPORTMINMAX(index, minmax_name.as_ptr() as *mut c_char, minmax_unit.as_ptr() as *mut c_char, minmax_name.to_bytes().len(), minmax_unit.to_bytes().len());
    }
}


pub fn init_report_text(index: &mut i32, txt_name: &str, txt_val: &str) {
    unsafe {
        let txt_name = CString::new(txt_name).unwrap();
        let txt_val = CString::new(txt_val).unwrap();
        ext_c::INITREPORTTEXT(index, txt_name.as_ptr() as *mut c_char, txt_val.as_ptr() as *mut c_char, txt_name.to_bytes().len(), txt_val.to_bytes().len());
    }
}


pub fn init_report_value(index: &mut i32, val_name: &str, val_val: &f64, val_unit: &str) {
    unsafe {
        let val_name = CString::new(val_name).unwrap();
        let val_unit = CString::new(val_unit).unwrap();
        let mut val_val = *val_val as c_double;
        ext_c::INITREPORTVALUE(index, val_name.as_ptr() as *mut c_char, &mut val_val, val_unit.as_ptr() as *mut c_char, val_name.to_bytes().len(), val_unit.to_bytes().len());
    }
}


pub fn read_next_char(lun: &mut i32) -> i32 {
    unsafe {
        ext_c::READNEXTCHAR(lun)
    }
}


pub fn set_desired_discrete_control_state(mut i: i32, mut j: i32) {
    unsafe {
        ext_c::SETDESIREDDISCRETECONTROLSTATE(&mut i, &mut j)
    }
}


pub fn set_dynamic_array_initial_value(mut i: i32, mut value: f64) {
    unsafe {
        ext_c::SETDYNAMICARRAYINITIALVALUE(&mut i, &mut value)
    }
}


pub fn set_dynamic_array_value_this_iteration(mut i: i32, mut value: f64) {
    unsafe {
        ext_c::SETDYNAMICARRAYVALUETHISITERATION(&mut i, &mut value)
    }
}


pub fn set_input_units(mut i: i32, string: &str) {
    unsafe {
        let string = CString::new(string).unwrap();
        ext_c::SETINPUTUNITS(&mut i, string.as_ptr() as *mut c_char, string.to_bytes().len());
    }
}


pub fn set_iteration_mode(mut i: i32) {
    unsafe {
        ext_c::SETITERATIONMODE(&mut i)
    }
}


pub fn set_number_of_derivatives(mut i: i32) {
    unsafe {
        ext_c::SETNUMBEROFDERIVATIVES(&mut i)
    }
}


pub fn set_number_of_discrete_controls(mut i: i32) {
    unsafe {
        ext_c::SETNUMBEROFDISCRETECONTROLS(&mut i)
    }
}


pub fn set_number_of_inputs(mut i: i32) {
    unsafe {
        ext_c::SETNUMBEROFINPUTS(&mut i)
    }
}


pub fn set_number_of_outputs(mut i: i32) {
    unsafe {
        ext_c::SETNUMBEROFOUTPUTS(&mut i)
    }
}


pub fn set_number_of_parameters(mut i: i32) {
    unsafe {
        ext_c::SETNUMBEROFPARAMETERS(&mut i)
    }
}


pub fn set_number_of_report_variables(mut n_int: i32, mut n_min_max: i32, mut n_vals: i32, mut n_text: i32) {
    unsafe {
        ext_c::SETNUMBEROFREPORTVARIABLES(&mut n_int, &mut n_min_max, &mut n_vals, &mut n_text)
    }
}


pub fn set_number_stored_variables(mut n_requested_static: i32, mut n_requested_dynamic: i32) {
    unsafe {
        ext_c::SETNUMBERSTOREDVARIABLES(&mut n_requested_static, &mut n_requested_dynamic)
    }
}


pub fn set_numerical_derivative(mut i: i32, mut value: f64) {
    unsafe {
        ext_c::SETNUMERICALDERIVATIVE(&mut i, &mut value)
    }
}


pub fn set_output_units(mut i: i32, string: &str) {
    unsafe {
        let string = CString::new(string).unwrap();
        ext_c::SETOUTPUTUNITS(&mut i, string.as_ptr() as *mut c_char, string.to_bytes().len());
    }
}


pub fn set_output_value(mut i: i32, mut value: f64) {
    unsafe {
        ext_c::SETOUTPUTVALUE(&mut i, &mut value)
    }
}


pub fn set_static_array_value(mut i: i32, mut value: f64) {
    unsafe {
        ext_c::SETSTATICARRAYVALUE(&mut i, &mut value)
    }
}


pub fn set_type_version(mut i: i32) -> i32 {
    unsafe {
        ext_c::SETTYPEVERSION(&mut i)
    }
}


pub fn error_found() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_ERRORFOUND())
    }
}


pub fn get_convergence_tolerance() -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETCONVERGENCETOLERANCE()
    }
}


pub fn get_current_type() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETCURRENTTYPE()
    }
}

pub fn get_current_unit() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETCURRENTUNIT()
    }
}

pub fn get_deck_filename() -> String {
    let mut buffer = [0 as c_char; 256];
    unsafe {
        let ptr = ext_c::TRNSYSFUNCTIONS_mp_GETDECKFILENAME(buffer.as_mut_ptr(), buffer.len());
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn get_dynamic_array_value_last_timestep(mut i: i32) -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETDYNAMICARRAYVALUELASTTIMESTEP(&mut i)
    }
}

pub fn get_format(label: &mut [u8], iunit: &mut i32, no: &mut i32) -> String {
    if label.len() > 256 {
        panic!("The label length must be less than 256 bytes");
    }
    let mut buffer = [0 as c_char; 256];
    // write the label to the buffer
    for (i, &byte) in label.iter().enumerate() {
        buffer[i] = byte as c_char;
    }
    unsafe {
        let ptr = ext_c::TRNSYSFUNCTIONS_mp_GETFORMAT(buffer.as_mut_ptr(), buffer.len(), iunit, no);
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn get_input_value(mut i: i32) -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETINPUTVALUE(&mut i)
    }
}


///Note that the function will return `true`, **no matter if a converged solution was found or not**
/// at the current time step.
pub fn is_end_of_timestep() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISENDOFTIMESTEP())
    }
}

pub fn is_first_call_of_simulation() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISFIRSTCALLOFSIMULATION())
    }
}

pub fn is_included_in_ssr() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISINCLUDEDINSSR())
    }
}

pub fn is_last_call_of_simulation() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISLASTCALLOFSIMULATION())
    }
}

pub fn is_reread_parameters() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISREREADPARAMETERS())
    }
}

pub fn is_start_time() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISSTARTTIME())
    }
}

pub fn is_version_signing_time() -> bool {
    unsafe {
        c_bool(ext_c::TRNSYSFUNCTIONS_mp_GETISVERSIONSIGNINGTIME())
    }
}

pub fn get_label(iunit: &mut i32, no: &mut i32) -> String {
    let mut buffer = [0 as c_char; 256];
    unsafe {
        let ptr = ext_c::TRNSYSFUNCTIONS_mp_GETLABEL(buffer.as_mut_ptr(), buffer.len(), iunit, no);
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn get_lu_filename(mut lu: i32) -> String {
    let mut buffer = [0 as c_char; 256];
    unsafe {
        let ptr = ext_c::TRNSYSFUNCTIONS_mp_GETLUFILENAME(buffer.as_mut_ptr(), buffer.len(), &mut lu);
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn get_max_descrip_length() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETMAXDESCRIPLENGTH()
    }
}

pub fn get_max_label_length() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETMAXLABELLENGTH()
    }
}

pub fn get_max_path_length() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETMAXPATHLENGTH()
    }
}

pub fn get_minimum_timestep() -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETMINIMUMTIMESTEP()
    }
}

pub fn get_next_available_logical_unit() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNEXTAVAILABLELOGICALUNIT()
    }
}

pub fn get_number_of_derivatives() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNUMBEROFDERIVATIVES()
    }
}

pub fn get_number_of_inputs() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNUMBEROFINPUTS()
    }
}

pub fn get_number_of_labels() -> i32 {
    let mut i = 0;
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNUMBEROFLABELS(&mut i)
    }
}

pub fn get_number_of_outputs() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNUMBEROFOUTPUTS()
    }
}

pub fn get_number_of_parameters() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNUMBEROFPARAMETERS()
    }
}

pub fn get_numerical_solution(mut i: i32) -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETNUMERICALSOLUTION(&mut i)
    }
}

pub fn get_output_value(mut i: i32) -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETOUTPUTVALUE(&mut i)
    }
}

pub fn get_parameter_value(mut i: i32) -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETPARAMETERVALUE(&mut i)
    }
}

pub fn get_previous_control_state(mut i: i32) -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETPREVIOUSCONTROLSTATE(&mut i)
    }
}

pub fn get_simulation_start_time() -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETSIMULATIONSTARTTIME()
    }
}

pub fn get_simulation_stop_time() -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETSIMULATIONSTOPTIME()
    }
}

pub fn get_simulation_time() -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETSIMULATIONTIME()
    }
}

pub fn get_simulation_time_step() -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETSIMULATIONTIMESTEP()
    }
}

pub fn get_static_array_value(mut i: i32) -> f64 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETSTATICARRAYVALUE(&mut i)
    }
}

pub fn get_timestep_iteration() -> i32 {
    unsafe {
        ext_c::TRNSYSFUNCTIONS_mp_GETTIMESTEPITERATION()
    }
}

pub fn get_trnsys_input_file_dir() -> String {
    let mut buffer = [0 as c_char; 256];
    unsafe {
        let ptr = ext_c::TRNSYSFUNCTIONS_mp_GETTRNSYSINPUTFILEDIR(buffer.as_mut_ptr(), buffer.len());
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn get_trnsys_root_dir() -> String {
    let mut buffer = [0 as c_char; 256];
    unsafe {
        let ptr = ext_c::TRNSYSFUNCTIONS_mp_GETTRNSYSROOTDIR(buffer.as_mut_ptr(), buffer.len());
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}


pub fn update_report_integral(index: &mut i32, int_val: &mut f64) {
    unsafe {
        ext_c::UPDATEREPORTINTEGRAL(index, int_val)
    }
}


pub fn update_report_min_max(index: &mut i32, new_val: &mut f64) {
    unsafe {
        ext_c::UPDATEREPORTMINMAX(index, new_val)
    }
}


pub use ext_c::FLUID_PROPERTIES as fluid_properties;
pub use ext_c::GETHORIZONTALRADIATION as get_horizontal_radiation;
pub use ext_c::GETTILTEDRADIATION as get_tilted_radiation;
pub use ext_c::INTERPOLATEDATA as interpolate_data;
pub use ext_c::MESSAGES as messages;
pub use ext_c::MOISTAIRPROPERTIES as moist_air_properties;
pub use ext_c::SOLVEDIFFEQ as solve_diff_eq;
pub use ext_c::STEAM_PROPERTIES as steam_properties;
