use std::collections::HashMap;

fn mean(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for v in vec {
        sum += v;
    };
    sum / vec.len() as i32
}

fn median(vec: &Vec<i32>) -> i32 {
    let ans = vec.len() / 2;
    let mut clone_vec = vec.clone();
    clone_vec.sort();
    clone_vec[ans]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for &v in vec {
        *map.entry(v).or_insert(0) += 1;
    }

    // if is iter() it returns mutable or inmutable reference
    // if is into_iter() it returns the value
    map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap()
}

fn main() {
    let vec = vec![10, 20, 20, 1, 1, 30, 0, 1, 3, 3, 2];
    let mean = mean(&vec);
    let median = median(&vec);
    let mode = mode(&vec);
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}
