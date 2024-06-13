# Rust Nested Value

Nested structures stores the following values ​​inside:

- ConstValue - constant values
- MutValue - mutable values
- FetchValue - values fetched from API
- MultiValue - collection, nested structire

Any value present in the structure can be accessed by string path  
such as 'root/constants/const-1' or 'root/api/api-value-1'

**Example:**

```rust
    let value = ConstValue::new(Value::Null);
    println!("const value: {:?}", value.get(""));

    let value = ConstValue::new(12345.6789012345);
    println!("const value: {:?}", value.get(""));

    let mut flags = MultiValue::new([
        ("bool-flags", Box::new(MultiValue::new([
            ("flag-1", Box::new(ConstValue::new(true))),
            ("flag-2", Box::new(MutValue::new(false))),
        ]))),
    ]);
    let key = "bool-flags/flag-1";
    println!("flag: {:?}", flags.get(key));
    flags
        .store("example", key, true)
        .unwrap_or_else(|err| println!("example | Store error: {}", err));
    println!("flag: {:?}", flags.get(key));
```
