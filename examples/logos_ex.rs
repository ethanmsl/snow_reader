use logos::Logos;
use tracing::{Level, debug, info};

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
        info!(?lex);
        for i in 0.. {
                let token = lex.next();
                let span = lex.span();
                let slice = lex.slice();
                debug!(i, ?token, ?span, ?slice);
                if token.is_none() {
                        break;
                }
        }

        let mut calclex = CalcToken::lexer("(13+ 05) *2 -77");
        info!(?calclex);
        for i in 0.. {
                let token = calclex.next();
                let span = calclex.span();
                let slice = calclex.slice();
                debug!(i, ?token, ?span, ?slice);
                if token.is_none() {
                        break;
                }
        }

        // // Regex '.*' is prioritized over all the specific elements ...
        // let mut xlex = XmlToken::lexer(XML_EXAMPLE);
        // info!(?xlex);
        // for i in 0.. {
        //         let token = xlex.next();
        //         let span = xlex.span();
        //         let slice = xlex.slice();
        //         debug!(i, ?token, ?span, ?slice);
        //         if token.is_none() {
        //                 break;
        //         }
        // }

        Ok(())
}

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \t\n]+")]
#[logos(error = String)]
enum CalcToken
{
        #[token("+")]
        Plus,

        #[token("-")]
        Minus,

        #[token("*")]
        Multiply,

        #[token("/")]
        Divide,

        #[token("(")]
        LParen,

        #[token(")")]
        RParen,

        #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
        Integer(isize),
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n]+")]
enum XmlToken
{
        // Tokens can be literal strings, of any length.
        #[token("<")]
        OpenTagStart,

        #[token("</")]
        CloseTagStart,

        #[token(">")]
        TagEnd,

        #[token(r"/>")]
        MonoCloseTagEnd,

        #[regex(".*")]
        Other,
}

const XML_EXAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
        <unload unload_date="2022-10-14 22:34:04">
            <sysevent_script_action action="INSERT_OR_UPDATE">
                <active>true</active>
                <event_name>x_pd_integration.add_inc_external_ref</event_name>
                <sys_replace_on_upgrade>false</sys_replace_on_upgrade>
                <sys_scope display_value="PagerDuty Platform for Real-Time Operations">39a9d9664f834e00dd657bb28110c77b</sys_scope>
            </sysevent_script_action>
        </unload>"#;