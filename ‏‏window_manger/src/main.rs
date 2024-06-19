#![windows_subsystem = "windows"]
use window_manger::window_mapping::WindowMapping;
use std::process;

fn main() {
    // keys to switch between windows
    let mapping: Vec<String> = Vec::from([String::from("Alt+a"),String::from("Alt+s"),String::from("Alt+d")]);

    // the alt f is for reseting the windows.
    let mut mapper = WindowMapping::new(mapping,String::from("Alt+f")).unwrap_or_else(|err| {
        eprintln!("Problem parsing mappings, {}",err);
        process::exit(1);
    });
    mapper.run().unwrap_or_else(|err| {
        eprintln!("Problem handeling press, {}",err);
        process::exit(1);
    });

}
