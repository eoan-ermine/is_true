This crate entroduces `is_true!` macro and `is_true()` function which checks if argument is true

```toml
[dependencies] 
is_true = "0.1"
```

## Usage

```rust
use is_true::is_true;

fn main() {
    let result = is_true!(4 % 2 == 0, 5 % 2 == 0);
    assert_eq!(result, false);        
}
```

```rust
use is_true::is_true;

fn main() {
    assert_eq!(is_true!(true), true);
}
```