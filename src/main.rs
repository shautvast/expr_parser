use std::iter::Peekable;
use std::str::Chars;

fn main() {
    println!("{:?}", parse("1+2*62+sin(90)" ));
}

pub fn parse(exp: &str) -> f64 {
    let mut parser = Parser {
        expr: &mut exp.chars().peekable(),
    };
    parser.parse()
}

struct Parser<'a> {
    expr: &'a mut Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> f64 {
        self.parse_expression()
    }

    fn next_char(&mut self) {
        self.expr.next();
    }

    fn eat(&mut self, char_to_eat: char) -> bool {
        while self.peek() == ' ' {
            self.next_char();
        }
        if self.peek() == char_to_eat {
            self.next_char();
            return true;
        }
        false
    }

    fn peek(&mut self) -> char{
        match self.expr.peek() {
            Some(e) => { *e }
            None => '\u{0}'
        }
    }

    fn parse_expression(&mut self) -> f64 {
        let mut x = self.parse_term();
        loop {
            if self.eat('+') {
                x += self.parse_term();
            } else if self.eat('-') {
                x -= self.parse_term();
            } else { return x; }
        }
    }

    fn parse_term(&mut self) -> f64 {
        let mut x = self.parse_factor();
        loop {
            if self.eat('*') {
                x *= self.parse_factor();
            } else if self.eat('/') {
                x /= self.parse_factor();
            } else { return x; }
        }
    }

    fn parse_factor(&mut self) -> f64 {
        if self.eat('+') {
            return self.parse_factor(); // unary plus
        }
        if self.eat('-') {
            return -self.parse_factor(); // unary minus
        }
        let mut x: f64;

        if self.eat('(') { // parentheses
            x = self.parse_expression();
            self.eat(')');
        } else if self.peek().is_numeric() || self.peek() == '.' { // numbers
            let mut buffer = String::new();

            while self.peek().is_numeric() || self.peek() == '.' {
                buffer.push(self.peek());
                self.next_char();
            }

            x = buffer.parse().unwrap();
        } else if self.peek().is_alphabetic() { // functions
            let mut func = String::new();
            while self.peek().is_alphabetic() {
                func.push(self.peek());
                self.next_char();
            }

            x = self.parse_factor();
            x = match func.as_str(){
                "sqrt" => x.sqrt(),
                "sin" => x.to_radians().sin(),
                "cos" => x.to_radians().cos(),
                "tan" => x.to_radians().tan(),
                _ => panic!(format!("Unknown function: {}", func))
            };
        } else {
            panic!(format!("Unexpected: {}", self.peek()));
        }

        if self.eat('^') {
            x = x.powf(self.parse_factor()); // exponentiation
        }
        x
    }
}