## Sorting algorightms
[![Build Status](https://travis-ci.com/MaciejSzybiak/sorting_algorithms.svg?branch=master)](https://travis-ci.com/MaciejSzybiak/sorting_algorithms)

This repository compiles an example implementation of some common sorting algorithms. Its main purpose is to help the autor understand how Rust works :).

The program itself provides a quick benchmark utility which measures the average speed of all implemented algorithms.

##### Algorithms implemented so far:
- [bubble sort](src/bubble.rs)
- [insertion sort](src/insertion.rs)
- [selection sort](src/selection.rs)
- [merge sort](src/merge.rs) (top-down and bottom-up versions)
- [heap sort](src/heap.rs)
- [quicksort](src/quicksort.rs)

##### Example program output:
```
Running 10 benchmark passes for each array size.
Average time for 100 array elements:
Benchmarking bubble sort...     204.16µs
Benchmarking insertion sort...  56.16µs
Benchmarking selection sort...  157.49µs
Benchmarking merge top-down...  101.56µs
Benchmarking merge bottom-up... 97.06µs
Benchmarking heap sort...       36.62µs
Benchmarking quicksort...       15.73µs

Average time for 1000 array elements:
Benchmarking bubble sort...     24.08514ms
Benchmarking insertion sort...  4.85086ms
Benchmarking selection sort...  14.79537ms
Benchmarking merge top-down...  1.09332ms
Benchmarking merge bottom-up... 1.06216ms
Benchmarking heap sort...       430.57µs
Benchmarking quicksort...       234.2µs

Average time for 5000 array elements:
Benchmarking bubble sort...     519.57212ms
Benchmarking insertion sort...  121.67679ms
Benchmarking selection sort...  380.27264ms
Benchmarking merge top-down...  6.43154ms
Benchmarking merge bottom-up... 5.54737ms
Benchmarking heap sort...       3.41574ms
Benchmarking quicksort...       1.56631ms
```