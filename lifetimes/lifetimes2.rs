// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    // Bring string2 into the same scope of string1, so they both have the same lifetime which is needed to log out result
    let string2 = String::from("xyz");
    
    {
        result = longest(string1.as_str(), string2.as_str());
        // Or we can bring println into the same scope of the computation
        // println!("The longest string is '{}'", result);

        // Or we can bring everything also into the same scope, inside the curlies
    }
    println!("The longest string is '{}'", result);
}
