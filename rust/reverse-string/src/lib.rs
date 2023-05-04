use cpuid::identify;

// This is the minimum length of the string to use multiple threads
const MIN_LEN_FOR_THREADING: usize = 2000;

// This function will be multi-threaded depending on the length of the string
pub fn reverse(input: &str) -> String {
    let input_len = input.len();

    // Get the length of the string
    let mut output = String::new();
    output.reserve_exact(input_len);

//    // Calculate optimal amount of cores to use
//    if input_len < MIN_LEN_FOR_THREADING {
//
//    } 
//    // if the string is not ling enough it's not worth multithreading
//    else {
    swap_chars();
//    }

    output
}

async fn swap_chars() {

}

fn get_optimal_nb_threads() -> i32 {
    match identify() {
        Ok(info) => {
            let num_cpus = info.num_logical_cpus;
            if num_cpus > 1 {
                // Figure out the most perfectest amount of cores
                // to use depending on system
                num_cpus
            } else {
                1
            }
        },
        // If we can't get cpu number we will just use one thread
        Err(_) => {
            1
        }
    }
}