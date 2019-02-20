fn main() {
    let mut my_vector = vec![11,45,14,8,10,191,9,36,4,3,64];
    let length = my_vector.len();
    
    for outer in 0..length - 1 {
        for inner in 0..length - 1 - outer {
            if ordered_at(&my_vector, inner) {
                continue;
            } else {
                my_vector = swap(&my_vector, inner);
            }
        }
    }
    
    println!("{:?}", my_vector);
}

fn ordered_at(my_vector:&Vec<u32>, location:usize)->bool {
    my_vector[location] <= my_vector[location + 1]
}

fn swap(my_vector:&Vec<u32>, location:usize) -> Vec<u32> {
    let mut result = my_vector.clone();
    result.swap(location, location + 1);
    result
}
