use std::time::Instant;
use rand::Rng;

mod bubble;
mod insertion;
mod selection;

const ARR_SIZE: usize = 1000;

fn benchmark(f: &dyn Fn(&mut [i32]), name: &str) {
    let mut rng = rand::thread_rng();
    let mut arr: [i32; ARR_SIZE] = [0; ARR_SIZE];
    for i in 0..ARR_SIZE {
        arr[i] = rng.gen::<i32>();
    }
    print!("Benchmarking {}...", name);

    let start = Instant::now();
    f(&mut arr);
    println!(" {:?}", start.elapsed());
}

fn run_benchmarks() {
    println!("Running benchmarks for {} array elements...", ARR_SIZE);

    benchmark(&bubble::sort, "bubble sort");
    benchmark(&insertion::sort, "insertion sort");
    benchmark(&selection::sort, "selection sort")
}

fn main() {
    run_benchmarks();
}
