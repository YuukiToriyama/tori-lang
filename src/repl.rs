use std::io::Write;

pub struct Repl {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl Repl {
    pub fn new(stdin: std::io::Stdin, stdout: std::io::Stdout) -> Repl {
        Repl { stdin, stdout }
    }

    pub fn start(&mut self) {
        const PROMPT: &str = ">> ";
        print!("This is the torilang interactive interface!\n{}", PROMPT);
        self.stdout.flush().unwrap();

        let mut input = String::new();
        while self.stdin.read_line(&mut input).is_ok() {
            print!("{}\n{}", input, PROMPT);
            input.clear();
            self.stdout.flush().unwrap();
        }
    }
}
