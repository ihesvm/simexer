# Lexer and Toknizer
### A simple tokenizer/lexer, written in Rust

### Usage
```bash
>>> tknize sample.text
[
    Token {
        token_type: Let,
        value: "let",
    },
    Token {
        token_type: Identifier,
        value: "foo",
    },
    Token {
        token_type: Assign,
        value: "=",
    },
    Token {
        token_type: Word,
        value: "bar",
    },
]
```
and 
```rust
let text = "10 + 10"; 
let res = toknizer(text).unwrap();

Output
[
    Number(
        10,
    ),
    Plus,
    Number(
        10,
    ),
    EOF,
]
```



---

# todo
- [ ] Parsing Integer in lexer