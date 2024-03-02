# string
Some traits for working with strings in Rust

# Usage:

- Step 1: Add log dependencies to your project as follows:

Cargo.toml:
````
[dependencies]
string = { git = "https://github.com/gobkc/string" }
````

- Step 2: run `cargo update`
- Step 3: now you can use log the library

# Example

````
use string::StringExtend;

fn main() {
    let s = "Hello, world!";
    let new_s = s.substr(0,5);
    println!("{}",new_s);//output: Hello
}
````
