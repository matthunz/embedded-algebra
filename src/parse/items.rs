use std::str::Chars;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Add,
    Pow,
}

#[derive(Clone, Copy, Debug)]
pub enum Item {
    Integer(i64),
    Operator(Operator),
    Variable(u8),
}

enum State {
    Init,
    Integer { start: usize },
    Item(Item),
}

pub struct Items<'a> {
    s: &'a str,
    chars: Chars<'a>,
    pos: usize,
    state: State,
}

impl<'a> Items<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            chars: s.chars(),
            pos: 0,
            state: State::Init,
        }
    }
}

impl Iterator for Items<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let State::Item(item) = self.state {
                self.state = State::Init;
                break Some(item);
            } else {
                if let Some(c) = self.chars.next() {
                    match c {
                        '0'..='9' => match self.state {
                            State::Integer { .. } => {}
                            _ => self.state = State::Integer { start: self.pos },
                        },
                        ' ' => match self.state {
                            State::Integer { start } => {
                                self.state = State::Init;

                                let a = &self.s[start..self.pos];
                                let n = a.parse::<i64>().unwrap();
                                self.pos += 1;
                                break Some(Item::Integer(n));
                            }
                            _ => {}
                        },
                        c => {
                            let item = match c {
                                '+' => Item::Operator(Operator::Add),
                                '^' => Item::Operator(Operator::Pow),
                                c => {
                                    let n = c as u8 - 97;
                                    if n <= 4 {
                                        Item::Variable(n)
                                    } else {
                                        todo!()
                                    }
                                }
                            };
                            match self.state {
                                State::Integer { start } => {
                                    self.state = State::Item(item);

                                    let a = &self.s[start..self.pos];
                                    let n = a.parse::<i64>().unwrap();
                                    self.pos += 1;
                                    break Some(Item::Integer(n));
                                }
                                _ => {
                                    self.state = State::Item(item);
                                }
                            }
                        }
                    }
                    self.pos += 1;
                } else {
                    match self.state {
                        State::Integer { start } => {
                            self.state = State::Init;
                            let a = &self.s[start..self.pos];
                            let n = a.parse::<i64>().unwrap();
                            break Some(Item::Integer(n));
                        }
                        _ => {
                            break None;
                        }
                    }
                }
            }
        }
    }
}
