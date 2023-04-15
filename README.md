| From           | To              | Use                                   | Comment                                                                       |
|----------------|----------------|---------------------------------------|-------------------------------------------------------------------------------|
| &str           | String         | String::from(st)                      |                                                                               |
| &str           | &[u8]          | st.as_bytes()                         |                                                                               |
| &str           | Vec<u8>        | st.as_bytes().to_owned()              | via &[u8]                                                                     |
| &str           | &OsStr         | OsStr::new(st)                        |                                                                               |
| String         | &str           | &s or s.as_str()                      |                                                                               |
| String         | &[u8]          | s.as_bytes()                          |                                                                               |
| String         | Vec<u8>        | s.into_bytes()                        |                                                                               |
| String         | OsString       | OsString::from(s)                     |                                                                               |
| &[u8]          | &str           | str::from_utf8(u).unwrap()            |                                                                               |
| &[u8]          | String         | String::from_utf8(u).unwrap()         |                                                                               |
| &[u8]          | Vec<u8>        | u.to_owned()                          |                                                                               |
| &[u8]          | &OsStr         | OsStr::from_bytes(u)                  | use std::os::unix::ffi::OsStrExt;                                             |
| [u8; 3]        | &[u8]          | &b[..]                                | byte literal                                                                  |
| [u8; 3]        | &[u8]          | "foo".as_bytes()                      | alternative via UTF-8 literal                                                |
| Vec<u8>        | &str           | str::from_utf8(&v).unwrap()            | via &[u8]                                                                     |
| Vec<u8>        | String         | String::from_utf8(v)                  |                                                                               |
| Vec<u8>        | &[u8]          | &v                                    |                                                                               |
| Vec<u8>        | OsString       | OsString::from_vec(v)                 | use std::os::unix::ffi::OsStringExt;                                          |
| &OsStr         | &str           | ost.to_str().unwrap()                 |                                                                               |
| &OsStr         | String         | ost.to_os_string().into_string().unwrap() | via OsString                                                             |
| &OsStr         | Cow<str>       | ost.to_string_lossy()                 | Unicode replacement characters                                               |
| &OsStr         | OsString       | ost.to_os_string()                     |                                                                               |
| &OsStr         | &[u8]          | ost.as_bytes()                        | use std::os::unix::ffi::OsStringExt;                                          |
| OsString       | &str           | os.to_str().unwrap()                  |                                                                               |
| OsString       | &OsStr         | os.as_os_str()                        |                                                                               |
| OsString       | Vec<u8>        | os.into_vec()                         | use std::os::unix::ffi::OsStringExt;                                          |
| Path           | String         | p.to_str().unwrap().to_string()        |                                                                               |
| PathBuf        | String         | pb.to_str().unwrap().to_string()       |                                                                               |
| String         | PathBuf        | PathBuf::from(s)                      |                                                                               |
| &Path          | &str           | p.to_str().unwrap()                   |                                                                               |
| &Path          | String         | p.to_str().unwrap().to_string()        |                                                                               |
| &Path          | PathBuf        | p.to_path_buf()                       |

