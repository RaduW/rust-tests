/*
hmacuse hmac::{Hmac, Mac};
use regex::Regex;

#[rustfmt::skip]
macro_rules! ip {
    (v4s) => { "(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)" };
    (v4a) => { concat!(ip!(v4s), "\\.", ip!(v4s), "\\.", ip!(v4s), "\\.", ip!(v4s)) };
    (v6s) => { "[0-9a-fA-F]{1,4}" };
}

macro_rules! hmac {
    ($ty:ident) => {{
        let mut mac = Hmac::<$ty>::new_varkey(key.as_bytes()).unwrap();
        mac.input(text.as_bytes());
        format!("{:X}", mac.result().code())
    }};
}

pub fn display_pii() {
    println!("In display_pii");

    println!("v4s {}", ip!(v4s));
}

fn int_to_string() {
    let mut v: Vec<String> = [1, 2, 3].iter().map(|val| format!("{}", val)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;*//**//*

    #[test]
    fn test_ip_macro() {
        insta::assert_json_snapshot_matches!(
            ip!(v4s),
            @r###""(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)""###);
        insta::assert_json_snapshot_matches!(
            ip!(v4a),
            @r###""(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)""###);
        insta::assert_json_snapshot_matches!(
            ip!(v6s),
            @r###""[0-9a-fA-F]{1,4}""###);
    }

    #[test]
    fn test_url_password_regex() {
        let imei: Regex = Regex::new(
            r#"(?x)
            \b((?:[a-z0-9]+:)?//[a-zA-Z0-9%_.-]+:)([a-zA-Z0-9%_.-]+)@

        "#,
        )
        .unwrap();

        assert!(imei.is_match("abc://a09_:abcd@"))
    }
}
*/
