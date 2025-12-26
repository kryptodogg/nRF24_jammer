# Technology Stack: nRF24 Jammer

## Hardware Platform
*   **Microcontroller:** ESP32 (esp32dev)
*   **Display:** 128x32 or 128x64 I2C/SPI OLED Displays (SSD1306 driver)
*   **Radio Modules:** nRF24L01+PA+LNA
*   **Input:** Tactile Buttons

## Development Environment
*   **Build System & Manager:** PlatformIO Core
*   **Language:** C++ (C++11/17)
*   **Framework:** Arduino Core for ESP32

## Core Libraries
*   **RF Communication:** `RF24` (Modified version located in `lib/`) - Handles SPI communication with nRF24 modules.
*   **Display:** `Adafruit SSD1306` & `Adafruit GFX Library` - Graphics rendering for OLEDs.
*   **Input:** `GyverButton` - Handling button presses (debounce, clicks, holds).

## Communication Interfaces
*   **Serial:** USB-Serial for CLI control.
*   **WiFi:** ESP32 WiFi Station/AP mode for Web Interface.
*   **Web Server:** ESP32 built-in WebServer (implied for Web UI).
