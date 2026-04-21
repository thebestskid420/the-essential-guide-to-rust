# Rusty — Rust learning workspace

This folder is a personal curriculum: notes in `practice/` and `projects/`, and small runnable sketches under `src/bin/`.

## Prerequisites

- Install Rust from [rustup.rs](https://rustup.rs/).
- On Windows with the default MSVC toolchain, you need the **Visual C++ Build Tools** (or full Visual Studio with the C++ workload) so `link.exe` is available. If `cargo build` complains about `link.exe`, install the build tools or switch to the GNU toolchain via `rustup`.

## How to run a topic

From this directory:

```bash
cargo run --bin t03_variables
```

List all topic binaries:

```bash
cargo build --bins
```

## Curriculum checklist

Use this as your syllabus; tick boxes as you go.

### 1. Rust By Practice

- [ ] Work through exercises in [`practice/`](practice/README.md) (links + log)

### 2. Small projects with elegant code

- [ ] Build mini programs in [`projects/`](projects/README.md)

### 3. Variables

- [ ] `t03_variables` — `let`, mutability, shadowing, constants

### 4. Basic Types

- [ ] **4.1** Numbers — integers, floats, wrapping ops
- [ ] **4.2** Char, bool, unit `()`
- [ ] **4.3** Statements vs expressions
- [ ] **4.4** Functions — params, return, diverging

*Binary: `t04_basic_types`*

### 5. Ownership and Borrowing

- [ ] **5.1** Ownership — move, copy, drop order
- [ ] **5.2** References and borrowing — `&T`, `&mut T`, rules

*Binary: `t05_ownership`*

### 6. Compound Types

- [ ] **6.1** String types — `&str`, `String`
- [ ] **6.2** Array
- [ ] **6.3** Slice
- [ ] **6.4** Tuple
- [ ] **6.5** Struct
- [ ] **6.6** Enum

*Binary: `t06_compound_types`*

### 7. Flow Control

- [ ] `if`, `loop`, `while`, `for`, `break` / `continue` with labels

*Binary: `t07_flow_control`*

### 8. Pattern Matching

- [ ] **8.1** `match`, `matches!`, `if let`, `while let`
- [ ] **8.2** Patterns — refutability, guards, `@` bindings

*Binary: `t08_pattern_match`*

### 9. Methods and Associated Functions

- [ ] `impl`, `Self`, associated functions vs methods

*Binary: `t09_methods`*

### 10. Generics and Traits

- [ ] **10.1** Generics on types and functions
- [ ] **10.2** Const generics
- [ ] **10.3** Traits — bounds, `impl Trait`, trait bounds on `impl`
- [ ] **10.4** Trait objects — `dyn Trait`
- [ ] **10.5** Advanced traits — default methods, associated types, GATs (as needed)

*Binary: `t10_generics_traits`*

### 11. Collection Types

- [ ] **11.1** `String`
- [ ] **11.2** `Vec`
- [ ] **11.3** `HashMap` / `HashSet`

*Binary: `t11_collections`*

### 12. Type Conversion

- [ ] **12.1** `as` casts
- [ ] **12.2** `From` / `Into`
- [ ] **12.3** Other conversions — `TryFrom`, parsing, etc.

*Binary: `t12_type_conversion`*

### 13. `Result` and panic

- [ ] **13.1** `panic!` and when not to
- [ ] **13.2** `Result`, `?`, error propagation

*Binary: `t13_result_panic`*

### 14. Crates and Modules

- [ ] **14.1** Package vs crate (library vs binary)
- [ ] **14.2** Modules — `mod`, file layout
- [ ] **14.3** `use`, `pub`, re-exports, prelude

*Binary: `t14_modules`* (plus this repo’s real `src/` layout)

### 15. Comments and Docs

- [ ] Line/block comments, doc comments `///`, `//!`, `rustdoc`

*Binary: `t15_comments_docs`*

### 16. Formatted Output

- [ ] **16.1** `println!`, `format!`, `print!`, `eprintln!`
- [ ] **16.2** `Debug` (`{:?}`) and `Display`
- [ ] **16.3** Width, alignment, precision, custom formatters

*Binary: `t16_formatted_output`*

### 17. Lifetimes

- [ ] **17.1** Basics — elision, explicit lifetimes on references
- [ ] **17.2** `'static` and `T: 'static`
- [ ] **17.3** Advanced — higher-ranked, outlives relations

*Binary: `t17_lifetime`*

### 18. Functional Style

- [ ] **18.1** Closures — captures, `Fn` traits, move closures
- [ ] **18.2** Iterators — `iter`, `into_iter`, adapters, consumers

*Binary: `t18_functional`*

### 19. Newtype and DST

- [ ] Newtype pattern, `type` aliases, DST basics (`str`, `[T]`, trait objects)

*Binary: `t19_newtype_dst`*

### 20. Smart pointers

- [ ] **20.1** `Box<T>`
- [ ] **20.2** `Deref` coercion
- [ ] **20.3** `Drop`
- [ ] **20.4** `Rc` / `Arc`
- [ ] **20.5** `Cell` / `RefCell`

*Binary: `t20_smart_pointers`*

### 21. Weak and circular references (TODO)

*Binary: `t21_weak_cycles`*

### 22. Self-referential structs (TODO)

*Binary: `t22_self_referential`*

### 23. Threads (TODO)

- [ ] **23.1** Spawning threads
- [ ] **23.2** Message passing — channels
- [ ] **23.3** Shared state — `Mutex`, `RwLock`
- [ ] **23.4** Atomics
- [ ] **23.5** `Send` and `Sync`

*Binary: `t23_threads`*

### 24. Global variables (TODO)

*Binary: `t24_global_variables`*

### 25. Errors in depth (TODO)

*Binary: `t25_errors`*

### 26. Unsafe Rust

- [ ] **26.1** Raw pointers, `unsafe` blocks, `unsafe fn`
- [ ] **26.2** Inline assembly (`asm!`) when you need it

*Binary: `t26_unsafe`*

### 27. Macros (TODO)

*Binary: `t27_macros`*

### 28. Tests (TODO)

- [ ] **28.1** Writing tests
- [ ] **28.2** Benchmarks (`criterion` or `bench` Nightly)
- [ ] **28.3** Unit vs integration tests
- [ ] **28.4** Assertions and custom messages

*See `tests/` and binary `t28_tests`*

### 29. Async / await (TODO)

- [ ] **29.1** `async` / `.await`
- [ ] **29.2** `Future` and executors
- [ ] **29.3** `Pin` / `Unpin`
- [ ] **29.4** `Stream`

*Binary: `t29_async`* (add `tokio` or similar when you start)

### 30. Standard library tour (TODO)

- [ ] **30.1** `String` and text APIs in depth

*Binary: `t30_stdlib`*

### 31. Fighting the compiler

- [ ] **31.1** Borrow checker errors — reading diagnostics, `NLL`, common fixes

*Binary: `t31_fighting_compiler`*

## Suggested external resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) (great alongside “Rust by Practice”)
- [Rust By Practice](https://practice.course.rs/) (if you meant this site, link it in `practice/README.md`)
