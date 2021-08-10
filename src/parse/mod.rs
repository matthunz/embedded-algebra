use std::{iter::Peekable, str::Chars};

use super::Monomial;

mod items;
pub use items::{Item, Items, Operator};

enum ParserState {
    Init,
    Integer,
    Variable { var: u8, pow: bool },
}

pub struct Parser<'a> {
    items: Items<'a>,
    mono: Option<Monomial>,
    state: ParserState,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            items: Items::new(s),
            mono: None,
            state: ParserState::Init,
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Monomial;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.items.next() {
                match self.state {
                    ParserState::Init => {
                        match item {
                            Item::Variable(var) => {
                                self.mono = Some(Monomial::default());
                                self.state = ParserState::Variable { var, pow: false };
                            }
                            Item::Integer(int) => {
                                self.state = ParserState::Integer;
                                self.mono = Some(Monomial::coefficient(int));
                            }
                            _ => {
                                // TODO Return error
                                todo!()
                            }
                        }
                    }
                    ParserState::Integer => {
                        match item {
                            Item::Variable(var) => {
                                self.state = ParserState::Variable { var, pow: false };
                            }
                            Item::Operator(Operator::Add) => {
                                self.state = ParserState::Init;
                                break self.mono.take();
                            }
                            _ => {
                                // TODO Return error
                                todo!()
                            }
                        }
                    }
                    ParserState::Variable { var, pow } => match item {
                        Item::Integer(exp) => {
                            if pow {
                                self.state = ParserState::Integer;
                                self.mono.as_mut().unwrap().exponents[var as usize] += exp as u16;
                            } else {
                                // TODO Return error
                                todo!()
                            }
                        }
                        Item::Operator(op) => {
                            match op {
                                Operator::Add => {
                                    if pow {
                                        // TODO Return error
                                        todo!()
                                    } else {
                                        self.state = ParserState::Init;
                                        let mut mono = self.mono.take().unwrap();
                                        mono.exponents[var as usize] = 1 as u16;
                                        break Some(mono);
                                    }
                                }
                                Operator::Pow => {
                                    if pow {
                                        // TODO Return error
                                        todo!()
                                    } else {
                                        self.state = ParserState::Variable { var, pow: true };
                                    }
                                }
                            }
                        }
                        Item::Variable(v2) => {
                            if !pow {
                                self.state = ParserState::Variable {
                                    var: v2,
                                    pow: false,
                                };
                                self.mono.as_mut().unwrap().exponents[var as usize] += 1;
                            } else {
                                // TODO Return error
                                todo!()
                            }
                        }
                    },
                }
            } else {
                match self.state {
                    ParserState::Variable { var, pow } => {
                        if !pow {
                            if let Some(ref mut mono) = self.mono {
                                mono.exponents[var as usize] += 1;
                            }
                            self.state = ParserState::Init;
                        } else {
                            // TODO Return error
                            todo!()
                        }
                    }
                    _ => {}
                }

                break self.mono.take();
            }
        }
    }
}

