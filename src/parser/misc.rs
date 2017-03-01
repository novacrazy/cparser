#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

pub mod parsing {
    use super::*;
    use nom::*;

    named!(pub storage_class_specifier<StorageClassSpecifier>, alt!(
        tag!("typedef")         => {|_| StorageClassSpecifier::Typedef     } |
        tag!("extern")          => {|_| StorageClassSpecifier::Extern      } |
        tag!("static")          => {|_| StorageClassSpecifier::Static      } |
        tag!("thread_local")    => {|_| StorageClassSpecifier::ThreadLocal } |
        tag!("auto")            => {|_| StorageClassSpecifier::Auto        } |
        tag!("register")        => {|_| StorageClassSpecifier::Register    }
    ));

    #[cfg(test)]
    mod test {
        use super::super::*;
        use super::*;

        use nom::*;

        use nom::IResult::Done;

        #[test]
        fn test_storage_class_specifier() {
            assert_eq!(storage_class_specifier(b"thread_local"), Done(&[] as &[u8], StorageClassSpecifier::ThreadLocal));
            assert_eq!(storage_class_specifier(b"auto"), Done(&[] as &[u8], StorageClassSpecifier::Auto));
            assert_eq!(storage_class_specifier(b"static"), Done(&[] as &[u8], StorageClassSpecifier::Static));
        }
    }
}