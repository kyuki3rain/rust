use std::rc::Rc;

use super::{ast, lexer, token};

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    Lowest,
    Assign,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Dot,
    Call,
    Index,
}

#[allow(dead_code)]
pub fn token_type_to_precedence(t: &token::TokenType) -> Precedence {
    match t {
        token::TokenType::Assign => Precedence::Assign,
        token::TokenType::Eq => Precedence::Equals,
        token::TokenType::NotEq => Precedence::Equals,
        token::TokenType::LtEq => Precedence::LessGreater,
        token::TokenType::GtEq => Precedence::LessGreater,
        token::TokenType::Lt => Precedence::LessGreater,
        token::TokenType::Gt => Precedence::LessGreater,
        token::TokenType::Plus => Precedence::Sum,
        token::TokenType::Minus => Precedence::Sum,
        token::TokenType::Slash => Precedence::Product,
        token::TokenType::Asterisk => Precedence::Product,
        // token::TokenType::DOT => Precedence::DOT,
        token::TokenType::LParen => Precedence::Call,
        // token::TokenType::LBRACKET => Precedence::INDEX,
        _ => Precedence::Lowest,
    }
}

#[allow(dead_code)]
pub struct Parser {
    l: lexer::Lexer,
    cur_token: Rc<token::Token>,
    peek_token: Rc<token::Token>,
    pub errors: Vec<String>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(l: lexer::Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Rc::new(token::Token {
                token_type: token::TokenType::Illegal,
                literal: "".to_string(),
            }),
            peek_token: Rc::new(token::Token {
                token_type: token::TokenType::Illegal,
                literal: "".to_string(),
            }),
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();

        p
    }

    fn peek_error(&mut self, t: token::TokenType) {
        self.errors.push(format!(
            "\r\nexpected next token to be {:?}, got {:?} instead.",
            t, self.peek_token.token_type
        ))
    }

    fn next_token(&mut self) {
        self.cur_token = Rc::clone(&self.peek_token);
        self.peek_token = Rc::new(self.l.next_token());
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            functions: Vec::new(),
        };
        while self.cur_token.token_type == token::TokenType::Function {
            if let Some(func) = self.parse_function() {
                program.functions.push(func);
            }
            self.next_token();
        }

        program
    }

    fn parse_function(&mut self) -> Option<ast::Function> {
        let mut function = ast::Function {
            name: "".to_string(),
            parameters: Vec::new(),
            body: ast::Statement::Block {
                statements: Vec::new(),
            },
        };

        if !self.expect_peek(token::TokenType::Ident) {
            return None;
        }
        function.name = self.cur_token.literal.to_string();

        if !self.expect_peek(token::TokenType::LParen) {
            return None;
        }

        function.parameters = self.parse_function_parameters()?;

        if !self.expect_peek(token::TokenType::LBrace) {
            return None;
        }

        function.body = self.parse_block_statement()?;

        Some(function)
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(&token::TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }
        self.next_token();

        identifiers.push(self.cur_token.literal.to_string());

        while self.peek_token_is(&token::TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(self.cur_token.literal.to_string());
        }

        if !self.expect_peek(token::TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.token_type {
            // token::TokenType::LET => return self.parse_let_statement(),
            token::TokenType::LBrace => self.parse_block_statement(),
            token::TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    // fn parse_let_statement(&mut self) -> Option<ast::Statement> {
    //     if !self.expect_peek(token::TokenType::IDENT) {
    //         return None;
    //     }
    //     let name = ast::Expression::Identifier {
    //         value: self.cur_token.literal.clone(),
    //     };
    //     if !self.expect_peek(token::TokenType::ASSIGN) {
    //         return None;
    //     }

    //     self.next_token();

    //     if let Some(expression) = self.parse_expression(Precedence::LOWEST) {
    //         let stmt = ast::Statement::Let {
    //             name,
    //             value: expression,
    //         };
    //         if self.peek_token_is(&token::TokenType::SEMICOLON) {
    //             self.next_token();
    //         }
    //         return Some(stmt);
    //     } else {
    //         return None;
    //     }
    // }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        self.next_token();

        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            let stmt = ast::Statement::Return {
                return_value: expression,
            };
            if self.peek_token_is(&token::TokenType::SemiColon) {
                self.next_token();
            }
            Some(stmt)
        } else {
            None
        }
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            let stmt = ast::Statement::Expression { expression };
            if self.peek_token_is(&token::TokenType::SemiColon) {
                self.next_token();
            }
            Some(stmt)
        } else {
            None
        }
    }

    fn parse_block_statement(&mut self) -> Option<ast::Statement> {
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(&token::TokenType::RBrace)
            && !self.cur_token_is(&token::TokenType::EoF)
        {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        if self.cur_token_is(&token::TokenType::EoF) {
            return None;
        }

        Some(ast::Statement::Block { statements })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        if let Some(left_exp) = self.parse_prefix_expression_fns() {
            let mut left = Box::new(left_exp);
            while !self.peek_token_is(&token::TokenType::SemiColon)
                && precedence < self.peek_precedence()
            {
                self.next_token();
                if let Some(left_exp_new) = self.parse_infix_expression_fns(left.clone()) {
                    left = Box::new(left_exp_new);
                } else {
                    return Some(*left);
                }
            }

            Some(*left)
        } else {
            self.no_prefix_parse_fn_error();

            None
        }
    }

    fn parse_prefix_expression_fns(&mut self) -> Option<ast::Expression> {
        match self.cur_token.token_type {
            token::TokenType::Ident => Some(self.parse_identifier()),
            token::TokenType::Int => self.parse_integer_literal(),
            // token::TokenType::STRING => self.parse_string_literal(),
            // token::TokenType::BANG => self.parse_prefix_expression(),
            token::TokenType::Minus => self.parse_prefix_expression(),
            // token::TokenType::TRUE => Some(self.parse_boolean()),
            // token::TokenType::FALSE => Some(self.parse_boolean()),
            token::TokenType::LParen => self.parse_grouped_expression(),
            // token::TokenType::LBRACKET => self.parse_array_literal(),
            token::TokenType::If => self.parse_if_expression(),
            token::TokenType::While => self.parse_while_expression(),
            // token::TokenType::FUNCTION => self.parse_function_literal(),
            // token::TokenType::LBRACE => self.parse_hash_literal(),
            _ => None,
        }
    }

    fn parse_infix_expression_fns(
        &mut self,
        left_exp: Box<ast::Expression>,
    ) -> Option<ast::Expression> {
        match self.cur_token.token_type {
            token::TokenType::Plus => self.parse_infix_expression(left_exp),
            token::TokenType::Minus => self.parse_infix_expression(left_exp),
            token::TokenType::Slash => self.parse_infix_expression(left_exp),
            token::TokenType::Asterisk => self.parse_infix_expression(left_exp),
            // token::TokenType::DOT => self.parse_infix_expression(left_exp),
            token::TokenType::Assign => self.parse_assign_expression(left_exp),
            token::TokenType::Eq => self.parse_infix_expression(left_exp),
            token::TokenType::NotEq => self.parse_infix_expression(left_exp),
            token::TokenType::Lt => self.parse_infix_expression(left_exp),
            token::TokenType::Gt => self.parse_infix_expression(left_exp),
            token::TokenType::LtEq => self.parse_infix_expression(left_exp),
            token::TokenType::GtEq => self.parse_infix_expression(left_exp),
            token::TokenType::LParen => self.parse_call_expression(*left_exp),
            // token::TokenType::LBRACKET => self.parse_index_expression(left_exp),
            _ => None,
        }
    }

    // fn parse_index_expression(&mut self, left: Box<ast::Expression>) -> Option<ast::Expression> {
    //     self.next_token();
    //     if let Some(index) = self.parse_expression(Precedence::LOWEST) {
    //         if !self.expect_peek(token::TokenType::RBRACKET) {
    //             return None;
    //         }

    //         return Some(ast::Expression::IndexExpression {
    //             left,
    //             index: Box::new(index),
    //         });
    //     }
    //     return None;
    // }

    fn parse_prefix_expression(&mut self) -> Option<ast::Expression> {
        let expression_operator = self.cur_token.literal.clone();

        self.next_token();

        self.parse_expression(Precedence::Prefix)
            .map(|right| ast::Expression::PrefixExpression {
                operator: expression_operator,
                right: Box::new(right),
            })
    }

    fn parse_assign_expression(&mut self, left: Box<ast::Expression>) -> Option<ast::Expression> {
        let precedence = self.cur_precedence();
        self.next_token();
        self.parse_expression(precedence)
            .map(|right| ast::Expression::AssignExpression {
                left,
                right: Box::new(right),
            })
    }

    fn parse_infix_expression(&mut self, left: Box<ast::Expression>) -> Option<ast::Expression> {
        let operator = self.cur_token.literal.clone();

        let precedence = self.cur_precedence();
        self.next_token();
        self.parse_expression(precedence)
            .map(|right| ast::Expression::InfixExpression {
                left,
                operator,
                right: Box::new(right),
            })
    }

    fn parse_call_expression(&mut self, function: ast::Expression) -> Option<ast::Expression> {
        match function {
            ast::Expression::Identifier { value } => self
                .parse_expression_list(token::TokenType::RParen)
                .map(|arguments| ast::Expression::CallExpression {
                    function: value,
                    arguments,
                }),
            _ => None,
        }
    }

    fn parse_expression_list(&mut self, end: token::TokenType) -> Option<Vec<ast::Expression>> {
        let mut args = Vec::new();

        if self.peek_token_is(&end) {
            self.next_token();
            return Some(args);
        }
        self.next_token();

        match self.parse_expression(Precedence::Lowest) {
            Some(expression) => {
                args.push(expression);
                while self.peek_token_is(&token::TokenType::Comma) {
                    self.next_token();
                    self.next_token();
                    match self.parse_expression(Precedence::Lowest) {
                        Some(exp) => args.push(exp),
                        None => return None,
                    }
                }

                if !self.expect_peek(end) {
                    return None;
                }

                Some(args)
            }
            None => None,
        }
    }

    fn parse_if_expression(&mut self) -> Option<ast::Expression> {
        if !self.expect_peek(token::TokenType::LParen) {
            return None;
        }

        self.next_token();
        match self.parse_expression(Precedence::Lowest) {
            Some(condition) => {
                if !self.expect_peek(token::TokenType::RParen) {
                    return None;
                }
                if !self.expect_peek(token::TokenType::LBrace) {
                    return None;
                }

                match self.parse_block_statement() {
                    Some(consequence) => {
                        if self.peek_token_is(&token::TokenType::Else) {
                            self.next_token();

                            if !self.expect_peek(token::TokenType::LBrace) {
                                return None;
                            }

                            match self.parse_block_statement() {
                                Some(alternative) => {
                                    let expression = ast::Expression::IfExpression {
                                        condition: Box::new(condition),
                                        consequence: Box::new(consequence),
                                        alternative: Some(Box::new(alternative)),
                                    };
                                    return Some(expression);
                                }
                                None => return Some(ast::Expression::NeedNext),
                            }
                        }

                        let expression = ast::Expression::IfExpression {
                            condition: Box::new(condition),
                            consequence: Box::new(consequence),
                            alternative: None,
                        };
                        Some(expression)
                    }
                    None => Some(ast::Expression::NeedNext),
                }
            }
            None => None,
        }
    }

    fn parse_while_expression(&mut self) -> Option<ast::Expression> {
        if !self.expect_peek(token::TokenType::LParen) {
            return None;
        }

        self.next_token();
        match self.parse_expression(Precedence::Lowest) {
            Some(condition) => {
                if !self.expect_peek(token::TokenType::RParen) {
                    return None;
                }
                if !self.expect_peek(token::TokenType::LBrace) {
                    return None;
                }

                match self.parse_block_statement() {
                    Some(consequence) => {
                        let expression = ast::Expression::WhileExpression {
                            condition: Box::new(condition),
                            consequence: Box::new(consequence),
                        };
                        Some(expression)
                    }
                    None => Some(ast::Expression::NeedNext),
                }
            }
            None => None,
        }
    }

    // fn parse_function_literal(&mut self) -> Option<ast::Expression> {
    //     if !self.expect_peek(token::TokenType::LPAREN) {
    //         return None;
    //     }

    //     match self.parse_expression_list(token::TokenType::RPAREN) {
    //         Some(parameters) => {
    //             if !self.expect_peek(token::TokenType::LBRACE) {
    //                 return None;
    //             }
    //             match self.parse_block_statement() {
    //                 Some(body) => {
    //                     return Some(ast::Expression::FunctionLiteral {
    //                         parameters,
    //                         body: Box::new(body),
    //                     })
    //                 }
    //                 None => return Some(ast::Expression::NeedNext),
    //             }
    //         }
    //         None => return None,
    //     }
    // }

    // fn parse_hash_literal(&mut self) -> Option<ast::Expression> {
    //     let mut pairs = Vec::new();

    //     while !self.peek_token_is(&token::TokenType::RBRACE) {
    //         self.next_token();
    //         let key = self.parse_expression(Precedence::LOWEST)?;

    //         if !self.expect_peek(token::TokenType::COLON) {
    //             return None;
    //         }

    //         self.next_token();
    //         let value = self.parse_expression(Precedence::LOWEST)?;
    //         pairs.push((key, value));

    //         if !self.peek_token_is(&token::TokenType::RBRACE)
    //             && !self.expect_peek(token::TokenType::COMMA)
    //         {
    //             return None;
    //         }
    //     }

    //     if !self.expect_peek(token::TokenType::RBRACE) {
    //         return None;
    //     }

    //     Some(ast::Expression::HashLiteral { pairs })
    // }

    fn parse_identifier(&self) -> ast::Expression {
        ast::Expression::Identifier {
            value: self.cur_token.literal.clone(),
        }
    }

    fn parse_integer_literal(&mut self) -> Option<ast::Expression> {
        if let Ok(value) = self.cur_token.literal.parse::<i64>() {
            Some(ast::Expression::IntegerLiteral { value })
        } else {
            self.errors.push(format!(
                "could not parse {} as integer",
                self.cur_token.literal
            ));

            None
        }
    }

    // fn parse_string_literal(&mut self) -> Option<ast::Expression> {
    //     return Some(ast::Expression::StringLiteral {
    //         value: self.cur_token.literal.clone(),
    //     });
    // }

    // fn parse_boolean(&mut self) -> ast::Expression {
    //     return ast::Expression::Boolean {
    //         value: self.cur_token_is(&token::TokenType::TRUE),
    //     };
    // }

    fn parse_grouped_expression(&mut self) -> Option<ast::Expression> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(token::TokenType::RParen) {
            return None;
        }

        exp
    }

    // fn parse_array_literal(&mut self) -> Option<ast::Expression> {
    //     if let Some(elements) = self.parse_expression_list(token::TokenType::RBRACKET) {
    //         return Some(ast::Expression::ArrayLiteral { elements });
    //     } else {
    //         return None;
    //     }
    // }

    fn cur_token_is(&self, t: &token::TokenType) -> bool {
        self.cur_token.token_type == *t
    }
    fn peek_token_is(&self, t: &token::TokenType) -> bool {
        self.peek_token.token_type == *t
    }
    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();

            true
        } else {
            self.peek_error(t);

            false
        }
    }

    fn no_prefix_parse_fn_error(&mut self) {
        self.errors.push(format!(
            "no prefix parse function for {:?} found",
            self.cur_token.token_type
        ));
    }

    fn peek_precedence(&mut self) -> Precedence {
        token_type_to_precedence(&self.peek_token.token_type)
    }

    fn cur_precedence(&mut self) -> Precedence {
        token_type_to_precedence(&self.cur_token.token_type)
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test() {
        let input = "
        fn main(a, b) {
            x = 5;
            y = 10;
            foobar = 838383;
        }
        "
        .to_string();

        let l = lexer::Lexer::new(&input);

        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{}", program);

        assert_eq!(
            program.to_string(),
            "fn main(a, b) {\r\n\tx = 5;\r\n\ty = 10;\r\n\tfoobar = 838383;\r\n}\r\n\r\n"
        );
        assert_eq!(program.functions.len(), 1);
    }

    #[test]
    fn test2() {
        let input = "
        fn add(a, b) {
            return a + b;
        }
        fn main() {
            add(1, 2);
        }
        "
        .to_string();

        let l = lexer::Lexer::new(&input);

        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{}", program);

        assert_eq!(
            program.to_string(),
            "fn add(a, b) {\r\n\treturn (a + b);\r\n}\r\n\r\nfn main() {\r\n\tadd(1, 2);\r\n}\r\n\r\n"
        );
        assert_eq!(program.functions.len(), 2);
    }
}

// #[cfg(test)]
// mod lexer_tests {
//     use super::*;
//     use counted_array::counted_array;

//     #[test]
//     fn test_let_statements() {
//         let input = "
// let x = 5;
// let y = 10;
// let foobar = 838383;
//         "
//         .to_string();

//         let l = lexer::Lexer::new(&input);
//         let mut p = Parser::new(l);

//         counted_array!(
//             let tests: [&str; _] = [
//                 "x",
//                 "y",
//                 "foobar"
//             ]
//         );

//         let program = p.parse_program();
//         check_parser_errors(p);

//         assert_eq!(program.statements.len(), 3);
//         for (i, t) in tests.iter().enumerate() {
//             let stmt = &program.statements[i];
//             test_let_statement(stmt, t);
//         }
//     }

//     #[test]
//     fn test_return_statements() {
//         let input = "
// return 5;
// return 10;
// return 993322;
//         "
//         .to_string();

//         let l = lexer::Lexer::new(&input);
//         let mut p = Parser::new(l);

//         let program = p.parse_program();
//         check_parser_errors(p);

//         assert_eq!(program.statements.len(), 3);
//         for stmt in program.statements {
//             assert!(
//                 if let ast::Statement::Return { return_value: _ } = stmt {
//                     true
//                 } else {
//                     false
//                 }
//             )
//         }
//     }

//     #[test]
//     fn test_identifier_expression() {
//         let input = "foobar;".to_string();

//         let l = lexer::Lexer::new(&input);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         assert_eq!(program.statements.len(), 1);
//         if let ast::Statement::Expression { expression } = &program.statements[0] {
//             if let ast::Expression::Identifier { value } = expression {
//                 assert_eq!(value, "foobar");
//             } else {
//                 panic!(
//                     "program.Statement[0] is not ast.Identifier. got={}",
//                     expression
//                 );
//             }
//         } else {
//             panic!(
//                 "program.Statement[0] is not ast.Expression. got={}",
//                 program.statements[0]
//             );
//         }
//     }
//     #[test]
//     fn test_integer_literal_expression() {
//         let input = "5;".to_string();

//         let l = lexer::Lexer::new(&input);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         assert_eq!(program.statements.len(), 1);
//         if let ast::Statement::Expression { expression } = &program.statements[0] {
//             if let ast::Expression::IntegerLiteral { value } = expression {
//                 assert_eq!(*value, 5 as i64);
//             } else {
//                 panic!(
//                     "program.Statement[0] is not ast.IntegerLiteral. got={}",
//                     expression
//                 );
//             }
//         } else {
//             panic!(
//                 "program.Statement[0] is not ast.Expression. got={}",
//                 program.statements[0]
//             );
//         }
//     }

//     #[test]
//     fn test_parsing_prefix_expressions() {
//         counted_array!(
//             let prefix_tests: [(&str, &str, i64); _] = [
//                 ("!5;", "!", 5),
//                 ("-15;", "-", 15),
//             ]
//         );

//         for t in prefix_tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();

//             check_parser_errors(p);
//             assert_eq!(program.statements.len(), 1);

//             if let ast::Statement::Expression { expression } = &program.statements[0] {
//                 if let ast::Expression::PrefixExpression { operator, right } = expression {
//                     assert_eq!(operator, t.1);
//                     let right_exp: ast::Expression = (**right).clone();
//                     if !test_integer_literal(&right_exp, t.2) {
//                         return;
//                     }
//                 } else {
//                     panic!();
//                 }
//             } else {
//                 panic!();
//             }
//         }
//     }

//     #[test]
//     fn test_parsing_infix_expressions() {
//         counted_array!(
//             let infix_tests: [(&str, i64, &str, i64); _] = [
//                 ("5 + 5;", 5, "+", 5),
//                 ("5 - 5;", 5, "-", 5),
//                 ("5 * 5;", 5, "*", 5),
//                 ("5 / 5;", 5, "/", 5),
//                 ("5 > 5;", 5, ">", 5),
//                 ("5 < 5;", 5, "<", 5),
//                 ("5 == 5;", 5, "==", 5),
//                 ("5 != 5;", 5, "!=", 5),
//             ]
//         );
//         for t in infix_tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();

//             check_parser_errors(p);
//             assert_eq!(program.statements.len(), 1);

//             if let ast::Statement::Expression { expression } = &program.statements[0] {
//                 if let ast::Expression::InfixExpression {
//                     left,
//                     operator,
//                     right,
//                 } = expression
//                 {
//                     assert_eq!(operator, t.2);
//                     let left_exp: ast::Expression = (**left).clone();
//                     if !test_integer_literal(&left_exp, t.1) {
//                         return;
//                     }
//                     let right_exp: ast::Expression = (**right).clone();
//                     if !test_integer_literal(&right_exp, t.3) {
//                         return;
//                     }
//                 } else {
//                     panic!();
//                 }
//             } else {
//                 panic!();
//             }
//         }
//     }

//     #[test]
//     fn test_operator_precedence_parsing() {
//         counted_array!(
//             let tests: [(&str, &str); _] = [
//                 ("a + b / c - d", "((a + (b / c)) - d)\r\n"),
//                 ("-a + b", "((-a) + b)\r\n"),
//                 ("!-a", "(!(-a))\r\n"),
//                 ("a + b + c", "((a + b) + c)\r\n"),
//                 ("a + b - c", "((a + b) - c)\r\n"),
//                 ("a * b * c", "((a * b) * c)\r\n"),
//                 ("a * b / c", "((a * b) / c)\r\n"),
//                 ("a + b / c", "(a + (b / c))\r\n"),
//                 ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)\r\n"),
//                 ("3 + 4; -5 * 5", "(3 + 4)\r\n((-5) * 5)\r\n"),
//                 ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))\r\n"),
//                 ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))\r\n"),
//                 ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))\r\n"),
//                 ("true", "true\r\n"),
//                 ("false", "false\r\n"),
//                 ("3 > 5 == false", "((3 > 5) == false)\r\n"),
//                 ("3 < 5 == true", "((3 < 5) == true)\r\n"),
//                 ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)\r\n"),
//                 ("(5 + 5) * 2", "((5 + 5) * 2)\r\n"),
//                 ("2 / (5 + 5)", "(2 / (5 + 5))\r\n"),
//                 ("-(5 + 5)", "(-(5 + 5))\r\n"),
//                 ("!(true == true)", "(!(true == true))\r\n"),
//                 ("a + add(b * c) + d", "((a + add((b * c))) + d)\r\n"),
//                 ("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))\r\n"),
//                 ("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))\r\n"),
//                 ("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4])[(b * c)]) * d)\r\n"),
//                 ("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b)[2]), (b)[1], (2 * ([1, 2])[1]))\r\n"),
//             ]
//         );

//         for t in tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();
//             check_parser_errors(p);

//             let actual = format!("{}", program);
//             assert_eq!(actual, t.1);
//         }
//     }

//     #[test]
//     fn test_boolean() {
//         counted_array!(
//             let tests: [(&str, &str); _] = [
//                 ("true;", "true\r\n"),
//                 ("false;", "false\r\n"),
//                 ("let foobar = true;", "let foobar = true;\r\n"),
//                 ("let barfoo = false", "let barfoo = false;\r\n"),
//             ]
//         );

//         for t in tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();
//             check_parser_errors(p);

//             let actual = format!("{}", program);
//             assert_eq!(actual, t.1);
//         }
//     }

//     #[test]
//     fn test_if_expression() {
//         counted_array!(
//             let tests: [(&str, &str); _] = [
//                 ("if (x < y) { x }", "if ((x < y)) {\r\n\tx\r\n}\r\n"),
//                 ("if (x < y) { x } else { y }", "if ((x < y)) {\r\n\tx\r\n} else {\r\n\ty\r\n}\r\n"),
//                 ("while (true) {}", "while (true) {\r\n}\r\n"),
//             ]
//         );

//         for t in tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();
//             check_parser_errors(p);

//             let actual = format!("{}", program);
//             assert_eq!(actual, t.1);
//         }
//     }

//     #[test]
//     fn test_parsing_array_literal() {
//         let t = "[1, 2 * 2, 3 + 3]";

//         let l = lexer::Lexer::new(t);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         if let ast::Statement::Expression {
//             expression: ast::Expression::ArrayLiteral { elements },
//         } = &program.statements[0]
//         {
//             assert_eq!(elements.len(), 3);
//             test_integer_literal(&elements[0], 1);
//             test_infix_expression(&elements[1], 2, "*", 2);
//             test_infix_expression(&elements[2], 3, "+", 3);
//         } else {
//             panic!();
//         }
//     }

//     #[test]
//     fn test_parsing_index_expressions() {
//         let t = "myArray[1 + 1]";

//         let l = lexer::Lexer::new(t);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         if let ast::Statement::Expression {
//             expression: ast::Expression::IndexExpression { left, index },
//         } = &program.statements[0]
//         {
//             if let ast::Expression::Identifier { value } = &**left {
//                 assert_eq!(value, "myArray");
//             } else {
//                 panic!("program.Statement[0] is not ast.Identifier. got={}", left);
//             }
//             test_infix_expression(&index, 1, "+", 1);
//         } else {
//             panic!();
//         }
//     }

//     #[test]
//     fn test_string_literal_expression() {
//         counted_array!(
//             let tests: [(&str, &str); _] = [
//                 ("\"Hello World!\"", "\"Hello World!\"\r\n"),
//             ]
//         );

//         for t in tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();
//             check_parser_errors(p);

//             let actual = format!("{}", program);
//             assert_eq!(actual, t.1);
//         }
//     }

//     #[test]
//     fn test_function_parameter_parsing() {
//         counted_array!(
//             let tests: [(&str, &str); _] = [
//                 ("fn() {};", "fn () {\r\n}\r\n"),
//                 ("fn(x) {};", "fn (x) {\r\n}\r\n"),
//                 ("fn(x, y, z) {};", "fn (x, y, z) {\r\n}\r\n"),
//             ]
//         );

//         for t in tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();
//             check_parser_errors(p);

//             let actual = format!("{}", program);
//             assert_eq!(actual, t.1);
//         }
//     }

//     #[test]
//     fn test_call_function() {
//         counted_array!(
//             let tests: [(&str, &str); _] = [
//                 ("add()", "add()\r\n"),
//                 ("sum(x);", "sum(x)\r\n"),
//                 ("get(x, y, z)", "get(x, y, z)\r\n"),
//             ]
//         );

//         for t in tests {
//             let l = lexer::Lexer::new(t.0);
//             let mut p = Parser::new(l);
//             let program = p.parse_program();
//             check_parser_errors(p);

//             let actual = format!("{}", program);
//             assert_eq!(actual, t.1);
//         }
//     }

//     #[test]
//     fn test_parsing_hash_literal_string_keys() {
//         let input = "{ \"one\": 1, \"two\": 2, \"three\": 3}";
//         let keys = vec!["one", "two", "three"];
//         let values = vec![1, 2, 3];

//         let l = lexer::Lexer::new(input);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         if let ast::Statement::Expression {
//             expression: ast::Expression::HashLiteral { pairs },
//         } = &program.statements[0]
//         {
//             for (i, (key, v)) in pairs.iter().enumerate() {
//                 if let ast::Expression::StringLiteral { value } = key {
//                     assert_eq!(value, keys[i]);
//                 } else {
//                     panic!();
//                 }
//                 if let ast::Expression::IntegerLiteral { value } = v {
//                     assert_eq!(value, &values[i]);
//                 } else {
//                     panic!();
//                 }
//             }
//         } else {
//             panic!();
//         }
//     }

//     #[test]
//     fn test_parsing_hash_literal_with_expressions() {
//         let input = "{ \"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}";
//         let keys = vec!["one", "two", "three"];
//         let values1 = vec![0, 10, 15];
//         let values2 = vec!["+", "-", "/"];
//         let values3 = vec![1, 8, 5];

//         let l = lexer::Lexer::new(input);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         if let ast::Statement::Expression {
//             expression: ast::Expression::HashLiteral { pairs },
//         } = &program.statements[0]
//         {
//             for (i, (key, v)) in pairs.iter().enumerate() {
//                 if let ast::Expression::StringLiteral { value } = key {
//                     assert_eq!(value, keys[i]);
//                 } else {
//                     panic!();
//                 }
//                 test_infix_expression(v, values1[i], values2[i], values3[i]);
//             }
//         } else {
//             panic!();
//         }
//     }

//     #[test]
//     fn test_parsing_empty_hash_literal() {
//         let input = "{}";

//         let l = lexer::Lexer::new(input);
//         let mut p = Parser::new(l);
//         let program = p.parse_program();
//         check_parser_errors(p);

//         if let ast::Statement::Expression {
//             expression: ast::Expression::HashLiteral { pairs },
//         } = &program.statements[0]
//         {
//             if pairs.len() != 0 {
//                 panic!();
//             }
//         } else {
//             panic!();
//         }
//     }

//     fn test_let_statement(stmt: &ast::Statement, t: &str) {
//         if let ast::Statement::Let { name, value: _ } = stmt {
//             if let ast::Expression::Identifier { value } = name {
//                 assert_eq!(value, t);
//             } else {
//                 panic!("expression does not equal to identifier.");
//             }
//         } else {
//             panic!("statement does not equal to letstatement.");
//         }
//     }

//     fn test_integer_literal(il: &ast::Expression, value: i64) -> bool {
//         if let ast::Expression::IntegerLiteral { value: integ } = il {
//             assert_eq!(*integ, value);
//             return true;
//         } else {
//             return false;
//         }
//     }

//     fn test_infix_expression(expression: &ast::Expression, l: i64, op: &str, r: i64) -> bool {
//         if let ast::Expression::InfixExpression {
//             left,
//             operator,
//             right,
//         } = expression
//         {
//             assert_eq!(operator, op);
//             let left_exp: ast::Expression = (**left).clone();
//             if !test_integer_literal(&left_exp, l) {
//                 return false;
//             }
//             let right_exp: ast::Expression = (**right).clone();
//             if !test_integer_literal(&right_exp, r) {
//                 return false;
//             }

//             return true;
//         } else {
//             panic!();
//         }
//     }

//     fn check_parser_errors(p: Parser) {
//         if p.errors.len() == 0 {
//             return;
//         }

//         let mut error_messages = "".to_string();

//         for msg in p.errors {
//             error_messages += &format!("{}\r\n", msg);
//         }
//         panic!("{}", error_messages);
//     }
// }
