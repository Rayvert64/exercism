use unicode_segmentation::UnicodeSegmentation;

// This function will be multi-threaded depending on the length of the string
pub fn reverse(input: &str) -> String {
    let input_len = input.len();

    // Get the length of the string
    let mut output = String::new();
    output.reserve_exact(input_len);

    input
        .graphemes(true)
        .rev()
        .for_each(|character| output.push_str(character));

    output
}
