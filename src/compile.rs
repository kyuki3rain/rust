use crate::ast::Program;

pub fn compile(program: Program) -> String {
    let mut asm = String::new();
    asm += &format!(".intel_syntax noprefix\n");
    asm += &format!(".globl main\n");

    asm += &format!("main:\n");

    asm += &format!("  mov rax, {}\n", program);
    asm += &format!("  ret\n");

    return asm;
}
