
extern crate collections;

use std::char::is_alphanumeric;
use std::io::{IoResult, IoError, Buffer};
use std::string::String;

pub enum Node {
    Atom(String),
    ParseError(String)
}

pub fn parse(source: &mut Buffer) -> IoResult<Node> {
    match source.read_char() {
        Ok(c) => {
            if is_alphanumeric(c) {
                // Read an atom.
                let mut atom = String::new();
                atom.push_char(c);

                loop {
                    match source.read_char() {
                        Ok(c) => {
                            if is_alphanumeric(c) || c == '_'
                            {
                                atom.push_char(c); 
                            }
                            else 
                            { 
                                // TODO: save c?
                                break;
                            }
                        }

                        Err(e) => return Err(e)
                    }
                }

                return Ok(Atom(atom));
            }
            else
            {
                return Err(IoError { kind: std::io::OtherIoError, desc: "Unknown error", detail: None});
            }
        }

        Err(e) => return Err(e)
    }
}


