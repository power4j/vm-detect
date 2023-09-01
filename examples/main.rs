use vm_detect::{vm_detect, Detection};

fn main() {
    // Run detection
    let detection = vm_detect();

    // Inspect detections
    if detection.contains(Detection::HYPERVISOR_BIT) {
        println!("Hypervisor bit set!");
    }
}
