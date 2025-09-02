#[test]
fn test_lox_files() {
    use crate::run_file;

    let base_directory = "lox/";
    let files = [
        "control_flow/if_else",
        "control_flow/loops",
        "functions/fibonacci",
        "functions/functions_2",
        "functions/functions",
        "functions/recursion",
        "mixed_types_expression",
        "scopes",
    ];

    for file in files {
        run_file(&(base_directory.to_owned() + file + ".lox"));
    }
}
