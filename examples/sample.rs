//! Sample Rust file to demonstrate Colorful Darcula theme highlighting
//!
//! This file showcases various Rust syntax elements and how they are
//! highlighted with the theme optimized for Rust development.

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::error::Error;
use std::sync::atomic::AtomicUsize;

// Exported constant (purple)
pub const EXPORTED_CONSTANT: i32 = 42;

// Private constant (purple)
const PRIVATE_CONSTANT: &str = "private";

/// Exported trait demonstrating trait highlighting
pub trait ExportedTrait {
    fn exported_method(&self) -> String;
    fn process_data(&mut self, input: &str) -> Result<(), Box<dyn Error>>;
}

/// Private trait
trait PrivateTrait {
    fn internal_process(&self) -> bool;
}

/// Exported struct demonstrating struct highlighting (brown)
#[derive(Debug, Clone)]
pub struct ExportedStruct {
    pub exported_field: String,  // Public field
    private_field: i32,           // Private field
    pub data: Vec<u8>,
}

/// Private struct
#[derive(Debug)]
struct PrivateStruct {
    value: String,
    counter: usize,
}

/// Exported enum demonstrating enum highlighting (tan)
#[derive(Debug, PartialEq)]
pub enum ExportedEnum {
    Variant1,
    Variant2(String),
    Variant3 { field: i32 },
}

/// Type alias demonstration
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Static variable
static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Exported function (yellow)
pub fn exported_function(param: &str, count: i32) -> Result<String> {
    // Local variables
    let local_var = "local variable";
    let result = count * 2;

    // String formatting with escape sequences
    let formatted = format!("Result: {}\nCount: {}", result, count);

    // Shadowing demonstration
    {
        let local_var = "shadowed variable";
        println!("{}", local_var);
    }

    // Function calls
    private_function(param);

    Ok(formatted)
}

// Private function (orange)
fn private_function(input: &str) -> String {
    format!("processed: {}", input)
}

// Implementation block
impl ExportedStruct {
    /// Constructor (new is a convention)
    pub fn new(field: String) -> Self {
        Self {
            exported_field: field,
            private_field: 0,
            data: Vec::new(),
        }
    }

    /// Method with self reference
    pub fn exported_method(&self) -> &str {
        &self.exported_field
    }

    /// Mutable method
    pub fn update_field(&mut self, value: String) {
        self.exported_field = value;
    }

    /// Method with various types
    pub fn process_data(&mut self, input: &str) -> Result<()> {
        // String literals (green)
        let message = format!("Processing: {}", input);

        // Numbers (light blue)
        let timeout = 30;
        let max_retries = 3;

        // Boolean (orange)
        let is_valid = true;

        // Control flow keywords (orange)
        if is_valid && !input.is_empty() {
            for i in 0..max_retries {
                println!("Attempt {} of {}", i + 1, max_retries);
                std::thread::sleep(std::time::Duration::from_secs(timeout));
            }
        }

        println!("{}", message);
        Ok(())
    }

    // Private method
    fn private_method(&self) -> i32 {
        self.private_field * 2
    }
}

// Trait implementation
impl ExportedTrait for ExportedStruct {
    fn exported_method(&self) -> String {
        self.exported_field.clone()
    }

    fn process_data(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        self.exported_field = input.to_string();
        Ok(())
    }
}

// Display trait implementation
impl Display for ExportedStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExportedStruct {{ field: {} }}", self.exported_field)
    }
}

// Generic function
pub fn generic_function<T: Clone>(items: &[T]) -> Option<T> {
    items.first().cloned()
}

// Generic function with multiple bounds
pub fn complex_generic<T, U>(item: T, other: U) -> T
where
    T: Clone + Display,
    U: Into<String>,
{
    println!("{}", other.into());
    item
}

// Async function
pub async fn async_function(url: &str) -> Result<String> {
    // Simulated async operation
    let data = format!("Data from: {}", url);
    Ok(data)
}

// Macro demonstration
macro_rules! create_struct {
    ($name:ident, $field:ty) => {
        struct $name {
            value: $field,
        }
    };
}

create_struct!(GeneratedStruct, i32);

// Attribute macro usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exported_function() {
        let result = exported_function("test", 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_struct_creation() {
        let mut obj = ExportedStruct::new("example".to_string());
        assert_eq!(obj.exported_method(), "example");

        obj.update_field("updated".to_string());
        assert_eq!(obj.exported_method(), "updated");
    }

    #[test]
    fn test_enum_matching() {
        let variant = ExportedEnum::Variant2("test".to_string());

        match variant {
            ExportedEnum::Variant1 => println!("Variant 1"),
            ExportedEnum::Variant2(ref s) => println!("Variant 2: {}", s),
            ExportedEnum::Variant3 { field } => println!("Variant 3: {}", field),
        }
    }
}

fn main() {
    // Comments in gray with italic style
    // This demonstrates various Rust syntax highlighting

    // String literals (green)
    let greeting = "Hello, Colorful Darcula!";

    // Raw strings
    let raw_string = r#"Raw string with "quotes""#;

    // Numbers (light blue)
    let integer = 42;
    let float = 3.14159;
    let hex = 0xFF;
    let binary = 0b1010;

    // Character literals
    let character = 'R';

    // Boolean
    let is_active = true;

    // Arrays and vectors
    let numbers = vec![1, 2, 3, 4, 5];
    let array: [i32; 3] = [10, 20, 30];

    // Function calls
    let result = exported_function(greeting, integer).unwrap();
    println!("{}", result);

    // Struct instantiation
    let mut obj = ExportedStruct::new("example".to_string());
    obj.exported_field = "updated".to_string();

    // Method calls
    let data = obj.exported_method();
    obj.process_data(data).unwrap();

    // Pattern matching
    let value = ExportedEnum::Variant2("test".to_string());
    match value {
        ExportedEnum::Variant1 => println!("First variant"),
        ExportedEnum::Variant2(s) => println!("Second variant: {}", s),
        ExportedEnum::Variant3 { field } => println!("Third variant: {}", field),
    }

    // HashMap operations
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key".to_string(), 100);

    // Iterator operations
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .collect();

    // Range iteration
    for (idx, num) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", idx, num);
    }

    // Error handling
    if let Err(e) = obj.process_data("test") {
        eprintln!("Error: {}", e);
    }

    // Option handling
    let maybe_value = Some(42);
    if let Some(v) = maybe_value {
        println!("Got value: {}", v);
    }

    // Closure demonstration
    let add_one = |x: i32| x + 1;
    let incremented = numbers.iter().map(|&x| add_one(x)).collect::<Vec<_>>();

    // Generic function usage
    let first = generic_function(&numbers);
    println!("First element: {:?}", first);

    // Reference and dereferencing
    let reference = &integer;
    let dereferenced = *reference;

    // Lifetime demonstration
    let string1 = String::from("long string");
    let string2 = "short";
    let result = longest(&string1, string2);
    println!("Longest: {}", result);

    // Print final state
    println!("{}", obj);
    println!("Raw: {}", raw_string);
    println!("Array: {:?}, Doubled: {:?}", array, doubled);
}

// Function with lifetime parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Unsafe code block
unsafe fn unsafe_function(ptr: *const i32) -> i32 {
    *ptr
}
