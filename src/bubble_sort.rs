
    use std::time::Instant;

    pub fn sort(_input: &mut Vec<i32>) {
        println!("Unsorted input is {:?}", _input);
        let start = Instant::now();
        let _length = _input.len();

        for _outer in 0.._length {
            let mut is_swap = false;
            for _inner in 0.._length - _outer - 1 {
                if _input[_inner] > _input[_inner + 1] {
                    let temp = _input[_inner];
                    _input[_inner] = _input[_inner + 1];
                    _input[_inner + 1] = temp;
                    is_swap = true;
                }
            }

            if is_swap == false {
                break;
            }
        }

        let duration = start.elapsed();
        println!("Sorted input is {:?}", _input);
        println!(
            "Time(Micro) taken for bubble sorting is  {}",
            duration.as_micros()
        );
    }

