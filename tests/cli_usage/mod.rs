#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct CliUsageTestCase<'a> {
    args: Vec<&'a str>,
    stdin: &'a str,
    stdout: &'a str,
    stderr: &'a str,
}

/**
    Generates test cases defined in data file.

    Idea: <https://blog.cyplo.dev/posts/2018/12/generate-rust-tests-from-data/>
*/
pub fn generate() {
    use std::path::Path;

    const TESTS_FILE: &str = "test_cases.ron";

    let test_cases = Path::new("cli_usage").join(TESTS_FILE);

    println!("cargo:rerun-if-changed={}", test_cases.to_str().unwrap());

    let gen_path = std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap())
        .join(std::path::Path::new(file!()).with_file_name(format!("{TESTS_FILE}.rs")));

    let test_file_content = std::fs::read_to_string(test_cases).unwrap();
    let tests: Vec<CliUsageTestCase> = ron::de::from_str(&test_file_content).unwrap();
    let mut output = String::new();

    for (idx, test) in tests.iter().enumerate() {
        output.push_str(&format!(
            include_str!("template.txt"),
            idx = idx + 1,
            args = test.args,
            stdin = test.stdin,
            stdout = test.stdout,
            stderr = test.stderr,
        ));
    }

    std::fs::create_dir_all(gen_path.parent().unwrap()).unwrap();
    std::fs::write(gen_path, output).unwrap();
}
