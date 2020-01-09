use failure::{self, Error, Fail};

#[derive(Fail, Debug)]
#[fail(display = "Type1 here")]
pub struct Type1(i32);

#[derive(Fail, Debug)]
#[fail(display = "Type2 here")]
pub struct Type2(i64);

fn f1(val: i32) -> Result<i32, Type1> {
    if val > 10 {
        Ok(val)
    } else {
        Err(Type1(val))
    }
}
fn f2(val: i64) -> Result<i64, Type2> {
    if val > 10 {
        Ok(val)
    } else {
        Err(Type2(val))
    }
}

fn both(val1: i32, val2: i64) -> Result<i64, Error> {
    let v1 = f1(val1)?;
    let v2 = f2(val2)?;

    return Ok(v1 as i64 + v2);
}

struct X1;

fn fa1() -> X1 {
    X1
}

struct X2;
fn fa2() -> X2 {
    X2
}

struct X;

impl From<X1> for X {
    fn from(val: X1) -> X {
        X
    }
}
impl From<X2> for X {
    fn from(val: X2) -> X {
        X
    }
}

fn fa(val: i32) -> X {
    if val > 0 {
        fa1().into()
    } else {
        fa2().into()
    }
}

fn fx1() -> Result<(), X1> {
    Err(X1)
}

fn fx2() -> Result<(), X2> {
    Err(X2)
}

fn fx(val: i32) -> Result<(), X> {
    if val > 0 {
        fx1()?
    } else {
        fx2()?
    }
    Ok(())
}

fn main() {
    println!("Testing failure.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both() {
        let val = both(9, 9);
        println!("the value is {:?}", val)
    }
}
