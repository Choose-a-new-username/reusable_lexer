use reusable_lexer::*;
use std::env;
use std::fs;

macro_rules! elapsed {
    ($($code:tt)*) => {
        {
            let start = std::time::Instant::now();
            $($code)*
            println!("{:?}", start.elapsed());
        }
    }
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let file = fs::read_to_string(arg).expect("failed to read file");
        let lexer: Lexer<'_> = Lexer::new(&file);

        elapsed!(
            for tok in lexer {
                println!("{tok:?}");
            };
            print!("Elapsed time: ");
        );
    }
}
