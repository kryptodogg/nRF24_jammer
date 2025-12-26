# Product Guidelines: nRF24 Jammer

## Documentation Style
*   **Concise & Direct:** Documentation must prioritize clarity and efficiency. Use bullet points, short sentences, and structured headers to allow users to quickly find and digest information.
*   **Technical Precision:** Avoid ambiguous language. When describing hardware interactions or protocol disruption, use exact terminology (e.g., "shared SPI bus", "GPIO", "frequency hopping").

## Interface Conventions
*   **Functional & Explicit Naming:** All user-facing commands and menu items must be descriptive and unambiguous. Commands like `START_JAMMING`, `STOP_ATTACK`, and `CONFIG_PINS` are preferred over vague alternatives.
*   **Consistency Across Platforms:** Command names and logical flows should be as consistent as possible between the Serial Interface, Web Interface, and OLED menu.

## Feedback and Error Handling
*   **Informative & Actionable Feedback:** When an operation fails or a configuration is invalid, the system must provide a clear, human-readable error message.
*   **Troubleshooting Support:** Error messages should, where possible, suggest corrective actions or refer the user to the relevant section of the documentation (e.g., "Error: SPI communication failed. Check CE/CSN pin configuration in settings.").
