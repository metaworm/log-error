
# log-error

A small crate to log the error result easily

Sometimes we just want to record the error result, rather than return it to upper caller or unwrap the result with a panic, this crate will help you do it in relaxed

## Example

```rust
use log_error::*;
use std::io::Error;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    if let Some(_file) = std::fs::read("").log_warn("optional file") {
        // do something
    }

    // detailed error message
    do_something().log_error_detail("do_something");
}

fn do_something() -> Result<(), Error> {
    // ...
    Err(Error::last_os_error())
}
```