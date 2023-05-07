use futures::executor::block_on;
use num_cpus;
use unicode_segmentation::UnicodeSegmentation;

// This is the minimum length of the string to use multiple threads
const MIN_LEN_FOR_THREADING: usize = 10000;

// This function will be multi-threaded depending on the length of the string
pub fn reverse(input: &str) -> String {
    let input_len = input.len();

    // Get the length of the string
    let mut output = String::new();
    output.reserve_exact(input_len);

    // Calculate optimal amount of cores to use
    //if input_len < MIN_LEN_FOR_THREADING {
    //    let num_tasks = get_optimal_nb_threads();
    //    let vec_slices = Vec<()>;
    //    for i in 0..num_tasks {
    //        // Create the string slices
    //    }
    //    join!()
    //}
    // if the string is not ling enough it's not worth multithreading
    //else {
    block_on(swap_chars(input, &mut output));
    //}

    output
}

async fn swap_chars(str_in: &str, str_out: &mut String) {
    if str_in.len() != str_out.capacity() {
        panic!("Strings should be same size")
    }

    let chars_in = str_in.graphemes(true).rev().collect::<String>();
    str_out.clone_from(&chars_in);
}

fn get_optimal_nb_threads() -> usize {
    let num_cpus = num_cpus::get();
    if num_cpus > 1 {
        // Figure out the most perfectest amount of cores
        // to use depending on system
        num_cpus
    } else {
        1
    }
}
