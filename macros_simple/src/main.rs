// Define a macro that lets us add multiple elements to a vector.
// This is a 'procedural' macro, and we export it for use by other programs.
#[macro_export]
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            $target.push($val);
        )+
    };
}

fn main() {
    let mut vec = Vec::new();
    push!(vec, 1, 2, 3, 5);
    println!("{:?}", vec)
}

// Macros can also call other macros:
/*
macro_rules! really_push {
    ($target: expr, $val: expr) => {
        $target.push($val);
    };
}

#[macro_export]
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            really_push!($target, $val);
        )+
    };
}
*/
