use std::env;
mod ast;
mod compiler;
mod lexer;
mod parser;
mod token;
mod environment;

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
    use std::fs;
    use std::io::Write;
    use std::process::Output;
    use std::{fs::File, process::Command};

    extern crate rand;
    use rand::Rng;

    use super::*;

    fn execute(program: &str) -> Output {
        let mut rng = rand::thread_rng();
        execute_with_filename(program, &rng.gen::<u32>().to_string())
    }

    fn execute_with_filename(program: &str, filename: &str) -> Output {
        let asm_path = String::new() + "./tmp/" + filename + ".s";
        let exe_path = String::new() + "./tmp/" + filename + ".out";

        let output = Command::new(env!("CARGO"))
            .args(vec!["run", "--", program])
            .output()
            .expect("failed to compile");

        let mut path = env::current_dir().unwrap();
        path.push(&asm_path);
        let mut file = File::create(path).unwrap();
        write!(file, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
        file.flush().unwrap();

        Command::new("cc")
            .args(&[&asm_path, "-o", &exe_path])
            .output()
            .expect("failed to build");

        let output = Command::new(&exe_path).output().expect("failed to execute");

        fs::remove_file(asm_path).expect("failed to remove asm file");
        fs::remove_file(exe_path).expect("failed to remove exe file");

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

    #[test]
    fn test_plusminus() {
        let program = "1+2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 3);

        let program = "1-2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 255);

        let program = "114+41-136";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 19);
    }

    #[test]
    fn test_muldiv() {
        let program = "4*2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 8);

        let program = "4/2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 2);

        let program = "3*4/2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 6);
    }

    #[test]
    fn test_arithmetic() {
        let program = "4+2*3";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 10);

        let program = "(4+2)*3";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 18);
    }

    #[test]
    fn test_minusprefix() {
        let program = "-1";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 255);

        let program = "-5+10";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 5);
    }

    #[test]
    fn test_cmp() {
        let program = "1>2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 0);

        let program = "1<2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 1);

        let program = "1<=2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 1);

        let program = "1>=2";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 0);

        let program = "1>=1";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 1);

        let program = "1==1";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 1);

        let program = "1!=1";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 0);
    }

    #[test]
    fn test_semicolon() {
        let program = "5 + 10; 8 * 5";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 40);
    }
    
    #[test]
    fn test_ident() {
        let program = "a = 10; a + 10";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 20);
    }

    #[test]
    fn test_complex_program() {
        let program = "a = 5; b = 10; c = a + b; d = a * b; d - c";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 35);
    }

    #[test]
    fn test_return() {
        let program = "a = 5; return a + 2; a + 5";
        let output = execute(program);
        assert_eq!(output.status.code().unwrap(), 7);
    }
}
