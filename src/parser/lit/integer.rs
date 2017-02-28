pub mod parsing {
    use nom::*;

    use ::parser::error::ParseError;
    use ::parser::ident::Ident;
    use ::parser::lit::Lit;

    named!(
        #[doc = "Matches integer-like literals and consumes whitespace"],
        pub integer_like_literal<&[u8], Lit, ParseError>,
        add_return_error!(
            ParseError::InvalidIntegerLikeLiteral.into_nom(),
            wse!(alt_complete!(
                keyword!("__LINE__")            => { |_| Lit::IntegerLike(Ident::from("__LINE__"))           } |
                keyword!("__STDC__")            => { |_| Lit::IntegerLike(Ident::from("__STDC__"))           } |
                keyword!("__STDC_VERSION__")    => { |_| Lit::IntegerLike(Ident::from("__STDC_VERSION__"))   } |
                keyword!("__STDC_HOSTED__")     => { |_| Lit::IntegerLike(Ident::from("__STDC_HOSTED__"))    } |
                keyword!("__cplusplus")         => { |_| Lit::IntegerLike(Ident::from("__cplusplus"))        } |
                keyword!("__OBJC__")            => { |_| Lit::IntegerLike(Ident::from("__OBJC__"))           } |
                keyword!("__ASSEMBLER__")       => { |_| Lit::IntegerLike(Ident::from("__ASSEMBLER__"))      }
            ))
        )
    );
}