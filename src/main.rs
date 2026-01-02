mod attacks;
mod hardware;

use attacks::{deauth, ble_spam, beacon_spam, mousejack};
use hardware::wifi::WifiInterface;
use hardware::ble::BleInterface;
use hardware::nrf24::Nrf24Interface;

fn main() {
    println!("nRF24 Jammer (Rust Port) - Attack Simulator");
    println!("===========================================");

    // Initialize hardware interfaces
    let wifi = WifiInterface::new();
    let ble = BleInterface::new();
    let nrf24 = Nrf24Interface::new();

    println!("\n[1] Running Deauthentication Attack");
    deauth::execute(&wifi);

    println!("\n[2] Running Beacon Spam");
    beacon_spam::execute(&wifi);

    println!("\n[3] Running BLE Spam");
    ble_spam::execute(&ble);

    println!("\n[4] Running nRF24 Mousejack");
    mousejack::execute(&nrf24);

    println!("\nAll simulated attacks completed.");
}
