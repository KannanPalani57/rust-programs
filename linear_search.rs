fn linear_search<T: PartialEq>(arr: &[T], target: T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if *item == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let data = vec![10, 20, 30, 40, 50];
    let target = 30;

    match linear_search(&data, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }
}
