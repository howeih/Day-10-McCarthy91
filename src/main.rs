fn mccarthy91(mut n: i32) -> i32 {
    let mut k = 1;
    while k > 0 {
        if n > 100 {
            n -= 10;
            k -= 1;
        } else {
            n += 11;
            k += 1;
        }
    }
    n
}
fn mccarthy91_rec(n: i32) -> i32 {
    if n > 100 {
        n - 10
    } else {
        mccarthy91_rec(mccarthy91_rec(n + 11))
    }
}

fn main() {
    let m = mccarthy91(80);
    let mr =  mccarthy91_rec(80);
    print!("{} , {}", m, mr);
}
