use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::num;


fn number_iterations(&element: &i32, second_column: &Vec<i32>) -> i32 {
    let count = second_column.into_iter()
                            .filter(|x| **x == element)
                            .count();
    // if count > 0 {
    //     println!("The count is {} and the element is {}", count, element);
    // }
    println!("The count is {} and the element is {}", count, element);
    count.try_into().unwrap()
}

fn main() {
    let mut similarity_score: i32 = 0;
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
        }
    }

    for element in first_column.iter() {
        let score = element*number_iterations(&element, &second_column);
        //similarity_score += element*number_iterations(&element, &second_column);
        println!("The current score is {}, the current element is {}, and the score is {}", similarity_score, element, score);
        similarity_score += score;
    }
    println!("The total similarity score is: {}", similarity_score);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}