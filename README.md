## How to use
```bash
quick-replace <target> <replacement> <input_file> <output_file>
```

## Example

Before replacement:
```
Hello, World!
```

run program: 
```bash
cargo run -- "World" "Rust" test.txt output.txt
```

and result:
```
Hello, Rust!
```
