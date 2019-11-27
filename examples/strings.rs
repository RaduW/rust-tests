use bytes::Bytes;

fn main() {
    println!("Hello from strings");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_strings_have_the_same_encoding_as_normal_strings() {
        let x = "hello";
        let y = b"hello";
        assert_eq!(x.as_bytes(), y);
    }

    #[test]
    fn test_split_bytes() {
        let mut a = Bytes::from(&b"hello world"[..]);
        let b = a.split_to(5);

        assert_eq!(&a[..], b" world");
        assert_eq!(&b[..], b"hello");
    }

    #[test]
    fn test_partial_order_string_bytes() {}
}
