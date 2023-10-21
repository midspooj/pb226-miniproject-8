use std::fs;

#[test]
fn test_cli_tool() {
    // Create a temporary input file
    let input_file = "test_input.csv";
    let mut input_content = "header1,header2\n".to_string();
    for i in 1..=10 {
        input_content += &format!("row{},{}\n", i, i * 2);
    }
    fs::write(input_file, input_content).expect("Failed to create input file for testing.");

    // Call the main function
    let result = super::main_with_args(vec!["test", input_file.to_string()]);

    // Check if the output file was created
    assert!(result.is_ok());
    assert!(fs::metadata("output.txt").is_ok());

    // Read and verify the content of the output file
    let output_content =
        fs::read_to_string("output.txt").expect("Failed to read output file for testing.");
    assert_eq!(output_content.trim(), "Average: 6.00");

    // Clean up: remove temporary files
    fs::remove_file(input_file).expect("Failed to clean up temporary input file.");
    fs::remove_file("output.txt").expect("Failed to clean up temporary output file.");
}
