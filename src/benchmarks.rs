use std::time::{Instant, Duration};
use rand::Rng;
use crate::{bubble, insertion, selection, merge, heap, quicksort};

const ARR_SIZES: [usize; 3] = [100, 1000, 5000];
const PASS_COUNT: usize = 10;

fn get_benchmark_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..size {
        vec.push(rng.gen::<i32>());
    }
    vec
}

fn benchmark(f: &dyn Fn(&mut [i32]), name: &str, size: usize) {
    let mut duration: Duration = Duration::new(0, 0);

    print!("Benchmarking {}...", name);
    for _ in 0..PASS_COUNT {
        let mut vec = get_benchmark_vector(size);
        let start = Instant::now();
        f(&mut vec);
        duration += start.elapsed();
    }
    println!("\t{:?}", duration / PASS_COUNT as u32);
}

pub fn run_benchmarks() {
    println!("Running {} benchmark passes for each array size.", PASS_COUNT);
    for i in 0..ARR_SIZES.len() {
        println!("Average time for {} array elements:", ARR_SIZES[i]);
        
        benchmark(&bubble::sort, "bubble sort", ARR_SIZES[i]);
        benchmark(&insertion::sort, "insertion sort", ARR_SIZES[i]);
        benchmark(&selection::sort, "selection sort", ARR_SIZES[i]);
        benchmark(&merge::sort_top_down, "merge top-down", ARR_SIZES[i]);
        benchmark(&merge::sort_bottom_up, "merge bottom-up", ARR_SIZES[i]);
        benchmark(&heap::sort, "heap sort", ARR_SIZES[i]);
        benchmark(&quicksort::sort, "quicksort", ARR_SIZES[i]);

        println!();
    }
}
