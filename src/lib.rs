//! `calcolo` is a CLI calculator that lets you write math expressions
//! using LaTeX-style notation and evaluate them from script files.

/// Type definition of the abstract syntax tree.
pub mod ast;

/// The process of evaluating an AST to obtain the calculation result.
pub mod eval;

/// Definition of tokens used in lexical analysis.
pub mod token;

/// Converts a string into a sequence of Tokens.
pub mod lexer;
