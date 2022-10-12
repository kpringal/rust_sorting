
#[cfg(test)]
mod sorting_test {
    use sortingalgo::UserInput;

    use crate::sorting;

    #[test]
    fn test_bubble_sort() {
        let _result = vec![2, 2, 5, 5, 5, 5, 21, 24, 54, 85, 154, 561, 564, 121154];

        let mut _user_input = UserInput {
            input_commnad: sortingalgo::SortingAlgo::BubbleSort,
            input_value: vec![24, 564, 561, 5, 154, 21, 5, 121154, 2, 54, 85, 2, 5, 5],
        };

        sorting::sort(&mut _user_input);

        assert_eq!(_user_input.input_value, _result)
    }

    #[test]
    fn test_insertion_sort() {
        let _result = vec![2, 2, 5, 5, 5, 5, 21, 24, 54, 85, 154, 561, 564, 121154];

        let mut _user_input = UserInput {
            input_commnad: sortingalgo::SortingAlgo::InsertionSort,
            input_value: vec![24, 564, 561, 5, 154, 21, 5, 121154, 2, 54, 85, 2, 5, 5],
        };

        sorting::sort(&mut _user_input);

        assert_eq!(_user_input.input_value, _result)  
    }
}