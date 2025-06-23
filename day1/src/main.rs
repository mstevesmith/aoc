use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num;

// fn main() {
//     let mut total_sum: i32 = 0;
//     // File hosts.txt must exist in the current path
//     if let Ok(lines) = read_lines("./input") {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines.map_while(Result::ok) {
//             //println!("{}", line);

//             let mut parts = line.split_whitespace();

//             let p1 = parts.next().unwrap().parse::<i32>().unwrap();
//             let p2 = parts.next().unwrap().parse::<i32>().unwrap();

//             println!("The two strings are {} and {}, entire line is: {}, difference is: {}", p1, p2, line, (p2 - p1).abs());
//             total_sum = total_sum + (p2 - p1).abs();
//             //total_sum = total_sum + (parts.next().unwrap().parse::<i32>().unwrap() - parts.next().unwrap().parse::<i32>().unwrap()).abs();
//         }
//     }
//     println!("Final sum is {}", total_sum);
// }
fn main() {
    let mut total_sum: i32 = 0;
    let mut first_column: Vec<i32> = vec![];
    let mut second_column: Vec<i32> = vec![];
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            //println!("{}", line);

            let mut parts = line.split_whitespace();

            let p1 = parts.next().unwrap().parse::<i32>().unwrap();
            let p2 = parts.next().unwrap().parse::<i32>().unwrap();
            first_column.push(p1);
            second_column.push(p2);

            // println!("The two strings are {} and {}, entire line is: {}, difference is: {}", p1, p2, line, (p2 - p1).abs());
            // total_sum = total_sum + (p2 - p1).abs();
            //total_sum = total_sum + (parts.next().unwrap().parse::<i32>().unwrap() - parts.next().unwrap().parse::<i32>().unwrap()).abs();
        }
    }

    first_column.sort();
    second_column.sort();

    for (index, element) in first_column.iter().enumerate() {
        total_sum += (second_column[index] - element).abs();
    }
    println!("Final sum is {}", total_sum);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}