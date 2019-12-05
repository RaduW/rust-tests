/*
    $Id$
*/

/// Tests building a multipart stream from a string
use bytes::Bytes;

use futures;
use regex;
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
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

fn file_content() -> String {
    let content = include_str!("electron_simple.body.txt");
    content.to_string()
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
