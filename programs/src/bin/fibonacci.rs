pub fn fibonacci(n: u32) -> Vec<u32> {
    let mut first_num = 0;
    let mut second_num = 1;
    let mut fib_series: Vec<u32> = vec![first_num, second_num];
    for _ in 2..n {
        let tmp = second_num;
        second_num+=first_num;
        first_num=tmp;
        fib_series.push(second_num);
    }
    fib_series
}