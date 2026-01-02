use crate::hardware::wifi::WifiInterface;
use rand::Rng;

pub fn execute(wifi: &WifiInterface) {
    println!("Starting Beacon spam...");

    // Send 10 random beacons
    for _ in 0..10 {
        let ssid = generate_random_ssid();
        let frame = create_beacon_frame(&ssid);

        println!("Broadcasting Beacon for SSID: {}", ssid);
        wifi.inject_frame(&frame);
    }
}

fn generate_random_ssid() -> String {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(5..15);
    (0..len).map(|_| rng.gen_range(b'a'..=b'z') as char).collect()
}

fn create_beacon_frame(ssid: &str) -> Vec<u8> {
    let mut frame = Vec::new();
    let mut rng = rand::thread_rng();

    // Mac Address
    let mut mac = [0u8; 6];
    rng.fill(&mut mac);

    // Frame Control: 0x80 (Beacon)
    frame.push(0x80);
    frame.push(0x00);

    // Duration
    frame.push(0x00);
    frame.push(0x00);

    // Destination: Broadcast
    frame.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    // Source: Random MAC
    frame.extend_from_slice(&mac);

    // BSSID: Random MAC
    frame.extend_from_slice(&mac);

    // Sequence Control
    frame.push(0x00);
    frame.push(0x00);

    // Fixed Parameters
    // Timestamp (8 bytes)
    for _ in 0..8 { frame.push(0x00); }
    // Beacon Interval (2 bytes)
    frame.push(0x64); // 100 TU
    frame.push(0x00);
    // Capability Info (2 bytes)
    frame.push(0x01); // ESS
    frame.push(0x00);

    // Tagged Parameters

    // SSID (Tag 0)
    frame.push(0x00);
    frame.push(ssid.len() as u8);
    frame.extend_from_slice(ssid.as_bytes());

    // Supported Rates (Tag 1)
    frame.push(0x01);
    frame.push(0x08);
    frame.extend_from_slice(&[0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24]);

    // DS Parameter Set (Tag 3) - Channel
    frame.push(0x03);
    frame.push(0x01);
    frame.push(rng.gen_range(1..=14)); // Random channel

    frame
}
