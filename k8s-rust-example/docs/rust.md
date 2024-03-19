# Rust
Learning the basics of Rust Lang <br />

⚙️ **Package Manager:** Cargo <br />
Packages are called as **crates**

❗ **Rust is a compiled language that use statically typed variables**

### 1 - Import locally files
```rust
mod file_to_include;
```
#### Functions:
**name:** snake_case
```rust
// public
pub fn function_name() {

}

// private
fn function_name() {

}
```

#### Variables:
Variables are **immutable** by default but to change it you can use keyword **mut**
```rs
let mut v = "hi";
```

##### Ownership
Set of rules that manages memory rust program.

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

##### Tips:
- **prelude**: The prelude is the list of things that Rust automatically imports into every Rust program.