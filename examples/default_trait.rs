fn main() {
    println!("The function default_trait.");
}

trait MyTrait {
    fn f1(&self, a: i32) -> i32;
    fn f2(&self, a: i32) -> i32 {
        return a;
    }
}

struct X;

impl MyTrait for X {
    fn f1(&self, a: i32) -> i32 {
        a
    }
    fn f2(&self, a: i32) -> i32 {
        a + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait() {
        let x = X;

        println!(" f1={}, f2={}", x.f1(1), x.f2(1));
    }
}
