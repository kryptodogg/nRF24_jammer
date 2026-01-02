pub struct BleInterface;

impl BleInterface {
    pub fn new() -> Self {
        BleInterface
    }

    pub fn send_advertisement(&self, data: &[u8]) {
        println!("Sending BLE advertisement: {:?}", data);
    }
}
