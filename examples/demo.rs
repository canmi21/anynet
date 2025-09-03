/* examples/demo.rs */

use anynet::anynet;
use fancy_log::{LogLevel, set_log_level};

fn main() {
    set_log_level(LogLevel::Debug);
    println!("--- Running demo with public=false (default) ---");
    anynet!(port = 8080);
    println!("\n--- Running demo with public=true ---");
    anynet!(port = 3000, public = true);
    println!("\n--- Running demo with invalid port ---");
    anynet!(port = 0);

    set_log_level(LogLevel::Info);
    println!("--- Running demo with public=false (default) ---");
    anynet!(port = 8080);
    println!("\n--- Running demo with public=true ---");
    anynet!(port = 3000, public = true);
    println!("\n--- Running demo with invalid port ---");
    anynet!(port = 0);
}
