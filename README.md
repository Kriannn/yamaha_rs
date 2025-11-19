# Yamaha Extended Control API

## Auto-Discovery of Yamaha Devices

```rust
use yamaha_rs::discover_yamaha_devices;

fn main() {
    // Automatically find all Yamaha devices on the network
    let devices = discover_yamaha_devices();

    // Display the IP and name of each device
    for d in devices {
        println!("{} {}", d.ip, d.name);
    }
}
```
