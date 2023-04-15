## Rust String Conversions

This table provides useful information on how to convert between different string types in Rust. Here is a summary of the conversion methods listed in the table:

1. Converting from &str:
- To String: `String::from(st)`
- To &[u8]: `st.as_bytes()`
- To Vec<u8>: `st.as_bytes().to_owned()`
- To &OsStr: `OsStr::new(st)`

2. Converting from String:
- To &str: `&s` or `s.as_str()`
- To &[u8]: `s.as_bytes()`
- To Vec<u8>: `s.into_bytes()`
- To OsString: `OsString::from(s)`

3. Converting from &[u8]:
- To &str: `str::from_utf8(u).unwrap()`
- To String: `String::from_utf8(u).unwrap()`
- To Vec<u8>: `u.to_owned()`
- To &OsStr: `OsStr::from_bytes(u)`

4. Converting from [u8; 3]:
- To &[u8]: `&b[..]` (byte literal)
- To &[u8]: `"foo".as_bytes()` (alternative via UTF-8 literal)

5. Converting from Vec<u8>:
- To &str: `str::from_utf8(&v).unwrap()` via &[u8]
- To String: `String::from_utf8(v)`
- To &[u8]: `&v`
- To OsString: `OsString::from_vec(v)`

6. Converting from &OsStr:
- To &str: `ost.to_str().unwrap()`
- To String: `ost.to_os_string().into_string().unwrap()` via OsString
- To Cow<str>: `ost.to_string_lossy()` (Unicode replacement characters)
- To OsString: `ost.to_os_string()`
- To &[u8]: `ost.as_bytes()` via `std::os::unix::ffi::OsStringExt`

7. Converting from OsString:
- To &str: `os.to_str().unwrap()`
- To &OsStr: `os.as_os_str()`
- To Vec<u8>: `os.into_vec()` via `std::os::unix::ffi::OsStringExt`
- To String: `os.into_string().unwrap()` (returns original OsString on failure)

I hope this Markdown summary helps you understand the Rust string conversions better.

