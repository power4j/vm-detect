# Basic VM detection for x86 and x86_64

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
