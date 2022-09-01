// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the res
pub fn run_if_let() {
    let config_max = Some(3u8);

    //  if let is a syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

    // this extra boilerplate code for just a single pattern can be annoying
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let mut count = 1;
    // to simplify this we use if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        // we can also use else with if let its like "_" syntax in match statements
        count += 1;
    }
}
