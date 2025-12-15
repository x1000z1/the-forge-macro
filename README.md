# The Forge Macro (TFM)
**TFM** is a lightweight, high-performance automation tool for the "The Forge" game mode in Roblox. Built entirely in **Rust**, it focuses on speed, safety, and low resource usage.

<div align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/6809e8d8-fd37-4279-877b-b14da316c515" />
<img width="302" height="282" alt="image" src="https://github.com/user-attachments/assets/60556a6a-09de-4b8e-a38b-e26cee16a89b" />
</div>

## 🚀 Features
* **⛏️ Mining Clicker** – Automated mining to save your mouse and time.
* **🍀 Luck Potion** – Automatically manages and consumes luck potions.
* **💰 Auto Sell** – Prevents inventory overflow by selling resources automatically.
* **💾 Config Persistence** – Automatically saves and loads your settings (Keys, Timers, Window Position) to `settings.json`.

## 🛠️ Tech Stack & Dependencies

This project leverages the **Rust** ecosystem to ensure stability and performance. Key libraries used:

| Crate | Usage in Project |
| :--- | :--- |
| **`eframe`** | **GUI Framework.** Provides the immediate mode interface (based on egui), ensuring a responsive and modern look. |
| **`enigo`** | **Input Simulation.** Handles mouse clicks and keyboard events to interact with the game mechanics. |
| **`active-win-pos-rs`** | **Window Detection.** Ensures the macro only triggers when the Roblox window is actually active/focused. |
| **`rand`** | **Humanization.** Generates random intervals between actions to mimic human behavior and avoid detection. |
| **`image`** | **Asset Handling.** Manages icon loading (PNG) and rendering within the GUI. |
| **`serde` & `serde_json`** | **Data Serialization.** Handles saving and loading user configuration to/from JSON files. |
| **`winres`** | **Windows Resources.** Embeds application icons and metadata into the final `.exe` file during the build process. |

## 📦 How to Run
1.  Clone the repository:
    ```bash
    git clone [https://github.com/x1000z1/the-forge-macro.git](https://github.com/x1000z1/the-forge-macro.git)
    ```
2.  Navigate to the directory and run:
    ```bash
    cargo run --release
    ```

> **Note:** The release profile is highly optimized for binary size (`strip = true`, `opt-level = "z"`), resulting in a very lightweight executable.

## ⚠️ Disclaimer
This software is for **educational purposes only**. Using automation tools may violate Roblox Terms of Service. The author is not responsible for any account penalties. Use at your own risk.

---
**Author:** @x1000z1
