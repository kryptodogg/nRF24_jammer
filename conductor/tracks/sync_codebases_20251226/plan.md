# Plan: Sync Codebases and Migrate OLED to SPI

## Phase 1: Analysis and Preparation
- [x] Task: Analyze Differences 3ae97c7
    - compare `src/` and `nRF24_jammer_source/src/128x64_Flexible` to identify discrepancies.
    - Identify the specific lines responsible for OLED initialization in the reference code.
- [ ] Task: Conductor - User Manual Verification 'Analysis and Preparation' (Protocol in workflow.md)

## Phase 2: Implementation
- [ ] Task: Synchronize `nRF24_jammer.cpp`
    - Copy content from reference, adjusting OLED setup to SPI.
- [ ] Task: Synchronize `jam.cpp`
    - Ensure logical parity with reference.
- [ ] Task: Synchronize `options.cpp`
    - Ensure logical parity with reference.
- [ ] Task: Synchronize Helper Files (`bitmap.cpp`, `html.cpp`, `serial.cpp`)
    - Ensure logical parity with reference.
- [ ] Task: Update Headers
    - Ensure `include/` headers match the requirements of the updated source files.
- [ ] Task: Conductor - User Manual Verification 'Implementation' (Protocol in workflow.md)

## Phase 3: Verification
- [ ] Task: Compilation Check
    - Run `pio run` to ensure the project compiles successfully.
- [ ] Task: Final Diff Check
    - Verify that only OLED-related changes exist compared to the reference.
- [ ] Task: Conductor - User Manual Verification 'Verification' (Protocol in workflow.md)
