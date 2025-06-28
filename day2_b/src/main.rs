use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::num;

// fn number_iterations(&element: &i32, second_column: &Vec<i32>) -> i32 {
//     let count = second_column.into_iter()
//                             .filter(|x| **x == element)
//                             .count();
//     // if count > 0 {
//     //     println!("The count is {} and the element is {}", count, element);
//     // }
//     println!("The count is {} and the element is {}", count, element);
//     count.try_into().unwrap()
// }

fn main() {
    // let mut line_vectors: Vec<i32> = vec![];
    let mut line_vectors: Vec<Vec<i32>> = Vec::new();
    let mut total_unsafe: u32 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            line_vectors.push(line.split_whitespace()
                                    .filter_map(|x| x.parse::<i32>().ok())
                                    .collect());
        }
    }

    for lv in line_vectors.iter() {
        // println!("{:?}", lv);

        if !(lv.iter().is_sorted() || lv.iter().is_sorted_by(|a, b| b > a)) {
            total_unsafe += 1;
            continue
        }

        let mut last = 0;
        for (index, cv) in lv.iter().enumerate() {
            if index == 0 {
                last = *cv;
                continue
            } else if (last - cv).abs() > 3 {
                total_unsafe += 1;
                break
            }
            last = *cv;
        }
    }

    println!("Total unsafe: {}", total_unsafe);
    println!("Total safe {}", line_vectors.len() - total_unsafe as usize);

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}