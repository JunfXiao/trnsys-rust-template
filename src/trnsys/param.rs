use std::panic::catch_unwind;
use crate::trnsys::get_lu_filename;

pub struct TrnsysValue {
    pub value: f64,
}

impl TrnsysValue {
    pub fn new(value: f64) -> Self {
        TrnsysValue { value }
    }


}

impl From<f64> for TrnsysValue {
    fn from(value: f64) -> Self {
        TrnsysValue { value }
    }
}

impl  From<i32> for TrnsysValue {
    fn from(value: i32) -> Self {
        TrnsysValue { value: value as f64 }
    }
}



impl From<TrnsysValue> for f64 {
    fn from(param: TrnsysValue) -> Self {
        param.value
    }
}

impl From<TrnsysValue> for i32 {
    fn from(param: TrnsysValue) -> Self {
        // float to int conversion
        param.value as i32
    }
}


impl TryFrom<TrnsysValue> for String {
    type Error = String;

    fn try_from(param: TrnsysValue) -> Result<Self, Self::Error> {
        // try to convert the float to the string
        // by reading the logical unit of the file from value
        let logical_unit = (param.value + 0.1) as i32;
        let fname = catch_unwind(||{
            get_lu_filename(logical_unit)
        });

        match fname {
            Ok(name) => Ok(name),
            Err(e) =>
                match e.downcast::<String>() {
                Ok(panic_msg) => {
                    let panic_msg = format!("Cannot read param as string:{}", panic_msg);
                    println!("{}", &panic_msg);
                    Err(panic_msg.to_string())
                }
                Err(_) => {
                    let panic_msg = "Cannot read param as string: unknown error";
                    println!("{}", &panic_msg);
                    Err(panic_msg.to_string())
                }
            }
        }
    }
}