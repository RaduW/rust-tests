/// Tests building a multipart stream from a string
use bytes::Bytes;

use actix_web::actix::ResponseFuture;
use actix_web::dev::Payload;
use actix_web::error::MultipartError;
use actix_web::multipart::{self, Multipart, MultipartItem};
use actix_web::{error::PayloadError, multipart::Field};
use futures::future::FutureResult;
use futures::{
    self,
    future::{self},
    Future, Stream,
};

fn main() {
    println!("Hello from multipart stream");
}

fn detect_embeded_minidump_content(data: &str) -> Option<&str> {
    let data = data.trim_start_matches("\r\n");
    if data.starts_with("--") {
        if let Some(line) = data.lines().next() {
            return Some(&line[2..]);
        }
    }
    None
}

fn get_multipart_boundary(data: &[u8]) -> Option<&str> {
    let mut idx: usize = 0;

    let len = data.len();
    while idx < len && (data[idx] == b'\n' || data[idx] == b'\r') {
        idx = idx + 1;
    }

    if idx + 2 > len || data[idx..idx + 2] != [b'-', b'-'] {
        return None;
    }

    let start = idx + 2;

    while idx < len && data[idx] != b'\n' && data[idx] != b'\r' {
        idx = idx + 1;
    }

    if start == idx {
        None
    } else {
        std::str::from_utf8(&data[start..idx]).ok()
    }
}

struct FieldResult;

fn process_field<S>(field: Field<S>) -> impl Future<Item = FieldResult, Error = ()>
where
    S: Stream<Item = Bytes, Error = PayloadError>,
{
    if let Some(cd) = field.content_disposition() {
        println!("the content disposition is : {}", cd);
    }

    let ret_val = field.fold::<_, _, FutureResult<_, PayloadError>>(FieldResult, |result, data| {
        println!("we have data");
        future::ok(result)
    });

    ret_val.map_err(|_| ())
}

fn file_content() -> String {
    let content = include_str!("electron_simple.body.txt");
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::multipart::{Multipart, MultipartItem};

    #[test]
    fn test_get_boundary() {
        let examples: &[(&[u8], Option<&str>)] = &[
            (b"--some_val", Some("some_val")),
            (b"--\nsecond line", None),
            (b"\n\r--some_val", Some("some_val")),
            (b"\n\r--some_val\nadfa", Some("some_val")),
            (b"\n\r--some_val\rfasdf", Some("some_val")),
            (b"\n\r--some_val\r\nfasdf", Some("some_val")),
            (b"\n\rsome_val", None),
            (b"", None),
            (b"--", None),
        ];

        for (input, expected) in examples {
            let boundary = get_multipart_boundary(input);
            assert_eq!(*expected, boundary);
        }
    }

    #[test]
    fn test_extract_multipart() {
        let content = file_content();
        let boundary = detect_embeded_minidump_content(&content)
            .unwrap()
            .to_string();
        println!("the boundary is:{:?}", boundary);
        let content_bytes = Bytes::from(content);
        let content = futures::stream::once(Ok(content_bytes));
        let multipart = Multipart::new(Ok(boundary), content);

        let multi = multipart.wait();

        for item in multi {
            match item {
                Ok(MultipartItem::Field(x)) => {
                    println!("We got a field");
                    process_field(x).wait();
                }
                Ok(MultipartItem::Nested(x)) => {
                    println!("We got a nested element");
                }
                Err(x) => {
                    println!("We got an error '{:?}'", x);
                    break;
                }
            }
        }
        println!("At the end");
    }
}
