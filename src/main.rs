fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
    println!("2 + 3 = {}", add(2, 3));

    let unused = 42;

    let list = vec![1];
    if list.len() < 0 {
        println!("Universe broke.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
