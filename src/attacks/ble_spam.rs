use crate::hardware::ble::BleInterface;
use rand::Rng;

pub fn execute(ble: &BleInterface) {
    println!("Starting BLE spam...");

    // Simulate sending advertisements for different "devices"
    let device_names = vec![
        "iPhone 15",
        "Samsung Galaxy S24",
        "AirPods Pro",
        "Google Coral",
        "Smart Toaster",
    ];

    for name in device_names {
        println!("Advertising as: {}", name);
        let packet = create_ble_adv_packet(name);
        ble.send_advertisement(&packet);
    }
}

fn create_ble_adv_packet(name: &str) -> Vec<u8> {
    // Bluetooth Low Energy Advertisement Packet Structure
    // Preamble (1 byte)
    // Access Address (4 bytes) - 0x8E89BED6 for Advertisements
    // PDU Header (2 bytes)
    // Payload (Mac + Data)
    // CRC (3 bytes)

    // Note: The hardware interface typically handles Preamble, Access Address, and CRC.
    // We usually provide the PDU (Header + Payload).

    let mut pdu = Vec::new();
    let mut rng = rand::thread_rng();

    // PDU Header
    // Type: ADV_IND (0x00) or ADV_NONCONN_IND (0x02)
    // Let's use ADV_IND (Connectable Undirected Advertising)
    let pdu_type = 0x00;
    pdu.push(pdu_type);

    // Length (will be filled later)
    pdu.push(0x00);

    // Payload:
    // AdvA (Advertiser Address) - 6 bytes random
    let mut adv_a = [0u8; 6];
    rng.fill(&mut adv_a);
    pdu.extend_from_slice(&adv_a);

    // AdvData
    // AD Structure: Length, Type, Data

    // Flags
    pdu.push(0x02); // Length
    pdu.push(0x01); // Type: Flags
    pdu.push(0x06); // LE General Discoverable Mode, BR/EDR Not Supported

    // Complete Local Name
    let name_bytes = name.as_bytes();
    if name_bytes.len() < 20 { // Ensure it fits
        pdu.push((name_bytes.len() + 1) as u8); // Length
        pdu.push(0x09); // Type: Complete Local Name
        pdu.extend_from_slice(name_bytes);
    }

    // Update Length in PDU Header
    // Length = Payload size (AdvA + AdvData)
    // AdvA is 6 bytes. AdvData starts at index 8 of pdu vec.
    let payload_len = pdu.len() - 2;
    pdu[1] = payload_len as u8;

    pdu
}
