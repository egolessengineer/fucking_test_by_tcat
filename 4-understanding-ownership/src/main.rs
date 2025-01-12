fn main() {
    // println!("Hello, world!");
    // Test append_to_vector
    let mut vec1 = vec![1, 2, 3];
    append_to_vector(&mut vec1, 4);
    assert_eq!(vec1, vec![1, 2, 3, 4]);

    let mut vec2 = vec![10, 20, 30];
    append_to_vector(&mut vec2, 40);
    assert_eq!(vec2, vec![10, 20, 30, 40]);

    // Test calculate_average
    let vec3 = vec![1, 2, 3, 4];
    assert_eq!(calculate_average(&vec3), 2.5);

    println!("All tests passed")
}

fn append_to_vector(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);
}

fn calculate_average(vec: &Vec<i32>) -> f64 {
    let sum: i32 = vec.iter().sum();
    let count = vec.len() as f64;
    sum as f64 / count
}
