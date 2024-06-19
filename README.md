# WindowManager

A window manager for Windows using Rust.

## Usage

WindowManager allows you to assign key combinations to switch between different windows easily. Here's how to use it:

1. **Choose Key Combinations:** 
   - Define a list of key combinations for switching windows. 
   - Example: 
     ```rust
     let mapping: Vec<String> = Vec::from([String::from("Alt+a"), String::from("Alt+s"), String::from("Alt+d")]);
     ```

2. **Set Key Combinations:**
   - Click on a window and then press the desired key combination to assign it to that window.

3. **Switch Windows:**
   - Press the assigned key combination to switch to the corresponding window.

4. **Reset Bindings:**
   - Define a reset key to clear all current key bindings. 
   - Example: 
     ```rust
     let mut mapper = WindowMapping::new(mapping, String::from("Alt+f")).unwrap_or_else(|err| {
         eprintln!("Problem parsing mappings: {}", err);
         process::exit(1);
     });
     ```

5. **Automatic Reassignment:**
   - If a window mapped to a key is closed, pressing the key again will bind the currently focused window to that key.

### Key Combination Format

- Format: `Starter+letter`
- Example: `Alt+a`, `Ctr+b`, `Sht+c`, `Win+d`
  - `Alt` - Alt key
  - `Ctr` - Control key
  - `Sht` - Shift key
  - `Win` - Windows key

### Running the Project
Once you have configured the keys in the main file, you can run the project. You can also set the compiled executable to auto-run at startup to ensure WindowManager is always active.
