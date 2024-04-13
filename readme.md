
# trnsys_rust_template

TrnSys Rust Template is a template for creating TrnSys components in Rust. 
It is based on the TrnSys C++ Template and Documentation, which can be found under your TrnSys installation folder.

## Usage

1. Clone this repository
2. Put the `TRNDll64.lib` under the `lib` folder
3. Change the project name in `Cargo.toml` if needed
4. Change the Type number in `src/lib.rs` to the desired type number, e.g. \
`pub extern "C" fn TYPE256()`
5. Write the component logic in the `src/trnsys_type.rs` file, where you can use all the functions in different simulation time as documented.

## Advanced Usage

### Add more functions
You can add more functions to the `src/trnsys/ext_c.rs` file and add the corresponding rust-flavored function in the `src/trnsys/mod.rs` file.