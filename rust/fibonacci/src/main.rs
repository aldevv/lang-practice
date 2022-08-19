fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut cur = 1;
    let mut prev = 0;
    for _ in { 2..n }.into_iter() {
        let tmp = cur;
        cur += prev;
        prev = tmp;
    }
    return cur + prev;
}

fn main() {
    println!("{}", fibonacci(4));
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(7), 13);
    assert_eq!(fibonacci(8), 21);
}
