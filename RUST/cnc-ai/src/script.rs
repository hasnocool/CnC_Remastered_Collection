//! The AgentScript language: lexer, parser, compile-time checks and diagnostics.
//!
//! AgentScript is deliberately small and deterministic. It has no wall clock,
//! no floating point and no unbounded loops, so a compiled agent behaves the
//! same on every client of a lockstep match. Anything non-deterministic (an
//! LLM advisor, for example) is supplied out of band as a pre-recorded intent
//! and read back through the intent / intent_text builtins.
//!
//! Example:
//!
//! ~~~text
//! on start { set_stance("defensive") }
//! every 120 {
//!     if credits() < 800 { harvest() }
//!     if units("InfantryE1") < 4 { train("InfantryE1") }
//! }
//! ~~~

use std::fmt;

/// Maximum accepted source size. Protects the editor and the runtime.
pub const MAX_SOURCE_BYTES: usize = 256 * 1024;
/// Maximum number of handlers in one agent.
pub const MAX_HANDLERS: usize = 64;
/// Maximum block nesting accepted by the parser.
pub const MAX_DEPTH: usize = 16;

/// A compile-time problem with a source location (1-based line and column).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn new(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            column,
            message: message.into(),
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, " ({hint})")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostic {}

/// A parsed and checked agent program.
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub handlers: Vec<Handler>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub kind: HandlerKind,
    pub body: Block,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HandlerKind {
    /// Runs once when the match starts.
    Start,
    /// Runs every N simulation ticks. The expression is evaluated when due.
    Every(Expr),
    /// Runs when the host raises the named event (unit_lost, attacked, ...).
    Event(String),
}

pub type Block = Vec<Stmt>;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Block,
        else_body: Option<Block>,
    },
    While {
        condition: Expr,
        body: Block,
    },
    /// A bare expression, normally a builtin call such as attack(x, y).
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Int(i64),
    Str(String),
    Bool(bool),
    Var(String),
    Call(String, Vec<Expr>),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

/// Whether a builtin reads the world or issues a command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinKind {
    Query,
    Action,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Builtin {
    pub name: &'static str,
    pub kind: BuiltinKind,
    pub min_args: usize,
    pub max_args: usize,
}

/// Every function an agent script may call.
pub const BUILTINS: &[Builtin] = &[
    Builtin {
        name: "tick",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "credits",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "power",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "power_ratio",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "threat",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "enemies",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "units",
        kind: BuiltinKind::Query,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "buildings",
        kind: BuiltinKind::Query,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "map_width",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "map_height",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "enemy_x",
        kind: BuiltinKind::Query,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "enemy_y",
        kind: BuiltinKind::Query,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "nearest_enemy_x",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "nearest_enemy_y",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "nearest_threat_x",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "nearest_threat_y",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "base_x",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "base_y",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "stance",
        kind: BuiltinKind::Query,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "random",
        kind: BuiltinKind::Query,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "abs",
        kind: BuiltinKind::Query,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "min",
        kind: BuiltinKind::Query,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "max",
        kind: BuiltinKind::Query,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "clamp",
        kind: BuiltinKind::Query,
        min_args: 3,
        max_args: 3,
    },
    Builtin {
        name: "intent",
        kind: BuiltinKind::Query,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "intent_text",
        kind: BuiltinKind::Query,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "train",
        kind: BuiltinKind::Action,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "build",
        kind: BuiltinKind::Action,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "attack",
        kind: BuiltinKind::Action,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "defend",
        kind: BuiltinKind::Action,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "move",
        kind: BuiltinKind::Action,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "harvest",
        kind: BuiltinKind::Action,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "scout",
        kind: BuiltinKind::Action,
        min_args: 2,
        max_args: 2,
    },
    Builtin {
        name: "retreat",
        kind: BuiltinKind::Action,
        min_args: 0,
        max_args: 0,
    },
    Builtin {
        name: "set_stance",
        kind: BuiltinKind::Action,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "say",
        kind: BuiltinKind::Action,
        min_args: 1,
        max_args: 1,
    },
    Builtin {
        name: "log",
        kind: BuiltinKind::Action,
        min_args: 1,
        max_args: 1,
    },
];

pub fn builtin(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|builtin| builtin.name == name)
}

/// The class names mentioned by train(...) / build(...) calls, for catalog
/// validation by the editor and launcher.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassReference {
    pub line: usize,
    pub column: usize,
    pub name: String,
}

/// Compile source text into a checked program.
pub fn compile(source: &str) -> Result<Program, Vec<Diagnostic>> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(vec![Diagnostic::new(
            1,
            1,
            format!(
                "script is {} bytes; the limit is {MAX_SOURCE_BYTES}",
                source.len()
            ),
        )]);
    }
    let tokens = lex(source).map_err(|error| vec![error])?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().map_err(|error| vec![error])?;
    let mut diagnostics = Vec::new();
    validate_program(&program, &mut diagnostics);
    if diagnostics.is_empty() {
        Ok(program)
    } else {
        Err(diagnostics)
    }
}

/// Collect class names passed literally to train / build.
pub fn class_references(program: &Program) -> Vec<ClassReference> {
    let mut references = Vec::new();
    for handler in &program.handlers {
        walk_block(&handler.body, &mut |expr| {
            if let ExprKind::Call(name, args) = &expr.kind {
                if (name == "train" || name == "build") && args.len() == 1 {
                    if let ExprKind::Str(class) = &args[0].kind {
                        references.push(ClassReference {
                            line: expr.line,
                            column: expr.column,
                            name: class.clone(),
                        });
                    }
                }
            }
        });
    }
    references
}

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    Ident(String),
    Int(i64),
    Str(String),
    Sym(&'static str),
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

fn lex(source: &str) -> Result<Vec<Token>, Diagnostic> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    while index < chars.len() {
        let ch = chars[index];
        if ch == '\n' {
            index += 1;
            line += 1;
            column = 1;
            continue;
        }
        if ch.is_whitespace() {
            index += 1;
            column += 1;
            continue;
        }
        if ch == '#' || (ch == '/' && chars.get(index + 1) == Some(&'/')) {
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
                column += 1;
            }
            continue;
        }
        if ch == '"' {
            let start_line = line;
            let start_column = column;
            index += 1;
            column += 1;
            let mut value = String::new();
            let mut closed = false;
            while index < chars.len() {
                let current = chars[index];
                if current == '"' {
                    index += 1;
                    column += 1;
                    closed = true;
                    break;
                }
                if current == '\n' {
                    return Err(Diagnostic::new(
                        start_line,
                        start_column,
                        "unterminated string literal",
                    ));
                }
                if current == '\\' {
                    let Some(escaped) = chars.get(index + 1).copied() else {
                        break;
                    };
                    let replacement = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\\' => '\\',
                        other => {
                            return Err(Diagnostic::new(
                                line,
                                column,
                                format!("unknown escape \\{other}"),
                            ))
                        }
                    };
                    value.push(replacement);
                    index += 2;
                    column += 2;
                    continue;
                }
                value.push(current);
                index += 1;
                column += 1;
            }
            if !closed {
                return Err(Diagnostic::new(
                    start_line,
                    start_column,
                    "unterminated string literal",
                ));
            }
            tokens.push(Token {
                kind: TokenKind::Str(value),
                line: start_line,
                column: start_column,
            });
            continue;
        }
        if ch.is_ascii_digit() {
            let start = index;
            let start_column = column;
            while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '_') {
                index += 1;
                column += 1;
            }
            let digits: String = chars[start..index]
                .iter()
                .filter(|character| **character != '_')
                .collect();
            let value = digits.parse::<i64>().map_err(|_| {
                Diagnostic::new(
                    line,
                    start_column,
                    format!("integer {digits} is out of range"),
                )
            })?;
            tokens.push(Token {
                kind: TokenKind::Int(value),
                line,
                column: start_column,
            });
            continue;
        }
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = index;
            let start_column = column;
            while index < chars.len()
                && (chars[index].is_ascii_alphanumeric() || chars[index] == '_')
            {
                index += 1;
                column += 1;
            }
            let ident: String = chars[start..index].iter().collect();
            tokens.push(Token {
                kind: TokenKind::Ident(ident),
                line,
                column: start_column,
            });
            continue;
        }
        let two: String = chars[index..(index + 2).min(chars.len())].iter().collect();
        if let Some(symbol) = match two.as_str() {
            "==" => Some("=="),
            "!=" => Some("!="),
            "<=" => Some("<="),
            ">=" => Some(">="),
            "&&" => Some("&&"),
            "||" => Some("||"),
            _ => None,
        } {
            tokens.push(Token {
                kind: TokenKind::Sym(symbol),
                line,
                column,
            });
            index += 2;
            column += 2;
            continue;
        }
        let symbol = match ch {
            '{' => "{",
            '}' => "}",
            '(' => "(",
            ')' => ")",
            ',' => ",",
            '=' => "=",
            '<' => "<",
            '>' => ">",
            '+' => "+",
            '-' => "-",
            '*' => "*",
            '/' => "/",
            '%' => "%",
            '!' => "!",
            ';' => ";",
            _ => {
                return Err(Diagnostic::new(
                    line,
                    column,
                    format!("unexpected character {ch}"),
                ))
            }
        };
        tokens.push(Token {
            kind: TokenKind::Sym(symbol),
            line,
            column,
        });
        index += 1;
        column += 1;
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        line,
        column,
    });
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser {
    tokens: Vec<Token>,
    position: usize,
    depth: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            depth: 0,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position.min(self.tokens.len() - 1)]
    }

    fn kind(&self) -> &TokenKind {
        &self.current().kind
    }

    fn nth_kind(&self, offset: usize) -> Option<&TokenKind> {
        self.tokens
            .get(self.position + offset)
            .map(|token| &token.kind)
    }

    fn location(&self) -> (usize, usize) {
        (self.current().line, self.current().column)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
        token
    }

    fn at_eof(&self) -> bool {
        matches!(self.kind(), TokenKind::Eof)
    }

    fn check_sym(&self, symbol: &str) -> bool {
        matches!(self.kind(), TokenKind::Sym(current) if *current == symbol)
    }

    fn eat_sym(&mut self, symbol: &str) -> bool {
        if self.check_sym(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_sym(&mut self, symbol: &str) -> Result<(), Diagnostic> {
        if self.eat_sym(symbol) {
            Ok(())
        } else {
            let (line, column) = self.location();
            Err(Diagnostic::new(
                line,
                column,
                format!("expected {symbol}, found {}", self.describe_current()),
            ))
        }
    }

    fn describe_current(&self) -> String {
        match self.kind() {
            TokenKind::Ident(name) => name.to_string(),
            TokenKind::Int(value) => value.to_string(),
            TokenKind::Str(_) => "a string".to_string(),
            TokenKind::Sym(symbol) => symbol.to_string(),
            TokenKind::Eof => "end of script".to_string(),
        }
    }

    fn check_ident(&self, name: &str) -> bool {
        matches!(self.kind(), TokenKind::Ident(current) if current == name)
    }

    fn eat_ident(&mut self, name: &str) -> bool {
        if self.check_ident(name) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_ident(&mut self, what: &str) -> Result<String, Diagnostic> {
        if let TokenKind::Ident(name) = self.kind() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            let (line, column) = self.location();
            Err(Diagnostic::new(
                line,
                column,
                format!("expected {what}, found {}", self.describe_current()),
            ))
        }
    }

    fn parse_program(&mut self) -> Result<Program, Diagnostic> {
        let mut handlers = Vec::new();
        while !self.at_eof() {
            let (line, _) = self.location();
            let kind = if self.eat_ident("on") {
                if self.eat_ident("start") {
                    HandlerKind::Start
                } else {
                    self.expect_ident("event")?;
                    let event = match self.kind() {
                        TokenKind::Ident(name) => {
                            let name = name.clone();
                            self.advance();
                            name
                        }
                        TokenKind::Str(name) => {
                            let name = name.clone();
                            self.advance();
                            name
                        }
                        _ => {
                            let (line, column) = self.location();
                            return Err(Diagnostic::new(
                                line,
                                column,
                                "expected an event name after on event",
                            ));
                        }
                    };
                    if event.is_empty() {
                        return Err(Diagnostic::new(line, 1, "event name may not be empty"));
                    }
                    HandlerKind::Event(event)
                }
            } else if self.eat_ident("every") {
                HandlerKind::Every(self.parse_expr()?)
            } else {
                return Err(Diagnostic::new(
                    line,
                    1,
                    format!(
                        "expected on start, on event or every, found {}",
                        self.describe_current()
                    ),
                ));
            };
            let body = self.parse_block()?;
            handlers.push(Handler { kind, body, line });
            if handlers.len() > MAX_HANDLERS {
                return Err(Diagnostic::new(
                    line,
                    1,
                    format!("an agent may declare at most {MAX_HANDLERS} handlers"),
                ));
            }
            self.eat_sym(";");
        }
        if handlers.is_empty() {
            return Err(Diagnostic::new(
                1,
                1,
                "an agent needs at least one on start, on event or every handler",
            ));
        }
        Ok(Program { handlers })
    }

    fn parse_block(&mut self) -> Result<Block, Diagnostic> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            let (line, column) = self.location();
            return Err(Diagnostic::new(
                line,
                column,
                format!("blocks may nest at most {MAX_DEPTH} deep"),
            ));
        }
        self.expect_sym("{")?;
        let mut body = Vec::new();
        while !self.check_sym("}") {
            if self.at_eof() {
                let (line, column) = self.location();
                return Err(Diagnostic::new(line, column, "unclosed block; expected }"));
            }
            if self.eat_sym(";") {
                continue;
            }
            body.push(self.parse_stmt()?);
        }
        self.expect_sym("}")?;
        self.depth -= 1;
        Ok(body)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, Diagnostic> {
        if self.eat_ident("let") {
            let name = self.expect_ident("a variable name")?;
            self.expect_sym("=")?;
            let value = self.parse_expr()?;
            self.eat_sym(";");
            return Ok(Stmt::Let { name, value });
        }
        if self.eat_ident("if") {
            return self.parse_if();
        }
        if self.eat_ident("while") {
            let condition = self.parse_expr()?;
            let body = self.parse_block()?;
            self.eat_sym(";");
            return Ok(Stmt::While { condition, body });
        }
        if let TokenKind::Ident(name) = self.kind() {
            if matches!(self.nth_kind(1), Some(TokenKind::Sym("="))) {
                let name = name.clone();
                self.advance();
                self.advance();
                let value = self.parse_expr()?;
                self.eat_sym(";");
                return Ok(Stmt::Assign { name, value });
            }
        }
        let expr = self.parse_expr()?;
        self.eat_sym(";");
        Ok(Stmt::Expr(expr))
    }

    fn parse_if(&mut self) -> Result<Stmt, Diagnostic> {
        let condition = self.parse_expr()?;
        let then_body = self.parse_block()?;
        let else_body = if self.eat_ident("else") {
            if self.check_ident("if") {
                self.advance();
                Some(vec![self.parse_if()?])
            } else {
                Some(self.parse_block()?)
            }
        } else {
            None
        };
        self.eat_sym(";");
        Ok(Stmt::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, Diagnostic> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_and()?;
        while self.eat_sym("||") || self.eat_ident("or") {
            let right = self.parse_and()?;
            left = binary(BinOp::Or, left, right);
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_equality()?;
        while self.eat_sym("&&") || self.eat_ident("and") {
            let right = self.parse_equality()?;
            left = binary(BinOp::And, left, right);
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_comparison()?;
        loop {
            let operator = if self.eat_sym("==") {
                BinOp::Eq
            } else if self.eat_sym("!=") {
                BinOp::Ne
            } else {
                break;
            };
            let right = self.parse_comparison()?;
            left = binary(operator, left, right);
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_additive()?;
        loop {
            let operator = if self.eat_sym("<=") {
                BinOp::Le
            } else if self.eat_sym(">=") {
                BinOp::Ge
            } else if self.eat_sym("<") {
                BinOp::Lt
            } else if self.eat_sym(">") {
                BinOp::Gt
            } else {
                break;
            };
            let right = self.parse_additive()?;
            left = binary(operator, left, right);
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let operator = if self.eat_sym("+") {
                BinOp::Add
            } else if self.eat_sym("-") {
                BinOp::Sub
            } else {
                break;
            };
            let right = self.parse_multiplicative()?;
            left = binary(operator, left, right);
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_unary()?;
        loop {
            let operator = if self.eat_sym("*") {
                BinOp::Mul
            } else if self.eat_sym("/") {
                BinOp::Div
            } else if self.eat_sym("%") {
                BinOp::Rem
            } else {
                break;
            };
            let right = self.parse_unary()?;
            left = binary(operator, left, right);
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, Diagnostic> {
        if self.eat_sym("!") || self.eat_ident("not") {
            let (line, column) = self.location();
            let operand = self.parse_unary()?;
            return Ok(Expr {
                kind: ExprKind::Unary(UnOp::Not, Box::new(operand)),
                line,
                column,
            });
        }
        if self.eat_sym("-") {
            let (line, column) = self.location();
            let operand = self.parse_unary()?;
            return Ok(Expr {
                kind: ExprKind::Unary(UnOp::Neg, Box::new(operand)),
                line,
                column,
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, Diagnostic> {
        let (line, column) = self.location();
        match self.kind().clone() {
            TokenKind::Int(value) => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Int(value),
                    line,
                    column,
                })
            }
            TokenKind::Str(value) => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Str(value),
                    line,
                    column,
                })
            }
            TokenKind::Ident(name) if name == "true" || name == "false" => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Bool(name == "true"),
                    line,
                    column,
                })
            }
            TokenKind::Ident(name) => {
                self.advance();
                if self.eat_sym("(") {
                    let mut args = Vec::new();
                    if !self.check_sym(")") {
                        loop {
                            args.push(self.parse_expr()?);
                            if !self.eat_sym(",") {
                                break;
                            }
                        }
                    }
                    self.expect_sym(")")?;
                    Ok(Expr {
                        kind: ExprKind::Call(name, args),
                        line,
                        column,
                    })
                } else {
                    Ok(Expr {
                        kind: ExprKind::Var(name),
                        line,
                        column,
                    })
                }
            }
            TokenKind::Sym("(") => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect_sym(")")?;
                Ok(expr)
            }
            _ => Err(Diagnostic::new(
                line,
                column,
                format!("expected an expression, found {}", self.describe_current()),
            )),
        }
    }
}

fn binary(operator: BinOp, left: Expr, right: Expr) -> Expr {
    Expr {
        line: left.line,
        column: left.column,
        kind: ExprKind::Binary(operator, Box::new(left), Box::new(right)),
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_program(program: &Program, diagnostics: &mut Vec<Diagnostic>) {
    for handler in &program.handlers {
        if let HandlerKind::Every(expr) = &handler.kind {
            validate_expr(expr, diagnostics);
        }
        validate_block(&handler.body, diagnostics);
    }
}

fn validate_block(block: &Block, diagnostics: &mut Vec<Diagnostic>) {
    for statement in block {
        match statement {
            Stmt::Let { value, .. } | Stmt::Assign { value, .. } => {
                validate_expr(value, diagnostics)
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                validate_expr(condition, diagnostics);
                validate_block(then_body, diagnostics);
                if let Some(body) = else_body {
                    validate_block(body, diagnostics);
                }
            }
            Stmt::While { condition, body } => {
                validate_expr(condition, diagnostics);
                validate_block(body, diagnostics);
            }
            Stmt::Expr(expr) => {
                if !matches!(expr.kind, ExprKind::Call(_, _)) {
                    diagnostics.push(
                        Diagnostic::new(
                            expr.line,
                            expr.column,
                            "this expression has no effect; call a command or assign it",
                        )
                        .with_hint("try let value = ..."),
                    );
                }
                validate_expr(expr, diagnostics);
            }
        }
    }
}

fn validate_expr(expr: &Expr, diagnostics: &mut Vec<Diagnostic>) {
    match &expr.kind {
        ExprKind::Call(name, args) => {
            match builtin(name) {
                Some(spec) => {
                    if args.len() < spec.min_args || args.len() > spec.max_args {
                        let expected = if spec.min_args == spec.max_args {
                            format!("{}", spec.min_args)
                        } else {
                            format!("{}..{}", spec.min_args, spec.max_args)
                        };
                        diagnostics.push(Diagnostic::new(
                            expr.line,
                            expr.column,
                            format!(
                                "{name} takes {expected} argument(s), but {} were given",
                                args.len()
                            ),
                        ));
                    }
                    if (name == "intent" || name == "intent_text")
                        && !args.is_empty()
                        && !matches!(args[0].kind, ExprKind::Str(_))
                    {
                        diagnostics.push(
                            Diagnostic::new(
                                args[0].line,
                                args[0].column,
                                "the intent key must be a string literal",
                            )
                            .with_hint("the host prefetches keys before the match starts"),
                        );
                    }
                }
                None => {
                    let mut names: Vec<&str> = BUILTINS.iter().map(|item| item.name).collect();
                    names.sort_unstable();
                    diagnostics.push(
                        Diagnostic::new(expr.line, expr.column, format!("unknown function {name}"))
                            .with_hint(format!("known functions: {}", names.join(", "))),
                    );
                }
            }
            for argument in args {
                validate_expr(argument, diagnostics);
            }
        }
        ExprKind::Unary(_, operand) => validate_expr(operand, diagnostics),
        ExprKind::Binary(_, left, right) => {
            validate_expr(left, diagnostics);
            validate_expr(right, diagnostics);
        }
        ExprKind::Int(_) | ExprKind::Str(_) | ExprKind::Bool(_) | ExprKind::Var(_) => {}
    }
}

fn walk_block(block: &Block, visit: &mut impl FnMut(&Expr)) {
    for statement in block {
        match statement {
            Stmt::Let { value, .. } | Stmt::Assign { value, .. } => walk_expr(value, visit),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                walk_expr(condition, visit);
                walk_block(then_body, visit);
                if let Some(body) = else_body {
                    walk_block(body, visit);
                }
            }
            Stmt::While { condition, body } => {
                walk_expr(condition, visit);
                walk_block(body, visit);
            }
            Stmt::Expr(expr) => walk_expr(expr, visit),
        }
    }
}

fn walk_expr(expr: &Expr, visit: &mut impl FnMut(&Expr)) {
    visit(expr);
    match &expr.kind {
        ExprKind::Call(_, args) => {
            for argument in args {
                walk_expr(argument, visit);
            }
        }
        ExprKind::Unary(_, operand) => walk_expr(operand, visit),
        ExprKind::Binary(_, left, right) => {
            walk_expr(left, visit);
            walk_expr(right, visit);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_handlers_and_precedence() {
        let program = compile(
            r#"
            on start { set_stance("defensive") }
            every 10 + 20 * 2 {
                if credits() < 100 && units("InfantryE1") == 0 {
                    train("InfantryE1")
                } else {
                    harvest()
                }
            }
            on event "unit_lost" { defend(1, 2) }
            "#,
        )
        .expect("script should compile");
        assert_eq!(program.handlers.len(), 3);
        match &program.handlers[1].kind {
            HandlerKind::Every(expr) => match &expr.kind {
                ExprKind::Binary(BinOp::Add, _, right) => match &right.kind {
                    ExprKind::Binary(BinOp::Mul, _, _) => {}
                    other => panic!("expected multiply on the right, got {other:?}"),
                },
                other => panic!("expected addition, got {other:?}"),
            },
            other => panic!("expected every handler, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_functions_and_bad_arity() {
        let errors = compile("on start { teleport(1) }").unwrap_err();
        assert!(errors[0].message.contains("unknown function teleport"));

        let errors = compile("on start { attack(1) }").unwrap_err();
        assert!(errors[0].message.contains("attack takes 2 argument(s)"));
    }

    #[test]
    fn reports_line_and_column() {
        let errors = compile("on start {\n  train(\n}").unwrap_err();
        assert_eq!(errors[0].line, 3);
    }

    #[test]
    fn comments_and_escapes() {
        let program = compile(
            "on start {\n  # a comment\n  log(\"hi \\\"there\\\"\") // trailing\n  set_stance(\"x\")\n}",
        )
        .expect("script should compile");
        assert_eq!(program.handlers.len(), 1);
    }

    #[test]
    fn class_references_are_collected() {
        let program = compile("on start { train(\"InfantryE1\") build(\"Refinery\") }").unwrap();
        let names: Vec<String> = class_references(&program)
            .into_iter()
            .map(|reference| reference.name)
            .collect();
        assert_eq!(names, ["InfantryE1", "Refinery"]);
    }

    #[test]
    fn stray_semicolons_are_ignored() {
        compile("on start { ; harvest(); ; }").expect("stray semicolons should compile");
    }

    #[test]
    fn rejects_effectless_expressions() {
        let errors = compile("on start { 1 + 2 }").unwrap_err();
        assert!(errors[0].message.contains("no effect"));
    }
}
