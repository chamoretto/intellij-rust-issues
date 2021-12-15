#![allow(dead_code)]

fn main() {
    {
        use std::fs::File;
        use std::io::Read;

        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <File as Read>::read_to_string(&mut file, &mut buf);
    }

    {
        use std::io::Read;

        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <std::fs::File as Read>::read_to_string(&mut file, &mut buf);
    }

    {
        use std::fs;
        use std::io::Read;

        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <fs::File as Read>::read_to_string(&mut file, &mut buf);
    }

    pub mod some {
        pub mod path {
            pub struct StructForTest {
                pub(crate) inner: u8,
            }
        }
    }

    pub mod another {
        pub mod inner {
            pub trait TraitForTest {
                fn no_params(&self);
            }
        }
    }
    {
        impl another::inner::TraitForTest for some::path::StructForTest {
            fn no_params(&self) {}
        }
    }
    // some other successful examples
    let _nothing = <some::path::StructForTest as another::inner::TraitForTest>::no_params(
        &some::path::StructForTest { inner: 1 },
    );
    let _c = <u8 as core::convert::TryFrom<i8>>::try_from(7_i8).unwrap();
    let _a = <String as std::string::ToString>::to_string(&"".to_owned());

    {
        use std::fs::File;

        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <File as std::io::Read>::read_to_string(&mut file, &mut buf);
    }

    {
        use std::fs;
        use std::io;

        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <fs::File as io::Read>::read_to_string(&mut file, &mut buf);
    }

    {
        use std::fs::File;
        use std::io;

        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <File as io::Read>::read_to_string(&mut file, &mut buf);
    }

    {
        let mut buf = String::new();
        let mut file = std::fs::File::open("example.txt").unwrap();
        let _read_result = <std::fs::File as std::io::Read>::read_to_string(&mut file, &mut buf);
    }
}
