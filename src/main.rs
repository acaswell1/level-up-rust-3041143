fn find_median(list_values : Vec<f32>) -> Option<f32> {
    let list_len = list_values.len();
    
    if list_len == 0 {
        return None;
    }

    return match list_len % 2 {
        0 => {
            let left = list_values.get(list_len / 2 - 1).unwrap();
            let right = list_values.get(list_len / 2).unwrap();
            Some((left + right) / 2.0)
        },
        1 => {Some(list_values[list_len / 2])},
        _ => {None}
    };

    
}


fn main() {
    let odd_test: Vec<f32> = [1.0, 1.5, 1.9, 2.3, 9.7].to_vec();
    let even_test: Vec<f32> = [1.0, 1.5, 1.9, 2.3, 9.7, 18.6].to_vec();
    let empty_test: Vec<f32> = [].to_vec();

    assert_eq!(Some(1.9), find_median(odd_test));
    assert_eq!(Some(2.1), find_median(even_test));
    assert_eq!(None, find_median(empty_test));
    
    println!("All testd passed!");
}
