// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    let a = "dxVE5gds9aFI7kUtcpzgIp+bIrJdm1d7YxHSDT6HfBfL7PKslbP2VffPoYPPlBwm4q7pFpyhtznJ7aIcOHIpceEIlWaRSNUvHl9AZg3+iDBAsuHfGKWrjcty4Iy+yqmC";
    let a = ["a"; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
