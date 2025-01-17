# Architecture of `ergc`

## 1. Scan an Erg script (.er) and generate a `TokenStream`

src: [erg_parser\lex.rs](../../../crates/erg_parser/lex.rs)

* parser/lexer/Lexer generates `TokenStream` (this is an iterator of `Token`, `TokenStream` can be generated by `Lexer::collect()`)
  * `Lexer` is constructed from `Lexer::new` or `Lexer::from_str`, where `Lexer::new` reads the code from a file or command option.
  * `Lexer` can generate tokens sequentially as an iterator; if you want to get a `TokenStream` all at once, use `Lexer::lex`.
  * `Lexer` outputs `LexError`s as errors, but `LexError` does not have enough information to display itself. If you want to display the error, use the `LexerRunner` to convert the error.
  * `LexerRunner` can also be used if you want to use `Lexer` as standalone; `Lexer` is just an iterator and does not implement the `Runnable` trait.
    * `Runnable` is implemented by `LexerRunner`, `ParserRunner`, `Compiler`, and `DummyVM`.

## 2. Convert `TokenStream` -> `AST`

src: [erg_parser\parse.rs](../../../crates/erg_parser/parse.rs)

* `Parser`, like `Lexer`, has two constructors, `Parser::new` and `Parser::from_str`, and `Parser::parse` will give the `AST`.
* `AST` is the wrapper type of `Vec<Expr>`. It is for "Abstract Syntax Tree".

### 2.1 Desugaring `AST`

src: [erg_parser/desugar.rs](../../../crates/erg_parser/desugar.rs)

* expand nested vars (`Desugarer::desugar_nest_vars_pattern`)
* desugar multiple pattern definition syntax (`Desugarer::desugar_multiple_pattern_def`)

### 2.2 Reordering & Linking `AST`

src: [erg_compiler/reorder.rs](../../../crates/erg_compiler/reorder.rs)

* link class methods to class definitions
  * method definitions are allowed outside of the class definition file
  * current implementation is incomplete, only in the same file

## 3. Convert `AST` -> `HIR`

(main) src: [erg_compiler/lower.rs](../../../crates/erg_compiler/lower.rs)

## 3.1 Name Resolution

In the current implementation it is done during type checking.

* All ASTs (including imported modules) are scanned for name resolution before type inference.
* In addition to performing constant cycle checking and reordering, a context is created for type inference (however, most of the information on variables registered in this context is not yet finalized).

### 3.2 Type checking & inference

src: [erg_compiler/lower.rs](../../../crates/erg_compiler/lower.rs)

* `HIR` has every variable's type information. It is for "High-level Intermediate Representation".
* `ASTLowerer` can be constructed in the same way as `Parser` and `Lexer`.
* `ASTLowerer::lower` will output a tuple of `HIR` and `CompileWarnings` if no errors occur.
* `ASTLowerer` is owned by `Compiler`. Unlike conventional structures, `ASTLowerer` handles code contexts and is not a one-time disposable.
* If the result of type inference is incomplete (if there is an unknown type variable), an error will occur during name resolution.

## 4. Check side-effects

src: [erg_compiler/effectcheck.rs](../../../crates/erg_compiler/effectcheck.rs)

## 5. Check ownerships

src: [erg_compiler/ownercheck.rs](../../../crates/erg_compiler/ownercheck.rs)

## 6. Desugar `HIR`

src: [erg_compiler/desugar_hir.rs](../../../crates/erg_compiler/desugar_hir.rs)

Convert parts that are not consistent with Python syntax

* Convert class member variables to functions

## 7. Link

src: [erg_compiler/link.rs](../../../crates/erg_compiler/link.rs)

* Load all modules, resolve dependencies, and combine into a single HIR

## 8. Generate Bytecode (`CodeObj`) from `HIR`

src: [erg_compiler/codegen.rs](../../../crates/erg_compiler/codegen.rs)
