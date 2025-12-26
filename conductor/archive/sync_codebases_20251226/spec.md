# Specification: Sync Codebases and Migrate OLED to SPI

## Goal
To synchronize the current `src/` directory with the reference implementation in `nRF24_jammer_source/src/128x64_Flexible`, ensuring that the only significant deviation is the configuration of the OLED display to use SPI instead of I2C.

## Context
The project is currently in a state where the active source code in `src/` needs to be aligned with a specific reference version (`128x64_Flexible`). This alignment is a prerequisite for future development. The target hardware uses an SPI-connected OLED, whereas the flexible reference might default to or support I2C.

## Requirements
1.  **Code Parity:** The logic and structure of `src/*.cpp` and `include/*.h` must match `nRF24_jammer_source/src/128x64_Flexible` and its corresponding headers.
2.  **OLED Interface Migration:**
    *   Identify the I2C initialization code for the SSD1306 display in the reference code.
    *   Replace it with the appropriate SPI initialization code in the active `src/` code.
    *   Ensure pin definitions for SPI (MOSI, CLK, DC, CS, RES) are correctly defined and used.
3.  **Verification:**
    *   The code must compile without errors.
    *   A manual diff (or automated comparison) should show that files are identical except for the OLED configuration lines.

## Technical Details
*   **Reference Source:** `nRF24_jammer_source/src/128x64_Flexible/*.cpp`
*   **Target Source:** `src/*.cpp`
*   **Key Files:** `nRF24_jammer.cpp`, `jam.cpp`, `options.cpp`, `bitmap.cpp`, `html.cpp`, `serial.cpp`
*   **OLED Library:** `Adafruit_SSD1306`
*   **Expected Change:**
    *   *From:* `Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);` (or similar I2C constructor)
    *   *To:* `Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &SPI, OLED_DC, OLED_RESET, OLED_CS);` (or software SPI constructor if hardware SPI is shared/unavailable, though hardware SPI is preferred).

## Out of Scope
*   Adding new features.
*   Refactoring code style beyond what is necessary for the sync.
*   Changing pin configurations for the nRF24 modules (unless they conflict with the OLED SPI pins).
