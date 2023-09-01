# Basic VM detection for x86 and x86_64

[![Latest version](https://img.shields.io/crates/v/vm-detect.svg)](https://crates.io/crates/vm-detect)
[![Documentation](https://docs.rs/vm-detect/badge.svg)](https://docs.rs/vm-detect)
![License](https://img.shields.io/crates/l/vm-detect.svg)

Usage:

```rust
use vm_detect::{vm_detect, Detection};

fn main() {
    // Run detection
    let detection = vm_detect();

    // Inspect detections
    if detection.contains(Detection::HYPERVISOR_BIT) {
        println!("Hypervisor bit set!");
    }
}
```
