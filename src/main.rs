use std::env;

use navires;
use navires::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    navires::run(config);
}
