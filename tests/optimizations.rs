use std::env;
use std::fs;
use std::process::Command;

/// A helper function to compile a C string and check if the assembly output contains a given substring
fn check_assambly_output(test_name: &str, c_code: &str, substr: &str, error_message: &str) {
    // Set up a temporary directory for our generated files
    let out_dir = env::temp_dir().join("compiler_e2e_tests");
    fs::create_dir_all(&out_dir).expect("Failed to create test directory");

    let c_file = out_dir.join(format!("{}.c", test_name));
    let asm_file = out_dir.join(format!("{}.s", test_name));
    let output_file = out_dir.join(test_name);

    // Write the C code to disk
    fs::write(&c_file, c_code).expect("Failed to write C file");

    // Invoke compiler
    let compiler_exe = env!("CARGO_BIN_EXE_c-moon");

    let compile_status = Command::new(compiler_exe)
        .arg(c_file.to_str().unwrap())
        .arg("--asm")
        .arg("--opt")
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .status()
        .expect("Failed to execute compiler process");

    assert!(
        compile_status.success(),
        "Compilation failed for test: {}",
        test_name
    );

    let asm_program = fs::read_to_string(asm_file).expect("failed to read assembly output");

    // Assert the exit code
    assert!(
        asm_program.contains(substr),
        "{}: {}",
        test_name,
        error_message,
    );
}

#[test]
fn test_constant_fold_addition_asm() {
    let code = "
        int main() {
            int a = 20;
            int b = 22;
            return a + b;
        }
    ";
    check_assambly_output(
        "test_constant_fold_addition_asm",
        code,
        "mov rax, 42",
        "Compiler did not fold constants properly",
    );
}

#[test]
fn test_constant_fold_subtraction_asm() {
    let code = "
        int main() {
            int a = 22;
            int b = 20;
            return a - b;
        }
    ";
    check_assambly_output(
        "test_constant_fold_subtraction_asm",
        code,
        "mov rax, 2",
        "Compiler did not fold constants properly",
    );
}

#[test]
fn test_constant_fold_addition_multiplication_asm() {
    let code = "
        int main() {
            int a = 10;
            int b = 100;
            return a * b;
        }
    ";
    check_assambly_output(
        "test_constant_fold_addition_multiplication_asm",
        code,
        "mov rax, 1000",
        "Compiler did not fold constants properly",
    );
}

#[test]
fn test_constant_fold_complex_expression_asm() {
    let code = "
        int main() {
            int a = 1;
            int b = 2;
            int c = 2;
            return c * b + 10 - a;
        }
    ";
    check_assambly_output(
        "test_constant_fold_complex_expression_asm",
        code,
        "mov rax, 13",
        "Compiler did not fold constants properly",
    );
}

#[test]
fn test_constant_fold_within_if_asm() {
    let code = "
        int main() {
            int a = 2;

            if (a == 2) {
              a = a + 1;
            } else {
              a = 0;
            }

            return a + 1;
        }
    ";
    check_assambly_output(
        "test_constant_fold_within_if_asm",
        code,
        "mov rax, 4",
        "Compiler did not fold constants properly",
    );
}
