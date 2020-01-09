/// This contains tests for the standard `std::Option<T>` type

fn main() {
    println!("running option_tests file.\n Please run the tests instead");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let none: Option<i32> = None;
        let none2: Option<i32> = None;
        let one: Option<i32> = Some(1);
        let two: Option<i32> = Some(2);

        assert_eq!(none.and(none2), None);
        assert_eq!(one.and(two), Some(2));
        assert_eq!(one.and(one), Some(1));
        assert_eq!(one.and(none), None);
        assert_eq!(none.and(one), None);
    }

    #[test]
    fn test_and_then() {
        let none: Option<i32> = None;
        let one: Option<i32> = Some(1);

        assert_eq!(none.and_then(|val| -> Option<i32> { unreachable!() }), None);
        assert_eq!(one.and_then(|val| Some(val)), Some(1));
        assert_eq!(one.and_then(|val| None::<i32>), None);
    }

    #[test]
    fn test_filter() {
        let none: Option<i32> = None;
        let one: Option<i32> = Some(1);

        assert_eq!(
            none.filter(|&val| {
                unreachable!();
            }),
            None
        );
        assert_eq!(one.filter(|val| { *val == 2 }), None);
        assert_eq!(one.filter(|&val| { val == 1 }), one);
    }

    #[test]
    fn test_or() {
        let none: Option<i32> = None;
        let none2: Option<i32> = None;
        let one: Option<i32> = Some(1);
        let two: Option<i32> = Some(2);

        assert_eq!(none.or(none2), None);
        assert_eq!(none.or(one), Some(1));
        assert_eq!(one.or(none), Some(1));
        assert_eq!(one.or(two), Some(1));
    }

    #[test]
    fn test_or_else() {
        let none: Option<i32> = None;
        let one: Option<i32> = Some(1);

        assert_eq!(none.or_else(|| one), one);
        assert_eq!(one.or_else(|| unreachable!()), Some(1));
    }

    #[test]
    fn test_xor() {
        let none: Option<i32> = None;
        let none2: Option<i32> = None;
        let one: Option<i32> = Some(1);
        let two: Option<i32> = Some(2);

        assert_eq!(none.xor(none), None);
        assert_eq!(one.xor(two), None);
        assert_eq!(one.xor(none), one);
        assert_eq!(none.xor(one), one);
    }

    #[test]
    fn test_ger_or_insert() {
        let mut none: Option<i32> = None;
        let mut one: Option<i32> = Some(1);

        let y = none.get_or_insert(22);
        assert_eq!(*y, 22);
        assert_eq!(none, Some(22));
        let w = one.get_or_insert(22);
        assert_eq!(*w, 1);
        assert_eq!(one, Some(1));
    }

    #[test]
    fn test_ger_or_insert_with() {
        let mut none: Option<i32> = None;
        let mut one: Option<i32> = Some(1);

        let y = none.get_or_insert_with(|| 22);
        assert_eq!(*y, 22);
        assert_eq!(none, Some(22));
        let w = one.get_or_insert_with(|| unreachable!());
        assert_eq!(*w, 1);
        assert_eq!(one, Some(1));
    }

    #[test]
    fn test_use_option_value() {
        struct StructWithOption {
            val: Option<String>,
        };

        let mut x = StructWithOption {
            val: Some("value".to_string()),
        };

        if let Some(val) = x.val.take() {
            assert_eq!(&val, "value")
        }
        assert_eq!(x.val, None);
    }
}
