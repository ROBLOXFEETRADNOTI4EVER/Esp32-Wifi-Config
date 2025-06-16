# 🦀 ESP32 WiFi Configuration System

An advanced ESP32 Rust-based WiFi configuration system that automatically switches between Access Point mode for initial setup and Client mode for normal operation.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![ESP32](https://img.shields.io/badge/ESP32-E7352C.svg?style=for-the-badge&logo=espressif&logoColor=white)
![Embassy](https://img.shields.io/badge/Embassy-async-blue?style=for-the-badge)

## ✨ Features

- 🔄 **Dual Mode Operation**: Seamless switching between AP and Client modes
- 🌐 **Web-Based Configuration**: Intuitive HTML interface for WiFi setup
- 💾 **Persistent Storage**: Credentials securely stored in ESP32 flash memory
- 🛡️ **Memory Safety**: Built with Rust for zero-cost abstractions
- ⚡ **Embassy Async**: Modern async/await architecture
- 📱 **Mobile Responsive**: Works perfectly on all devices
- 🔌 **Auto-Reconnection**: Handles network drops gracefully
- 🚀 **Blazingly Fast**: Optimized performance with Rust

## 🎯 How It Works

### 🔧 Initial Setup Mode
1. ESP32 boots without stored credentials
2. Automatically starts in **Access Point mode**
3. Creates WiFi hotspot: `ESP32-Config`
4. Serves configuration page at `192.168.13.37`
5. User enters WiFi credentials via web interface
6. Credentials saved to flash and device reboots

### 🌐 Normal Operation Mode
1. ESP32 reads stored credentials from flash
2. Connects to configured WiFi network as client
3. Serves main application web interface
4. Monitors connection health and reconnects automatically

## 🛠️ Hardware Requirements

| Component | Requirement |
|-----------|-------------|
| **Microcontroller** | ESP32 (any variant with WiFi) |
| **Flash Memory** | Minimum 4MB |
| **Power Supply** | 3.3V, 500mA minimum |
| **Connection** | USB or UART for programming |

## 📦 Dependencies

[dependencies]
esp-hal = { version = "1.0.0-beta", features = ["esp32", "defmt", "unstable"] }
embassy-executor = { version = "0.7.0", features = ["defmt", "task-arena-size-65536"] }
embassy-net = { version = "0.6.0", features = ["dhcpv4", "medium-ethernet", "tcp", "udp"] }
embassy-time = { version = "0.4.0", features = ["generic-queue-8"] }
esp-wifi = { version = "0.13.0", features = ["builtin-scheduler", "defmt", "esp-alloc", "esp32", "wifi"] }
esp-storage = { version = "0.4.0", features = ["esp32"] }
picoserve = { version = "0.15.0", features = ["embassy"] }
heapless = { version = "0.8.0", default-features = false }
serde = { version = "1.0.217", default-features = false, features = ["derive"] }
static_cell = { version = "2.1.0", features = ["nightly"] }
defmt = "0.3.10"
anyhow = { version = "1.0.95", default-features = false }
embedded-storage = "0.3.0"
critical-section = "1.2.0"

## 🚀 Quick Start

### Prerequisites
https://esp32.implrust.com/wifi/index.html 
