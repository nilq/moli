use super::Traveler;

use super::{Expression, Statement, Operand, Type, operand};

use lexer::TokenType;

pub struct Parser {
    traveler: Traveler,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(traveler: Traveler) -> Parser {
        Parser {
            traveler,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stack = Vec::new();
        while self.traveler.remaining() > 1 {
            stack.push(self.statement());
            self.traveler.next();
        }
        stack
    }

    #[allow(unused_must_use)]
    fn statement(&mut self) -> Statement {
        match self.traveler.current().token_type {
            TokenType::Type => {
                let t = self.traveler.current_content();
                self.traveler.next();
                let id = self.traveler.expect(TokenType::Identifier).unwrap();
                self.traveler.next();
                self.traveler.expect_content("=");
                self.traveler.next();
                let expr = self.expression();

                Statement::Definition(id, Box::new(expr), Type::from_str(&t))
            },
            TokenType::Keyword => match self.traveler.current_content().as_str() {
                "return" => {
                    self.traveler.next();
                    if self.traveler.current_content() == "\n" {
                        self.traveler.next(); // skip \n
                        Statement::Return(None)
                    } else {
                        let expr = self.expression();
                        self.traveler.next(); // skip \n
                        Statement::Return(Some(Box::new(expr)))
                    }
                },

                _ => panic!("unexpected keyword: {}", self.traveler.current_content()),
            },
            _ => Statement::Expression(Box::new(self.expression())),
        }
    }

    fn expression(&mut self) -> Expression {
        let expr = self.atom();
        self.traveler.next();
        if self.traveler.remaining() > 0 {
            if self.traveler.current().token_type == TokenType::Operator {
                return self.operation(expr)
            }
            self.traveler.prev();
        }
        expr
    }

    fn block(&mut self) -> Vec<Statement> {
        match self.traveler.current().token_type {
            TokenType::Block(ref v) => {
                let mut p = Parser::new(Traveler::new(v.clone()));
                p.parse()
            },
            _ => panic!("expected block, found: {}", self.traveler.current_content()),
        }
    }

    #[allow(unused_must_use)]
    fn atom(&mut self) -> Expression {
        match self.traveler.current().token_type.clone() {
            TokenType::EOL => {
                self.traveler.next();
                match self.traveler.current().token_type {
                    TokenType::Block(_) => Expression::Block(Box::new(self.block())),
                    _                   => Expression::EOF,
                }
            },
            TokenType::IntLiteral    => Expression::IntLiteral(self.traveler.current_content().parse::<i64>().unwrap()),
            TokenType::FloatLiteral  => Expression::FloatLiteral(self.traveler.current_content().parse::<f64>().unwrap()),
            TokenType::BoolLiteral   => Expression::BoolLiteral(self.traveler.current_content() == "true"),
            TokenType::StringLiteral => Expression::StringLiteral(self.traveler.current_content().clone()),
            TokenType::Identifier    => Expression::Identifier(self.traveler.current_content()),
            TokenType::Symbol => match self.traveler.current_content().as_str() {
                "(" => {
                    self.traveler.next();
                    let expr = self.expression();
                    self.traveler.next();
                    self.traveler.expect_content(")");
                    
                    expr
                },
                "[" => {
                    self.traveler.next();

                    let mut ret = Type::Any;
                    let mut params = Vec::new();

                    loop {
                        match self.traveler.current().token_type {
                            TokenType::Identifier => {
                                params.push((Type::Any, self.traveler.current_content()));
                                self.traveler.next(); // skip id
                            },
                            TokenType::Type => {
                                let t = self.traveler.current_content();
                                self.traveler.next();
                                let id = self.traveler.expect(TokenType::Identifier).unwrap();
                                self.traveler.next();

                                params.push((Type::from_str(&t), id))
                            },
                            TokenType::Symbol => match self.traveler.current_content().as_str() {
                                ","  => { self.traveler.next(); },
                                "->" => {
                                    self.traveler.next();
                                    let t = self.traveler.expect(TokenType::Type).unwrap();
                                    self.traveler.next();
                                    
                                    ret = Type::from_str(&t)
                                },
                                "]" => {
                                    self.traveler.next();
                                    break
                                },
                                s => panic!("unexpected symbol: {}", s),
                            },
                            _ => panic!("unexpected '{}' in function signature", self.traveler.current_content()) 
                        }
                    }

                    let body: Vec<Statement>;

                    if self.traveler.current_content() == "\n" {
                        self.traveler.next();
                        body = self.block();
                    } else {
                        body = vec!(Statement::Expression(Box::new(self.expression())));
                    }

                    Expression::Function {
                        params,
                        ret,
                        body,
                    }
                },
                _ => panic!("unexpected symbol: {}", self.traveler.current_content()),
            },
            _ => panic!("unexpected: '{}'", self.traveler.current_content()),
        }
    }

    fn operation(&mut self, expression: Expression) -> Expression {
        let mut ex_stack = vec![expression];
        let mut op_stack: Vec<(Operand, u8)> = Vec::new();

        op_stack.push(operand(&self.traveler.current_content()).unwrap());

        self.traveler.next();

        ex_stack.push(self.atom());

        let mut done = false;
        while ex_stack.len() > 1 {
            if !done && self.traveler.next() {
                if self.traveler.current().token_type != TokenType::Operator {
                    self.traveler.prev();
                    done = true;
                    continue
                }

                let (op, precedence) = operand(&self.traveler.current_content()).unwrap();

                if precedence >= op_stack.last().unwrap().1 {
                    let left  = ex_stack.pop().unwrap();
                    let right = ex_stack.pop().unwrap();

                    ex_stack.push(Expression::Operation {
                        right:  Box::new(left),
                        op:    op_stack.pop().unwrap().0,
                        left: Box::new(right)
                    });

                    self.traveler.next();

                    ex_stack.push(self.atom());
                    op_stack.push((op, precedence));

                    continue
                }

                self.traveler.next();

                ex_stack.push(self.atom());
                op_stack.push((op, precedence));
            }

            let left  = ex_stack.pop().unwrap();
            let right = ex_stack.pop().unwrap();

            ex_stack.push(Expression::Operation {
                right:  Box::new(left),
                op:    op_stack.pop().unwrap().0,
                left: Box::new(right)
            });
        }

        ex_stack.pop().unwrap()
    }
}