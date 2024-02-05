use flipc::frontend;

fn main() {
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            match frontend::check(&line) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        }
    });
}
