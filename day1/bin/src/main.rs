use lib::id_checker::check_ids_offset;

// Solution https://adventofcode.com/2024/day/1

fn main() {
    let left_ids: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
    let right_ids: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
    let result = check_ids_offset(&left_ids, &right_ids);
    
    match result {
        Ok(valid_result) => { println!("Offset value: {}", valid_result) }
        Err(error) => { println!("{}", error.to_string()) }
    }
}
