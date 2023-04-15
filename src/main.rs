use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;


fn main() {
    // &str to String
    let st: &str = "hello";
    let s: String = String::from(st);
    println!("&str to String: {}", s);

    // &str to &[u8]
    let st: &str = "hello";
    let u: &[u8] = st.as_bytes();
    println!("&str to &[u8]: {:?}", u);

    // &str to Vec<u8>
    let st: &str = "hello";
    let v: Vec<u8> = st.as_bytes().to_owned();
    println!("&str to Vec<u8>: {:?}", v);

    // &str to &OsStr
    let st: &str = "hello";
    let os_st: &OsStr = OsStr::new(st);
    println!("&str to &OsStr: {}", os_st.to_string_lossy());







    // String to &str
    let s: String = String::from("hello");
    let st: &str = s.as_str();
    println!("String to &str: {}", st);

    // String to &[u8]
    let s: String = String::from("hello");
    let u: &[u8] = s.as_bytes();
    println!("String to &[u8]: {:?}", u);

    // String to Vec<u8>
    let s: String = String::from("hello");
    let v: Vec<u8> = s.into_bytes();
    println!("String to Vec<u8>: {:?}", v);

    // String to OsString
    let s: String = String::from("hello");
    let os_s: OsString = OsString::from(s);
    println!("String to OsString: {}", os_s.to_string_lossy());







    // &[u8] to &str
    let u: &[u8] = b"hello";
    let st: &str = std::str::from_utf8(u).unwrap();
    println!("&[u8] to &str: {}", st);

    // &[u8] to String
    let u: &[u8] = b"hello";
    let s: String = String::from_utf8(u.to_owned()).unwrap();
    println!("&[u8] to String: {}", s);

    // &[u8] to Vec<u8>
    let u: &[u8] = b"hello";
    let v: Vec<u8> = u.to_owned();
    println!("&[u8] to Vec<u8>: {:?}", v);

    // &[u8] to &OsStr
    let u: &[u8] = b"hello";
    let os_u: &OsStr = OsStr::from_bytes(u);
    println!("&[u8] to &OsStr: {}", os_u.to_string_lossy());







    // [u8; 3] to &[u8]
    let b: [u8; 3] = [104, 101, 121];
    let u: &[u8] = &b[..];
    println!("[u8; 3] to &[u8]: {:?}", u);



    // Vec<u8> to &str
    let bytes: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello" in ASCII
    let string_slice: &str = std::str::from_utf8(&bytes).unwrap();
    println!("bytes to &str: {}", string_slice);

    // Vec<u8> to OsString
    let bytes: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello" in ASCII
    let os_string: OsString = OsString::from_vec(bytes);
    println!("bytes to OsString: {:?}", os_string);

    // Vec<u8> -> &[u8]
    let bytes_vec: Vec<u8> = vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    let bytes_slice: &[u8] = &bytes_vec[..];
    println!("{:?}", bytes_slice); // [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]

    // Vec<u8> -> String
    let bytes_vec: Vec<u8> = vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    let string: String = String::from_utf8(bytes_vec).unwrap();
    println!("{}", string); // hello world



    // OsStr to &str
    let os_str: &OsStr = OsStr::new("Hello world");
    let str_slice: &str = os_str.to_str().unwrap();
    println!("Converted OsStr {:?} to &str: {}", os_str, str_slice);

    // OsStr to String
    let os_str: &OsStr = OsStr::new("Hello world");
    let string: String = os_str.to_os_string().into_string().unwrap();
    println!("Converted OsStr {:?} to String: {}", os_str, string);

    // OsStr to Cow<str>
    let os_str: &OsStr = OsStr::new("Hello world\x50\x6F\x78\x73");
    let cow: std::borrow::Cow<str> = os_str.to_string_lossy();
    println!("Converted OsStr {:?} to Cow<str>: {}", os_str, cow);

    // OsStr to OsString
    let os_str: &OsStr = OsStr::new("Hello world");
    let os_string: OsString = os_str.to_os_string();
    println!("Converted OsStr {:?} to OsString: {:?}", os_str, os_string);

    // OsStr to &[u8]
    let os_str: &OsStr = OsStr::new("Hello world");
    let bytes: &[u8] = os_str.as_bytes();
    println!("Converted OsStr {:?} to &[u8]: {:?}", os_str, bytes);



    // OsString to &str
    let os_string = OsString::from("hello world");
    let os_str_ref: &OsStr = os_string.as_ref();
    let str_ref: &str = os_str_ref.to_str().unwrap();
    println!("OsString to &str: {}", str_ref);

    // OsString to &OsStr
    let os_string = OsString::from("hello world");
    let os_str_ref: &OsStr = os_string.as_ref();
    println!("OsString to &OsStr: {}", os_str_ref.to_string_lossy());

    // OsString to Vec<u8>
    let os_string = OsString::from("hello world");
    let vec_u8: Vec<u8> = os_string.into_vec();
    println!("OsString to Vec<u8>: {:?}", vec_u8);

    // Vec<u8> to OsString
    let vec_u8 = vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    let os_string: OsString = OsString::from_vec(vec_u8);
    let os_str_ref: &OsStr = os_string.as_ref();
    println!("Vec<u8> to OsString: {}", os_str_ref.to_string_lossy());




}

