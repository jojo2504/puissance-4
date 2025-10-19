use console::Term;

pub fn play() -> i32 {
    let term = Term::stdout();
    loop {
        let resolved_input: Option<u32> = match term.read_char() {
            Ok(c) => {
                c.to_digit(10)
            }
            Err(_) => {
                println!("invalid input (0-6)");
                None
            }
        };

        if resolved_input.is_some() {
            return resolved_input.unwrap() as i32;
        }
        else {
            continue;
        }
    }
}