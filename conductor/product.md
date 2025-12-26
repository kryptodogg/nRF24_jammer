# Product Guide: nRF24 Jammer

## Initial Concept
This project is an advanced, ESP32-based RF interference tool designed for security investigation and defense. It leverages nRF24L01+ modules to disrupt various wireless communication protocols, providing a platform for testing resilience and gathering evidence against malicious actors.

## Target Audience
*   **Security Researchers & Investigators:** Professionals needing to test wireless protocol resilience and document interference for defensive purposes.
*   **Educational Institutions:** Instructors demonstrating the principles of RF interference and signal security.
*   **Defense & Protection:** Users seeking to defend specific areas or devices from malicious wireless activity (e.g., unauthorized drones or surveillance).

## Core Goals
*   **Defense & Investigation:** To serve as a defensive tool against malicious wireless actors and a means to collect evidence for investigation.
*   **Resilience Testing:** To evaluate the robustness of wireless systems (Bluetooth, BLE, Wi-Fi, Zigbee) against interference.
*   **Portability & Accessibility:** To provide a field-ready, battery-operated device with intuitive controls for on-site operations.

## Key Features
*   **Multi-Protocol Disruption:** Capable of disrupting Bluetooth, BLE, Wi-Fi, Zigbee, and drone control signals.
*   **Flexible Architecture:** Supports scaling from a single module up to 30 nRF24 modules using a shared SPI bus (Flexible version) for increased power and range.
*   **Multi-Modal Control:** Features a responsive Web Interface, a Serial Command Line Interface, and on-device controls via OLED display and buttons.
*   **Intelligent Jamming Roadmap:** Future iterations will focus on smarter, adaptive jamming algorithms to target specific threats more effectively.
