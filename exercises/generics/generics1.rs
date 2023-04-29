// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.


enum Generic<T> {
    Value(T),
}

fn main() {
//  let mut shopping_list: Vec<&'static str> = Vec::new();
//  shopping_list.push("milk");
    use crate::Generic::Value;
    let mut shopping_list: Vec<Generic<&'static str>> = Vec::new();
    shopping_list.push(Value("milk"));
    shopping_list.push(Value("milk"));
    shopping_list.push(Value("milk"));
    shopping_list.push(Value("milk"));
}
