/// Tests building a multipart stream from a string
use bytes::Bytes;

use futures;
use regex;
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};
use tokio::runtime::current_thread::Runtime;

//use d_web::multipart::Multipart;

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

fn file_content() -> String {
    let content = include_str!("electron_simple.body.txt");
    //let cleaned = content.replace('\n', "\r\n");
    //println!("{}", cleaned);
    //cleaned
    content.to_string()
}

fn clean_end_of_line(val: &str) -> Cow<str> {
    let r = regex::Regex::new("\r|\n|\r\n").unwrap();
    r.replace_all(val, "\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Payload;
    use actix_web::multipart::{Multipart, MultipartItem};
    use futures::{
        future::{lazy, result},
        Future, Stream,
    };
    use tokio::runtime::current_thread::Runtime;

    #[test]
    fn test_clean_end_of_line() {
        let my_str = "first line\nsecond line\rthird line\r\nfourth line\n\r\n\r\r\nfifth line";
        let result = clean_end_of_line(my_str);
        let expected_result =
            "first line\r\nsecond line\r\nthird line\r\nfourth line\r\n\r\n\r\n\r\nfifth line";
        assert_eq!(result, expected_result);
    }

    //    #[test]
    //    fn test_boundary() {
    //        let content = file_content();
    //        let content_marker = detect_embeded_minidump_content(&content);
    //        let expected_marker = "0eede1e790b8498a";
    //        assert_eq!(Some(expected_marker), content_marker)
    //    }

    #[test]
    fn test_extract_multipart() {
        let content = file_content();
        let boundary = detect_embeded_minidump_content(&content)
            .unwrap()
            .to_string();
        println!("the boundary is:{:?}", boundary);
        let content_bytes = Bytes::from(content);
        let multipart = Multipart::new(
            Ok("0eede1e790b8498a".to_string()),
            futures::stream::iter_ok(vec![content_bytes]),
        );

        println!("before wait");
        let multi = multipart.wait();
        println!("after wait");

        for item in multi {
            match item {
                Ok(MultipartItem::Field(x)) => {
                    println!("We got a field");
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

    #[test]
    fn test_multipart2() {
        /*
                let bytes = Bytes::from(
                    "testasdadsad\r\n\
                     --abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
                     Content-Disposition: form-data; name=\"file\"; filename=\"fn.txt\"\r\n\
                     Content-Type: text/plain; charset=utf-8\r\nContent-Length: 4\r\n\r\n\
                     test\r\n\
                     --abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
                     Content-Type: text/plain; charset=utf-8\r\nContent-Length: 4\r\n\r\n\
                     data\r\n\
                     --abbc761f78ff4d7cb7573b5a23f96ef0--\r\n",
                );
        */
        /*
                let bytes = Bytes::from(
                    "--abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
                     Unknown-Header: some value\r\n\r\n\
                     test\r\n\
                     --abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
                     Content-Type: text/plain; charset=utf-8\r\nContent-Length: 4\r\n\r\n\
                     data\r\n\
                     --abbc761f78ff4d7cb7573b5a23f96ef0--\r\n",
                );
        */
        let bytes = Bytes::from(
            "--0eede1e790b8498a\r\n\
             Hello: 5\r\n\r\n\
             22385\r\n\
             --0eede1e790b8498a\r\n\
             Unknwon-Header: 5\r\n\r\n\
             linux\r\n\
             --0eede1e790b8498a\r\n\
             Content-Disposition: form-data;\r\n\r\n\
             browser\r\n\
             --0eede1e790b8498a--\r\n",
        );

        let mut multipart = Multipart::new(
            Ok("0eede1e790b8498a".to_owned()),
            futures::stream::once(Ok(bytes)),
        );

        println!("before wait");
        let multi = multipart.wait();
        println!("after wait");

        for item in multi {
            match item {
                Ok(MultipartItem::Field(x)) => {
                    println!("We got a field");
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

    //    #[test]
    //    fn test_multipart() {
    //        Runtime::new()
    //            .unwrap()
    //            .block_on(lazy(|| {
    //                let (mut sender, payload) = Payload::new(false);
    //
    //                let bytes = Bytes::from(
    //                    "testasdadsad\r\n\
    //                     --abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
    //                     Content-Disposition: form-data; name=\"file\"; filename=\"fn.txt\"\r\n\
    //                     Content-Type: text/plain; charset=utf-8\r\nContent-Length: 4\r\n\r\n\
    //                     test\r\n\
    //                     --abbc761f78ff4d7cb7573b5a23f96ef0\r\n\
    //                     Content-Type: text/plain; charset=utf-8\r\nContent-Length: 4\r\n\r\n\
    //                     data\r\n\
    //                     --abbc761f78ff4d7cb7573b5a23f96ef0--\r\n",
    //                );
    //                sender.feed_data(bytes);
    //
    //                let mut multipart =
    //                    Multipart::new(Ok("abbc761f78ff4d7cb7573b5a23f96ef0".to_owned()), payload);
    //                match multipart.poll() {
    //                    Ok(Async::Ready(Some(item))) => match item {
    //                        MultipartItem::Field(mut field) => {
    //                            {
    //                                use http::header::{DispositionParam, DispositionType};
    //                                let cd = field.content_disposition().unwrap();
    //                                assert_eq!(cd.disposition, DispositionType::FormData);
    //                                assert_eq!(cd.parameters[0], DispositionParam::Name("file".into()));
    //                            }
    //                            assert_eq!(field.content_type().type_(), mime::TEXT);
    //                            assert_eq!(field.content_type().subtype(), mime::PLAIN);
    //
    //                            match field.poll() {
    //                                Ok(Async::Ready(Some(chunk))) => assert_eq!(chunk, "test"),
    //                                _ => unreachable!(),
    //                            }
    //                            match field.poll() {
    //                                Ok(Async::Ready(None)) => (),
    //                                _ => unreachable!(),
    //                            }
    //                        }
    //                        _ => unreachable!(),
    //                    },
    //                    _ => unreachable!(),
    //                }
    //
    //                match multipart.poll() {
    //                    Ok(Async::Ready(Some(item))) => match item {
    //                        MultipartItem::Field(mut field) => {
    //                            assert_eq!(field.content_type().type_(), mime::TEXT);
    //                            assert_eq!(field.content_type().subtype(), mime::PLAIN);
    //
    //                            match field.poll() {
    //                                Ok(Async::Ready(Some(chunk))) => assert_eq!(chunk, "data"),
    //                                _ => unreachable!(),
    //                            }
    //                            match field.poll() {
    //                                Ok(Async::Ready(None)) => (),
    //                                _ => unreachable!(),
    //                            }
    //                        }
    //                        _ => unreachable!(),
    //                    },
    //                    _ => unreachable!(),
    //                }
    //
    //                match multipart.poll() {
    //                    Ok(Async::Ready(None)) => (),
    //                    _ => unreachable!(),
    //                }
    //
    //                let res: Result<(), ()> = Ok(());
    //                result(res)
    //            }))
    //            .unwrap();
    //    }
}
