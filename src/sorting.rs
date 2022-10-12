use sortingalgo::{bubble_sort, insertion_sort, SortingAlgo, UserInput};

#[cfg(test)]
pub mod sorting_test;

pub fn sort(_input: &mut UserInput) {
    println!("Started sorting using {:?}", _input.input_commnad);
    match _input.input_commnad {
        SortingAlgo::BubbleSort => bubble_sort::sort(&mut _input.input_value),
        SortingAlgo::InsertionSort => insertion_sort::sort(&mut _input.input_value),
    }
    println!("Sorting completed using {:?}", _input.input_commnad);
}
