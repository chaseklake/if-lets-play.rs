// Example of "if let" statements in Rust, a more concise form of "match"

fn main() {
    let config_max = Some(3u8);

    // Kinda wordy to use match here, since we only care about one arm of the possibilities:
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Much more concise!
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Really useful for when you want to avoid exhaustively handling every arm in match
    // Also useful to handle when you want to handle all cases except for one using 'else':
    else {
        println!("Config_max is NOT a u8!");
    }
}
