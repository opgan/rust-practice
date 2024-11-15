// A module named `my_mod`
pub mod my_mod {
    // Items in modules default to private visibility.
    pub fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }
}