# Variables
     Default variable in Rust is immutable

# Shadowing

```rust
fn main() {
     let outer = 10;
     {
          let inner = 200;
          println!("inner = {}", inner);
          let outer = 300;
          println!("outer = {}", outer);
     }
     println!("outter - {}", outer);
}
```

# Data types
## 1. Scalar Data
- Interger
```rust

```
- String
- Boolean
- Float
## 2. Compound Data
- Tuple
```rust
     fn main() {
          let tup: (&str, i32) = ("hello", 100_000);
     }
```

