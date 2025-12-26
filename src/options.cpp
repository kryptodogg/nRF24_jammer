#include "options.h"

//  Changeable parameters
GButton butt1(25);
GButton buttNext(26);
GButton buttPrevious(27);
const char *default_ssid = "jammer";
const char *default_password = "W0rthlessS0ul";
String Version_Number = "V2.8.0";
String Version_Name = "Flexible";

//  Unchangeable parameters
uint8_t SCREEN_WIDTH = 128;
uint8_t SCREEN_HEIGHT = 64;
int8_t OLED_RESET_VAL = 27; // Renamed to avoid conflict if any, but used in constructor
size_t EEPROM_SIZE = 512;

// OLED display pins for SPI
#define OLED_MOSI 13
#define OLED_CLK  14
#define OLED_DC   26
#define OLED_CS   25
#define OLED_RESET 27

// Initialize SSD1306 display with SPI
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, 
                         OLED_MOSI, OLED_CLK, OLED_DC, OLED_RESET, OLED_CS);

SPIClass *hp = nullptr;
int menu_number = 0;
int bluetooth_jam_method;
int drone_jam_method;
int display_setting;
int wifi_jam_method;
int zigbee_jam_method;
int Separate_or_together;
int misc_jam_method;
int logo;
int access_point;
int buttons;
int channel1 = 0;
int channel2 = 0;
int flag = 0;
bool hspi = false;
byte bluetooth_channels[] = {32, 34, 46, 48, 50, 52, 0,  1,  2,  4, 6,
                             8,  22, 24, 26, 28, 30, 74, 76, 78, 80};
byte ble_channels[] = {2, 26, 80};
const char jam_text[] = "xxxxxxxxxxxxxxxx";
RF24 *radios[30];
int ce_pins[30];
int csn_pins[30];
int nrf24_count;
WebServer server(80);
String logotype = "\n"
"  _  _  ____   ____  ___    _   _                                            \n"
" | \\| ||  _ \ |  __||__ \  | | | |                                           \n"
" |  ' || |_) || |__    ) | | | | | __ _  _ __ ___   _ __ ___    ___  _ __    \n"
" | .  ||  _ < |  __|  / /  | | | |/ _` || '_ ` _ \ | '_ ` _ \  / _ \| '__|   \n"
" | |\ || | \ \| |    / /_  | |_| | (_| || | | | | || | | | | ||  __/| |      \n"
" |_| \_||_|  \_\_|   |____|  \___/ \__,_||_| |_| |_||_| |_| |_| \___||_|      \n";
