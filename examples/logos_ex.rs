use logos::Logos;
use tracing::{Level, debug};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token
{
        // Tokens can be literal strings, of any length.
        #[token("fast")]
        Fast,

        #[token(".")]
        Period,

        // Or regular expressions.
        #[regex("[a-zA-Z]+")]
        Text,
}
//  C r e a t e   r i d i c u l o u s l y   f a s t   L e x e r s .
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2
// 0 1 2 3 4 5 _
//               7 8 9 0 1 2 3 4 5 6 7 8 _
//                                         0 1 2 3 _
//                                                   5 6 7 8 9 0 1 _

fn main() -> Result<(), Box<dyn std::error::Error>>
{
        tracing_subscriber::fmt().with_max_level(Level::DEBUG).pretty().init();
        let mut lex = Token::lexer("Create ridiculously fast Lexers.");
        debug!(?lex);

        for i in 0.. {
                let token = lex.next();
                let span = lex.span();
                let slice = lex.slice();
                debug!(i, ?token, ?span, ?slice);
                if token.is_none() {
                        break;
                }
        }

        // assert_eq!(lex.next(), None);
        Ok(())
}
