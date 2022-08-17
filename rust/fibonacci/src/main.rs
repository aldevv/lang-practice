fn fibonacci(n: i32) -> i32{
    if n == 0 {
        return 1
    }
    if n == 1 {
        return 1
    }
    let mut cur = 2;
    let mut prev = 1;
    for _ in {2..n}.into_iter() {
        let tmp = cur;
        cur += prev;
        prev = tmp;
    }
    return cur
}

fn main() {
    println!("{}", fibonacci(4));
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(6), 13);
    assert_eq!(fibonacci(7), 21);
    assert_eq!(fibonacci(8), 34);
}
