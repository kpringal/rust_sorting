use std::time::Instant;

pub fn sort(_input: &mut Vec<i32>) {
    println!("Unsorted input is {:?}", _input);
    let start = Instant::now();
    let _length = _input.len();
    let mut _inner = 0;
    let mut _temp = 0;

    for _outer in 1.._length {
        _inner = _outer;
        _temp = _input[_outer];

        while _inner >= 1 && _input[_inner-1] > _temp {
            _input[_inner ] = _input[_inner -1];
            if _inner == 0 {
                break;
            }
            _inner -= 1;
        }
        _input[_inner ] = _temp;
    }

    let duration = start.elapsed();
    println!("Sorted input is {:?}", _input);
    println!(
        "Time(Micro Seconds) taken for bubble sorting is  {}",
        duration.as_micros()
    );
}

