pub struct WifiInterface;

impl WifiInterface {
    pub fn new() -> Self {
        WifiInterface
    }

    pub fn _set_channel(&self, channel: u8) {
        println!("Setting WiFi channel to {}", channel);
    }

    pub fn inject_frame(&self, frame: &[u8]) {
        println!("Injecting WiFi frame: {:?}", frame);
    }
}
