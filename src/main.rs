use std::env;
mod compile;
mod token;
mod lexer;
mod parser;
mod ast;
mod compiler;

// c-compiler
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments!");
    }

    let l = lexer::Lexer::new(&args[1]);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();

    let asm = compiler::Compiler::new().compile_program(program).unwrap();

    println!("{}", asm);
}

#[cfg(test)]
mod test {
    use std::process::Output;
    use std::{process::Command, fs::File};
    use std::io::Write;
    
    use super::*;

    fn execute(program: &str) -> Output {
        let output = Command::new(env!("CARGO"))
        .args(vec!["run", program])
        .output()
        .expect("failed to compile");

        let mut path = env::current_dir().unwrap();
        path.push("tmp");
        path.push("test1.s");
        let mut file = File::create(path).unwrap();
        write!(file, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
        file.flush().unwrap();
        
        Command::new("cc")
        .args(&["-o", "./tmp/test1.out", "./tmp/test1.s"])
        .output()
        .expect("failed to build");

        let output = Command::new("./tmp/test1.out")
        .output()
        .expect("failed to execute");

        output
    }

    #[test]
    fn test_compile() {
        let program = "0";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 0);

        let program = "42";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 42);

        let program = "123";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 123);
    }
}