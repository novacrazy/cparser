pub mod parsing {
    use nom::*;

    named!(consume_null<()>, fold_many0!(anychar, (), |_, _| ()));

    named!(block_comment_raw<()>, delimited!(tag!("/*"), consume_null, tag!("*/")));

    //named!(filter_strings, )

    #[cfg(test)]
    mod test {
        #[test]
        fn test_block_comments() {}
    }
}