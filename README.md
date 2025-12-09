# The Forge Macro (TFM)
**TFM** is a lightweight, high-performance automation tool for the "The Forge" game mode in Roblox. Built entirely in **Rust**, it focuses on speed, safety, and low resource usage.
*(Make sure to upload your screenshot to the repo and name it strictly like this, or change this link)*

<div align="center">
  # The Forge Macro
    
  <img src="https://github.com/user-attachments/assets/84036e84-3961-447d-94ad-0cd4a4c385e3" alt="App Screenshot" />
</div>

## 🚀 Features
* **⛏️ Mining Clicker** – Automated mining to save your mouse and time.
* **🍀 Luck Potion** – Automatically manages and consumes luck potions.
* **💰 Auto Sell** – Prevents inventory overflow by selling resources automatically.
## 🛠️ Tech Stack & Dependencies

This project leverages the **Rust** ecosystem to ensure stability and performance. Key libraries used:

| Crate | Usage in Project |
| :--- | :--- |
| **`eframe`** | **GUI Framework.** Provides the immediate mode interface (based on egui), ensuring a responsive and modern look. |
| **`enigo`** | **Input Simulation.** Handles mouse clicks and keyboard events to interact with the game mechanics. |
| **`active-win-pos-rs`** | **Window Detection.** Ensures the macro only triggers when the Roblox window is actually active/focused. |
| **`rand`** | **Humanization.** Generates random intervals between actions to mimic human behavior and avoid detection. |
| **`image`** | **Asset Handling.** Manages icon loading and rendering within the GUI. |
## 📦 How to Run
1.  Clone the repository:
    ```bash
    git clone [https://github.com/YOUR_USERNAME/The-Forge-Macro.git](https://github.com/YOUR_USERNAME/The-Forge-Macro.git)
    ```
2.  Navigate to the directory and run:
    ```bash
    cargo run --release
    ```
    
## ⚠️ Disclaimer
This software is for **educational purposes only**. Using automation tools may violate Roblox Terms of Service. The author is not responsible for any account penalties. Use at your own risk.
---
**Author:** @x1000z1
