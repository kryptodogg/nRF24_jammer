pub struct Nrf24Interface;

impl Nrf24Interface {
    pub fn new() -> Self {
        Nrf24Interface
    }

    pub fn set_channel(&self, channel: u8) {
        println!("Setting nRF24 channel to {}", channel);
    }

    pub fn transmit(&self, data: &[u8]) {
        println!("Transmitting on nRF24: {:?}", data);
    }
}
