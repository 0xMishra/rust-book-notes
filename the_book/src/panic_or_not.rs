use std::net::IpAddr;
pub fn to_panic_or_not() {
    // If you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    //If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure, we’d definitely want to handle the Result
}
