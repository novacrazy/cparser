/// Prematurely returns an error from the sub-parser
///
/// Useful to avoiding other combinators eating errors
#[macro_export]
macro_rules! forward_error (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        match { $submac!($i, $($args)*) } {
            ::nom::IResult::Error(e) => {
                return ::nom::IResult::Error(e)
            },
            e @ _ => e,
        }
    });

    ($i:expr, $f:expr) => (
        forward_error!($i, call!($f));
    );
);

/// Wrapper around `char!` that adds a `ParseError::InvalidPunctuation` error to it
#[macro_export]
macro_rules! punct (
    ($i:expr, $c:expr) => ({
        use $crate::parser::error::ParseError;

        add_return_error!($i,
            ParseError::InvalidPunctuation($c).into_nom(),
            fix_error!(ParseError, char!($c))
        )
    })
);

/// Wrapper around `tag!` that adds a `ParseError::InvalidKeyword` error to it
#[macro_export]
macro_rules! keyword (
    ($i:expr, $c:expr) => ({
        use $crate::parser::error::ParseError;

        add_return_error!($i,
            ParseError::InvalidKeyword($c).into_nom(),
            fix_error!(ParseError, tag!($c))
        )
    })
);