# 🔋 Battery Alert (Tauri App)

A lightweight desktop app built with **Tauri**, **React (Vite)**, and **Rust**, that monitors your system’s battery level and sends real-time notifications when the charge goes **below 30%** or **above 90%**.  
The app runs silently in the **system tray** and can **auto-start** on system boot.

---

## 🚀 Features

- ✅ Real-time battery monitoring  
- 🔔 Notifications for low (<30%) or high (>90%) battery levels  
- 🪟 Runs silently in the system tray  
- ⚙️ Auto-starts on system boot  
- 🧩 Built with lightweight Tauri (fast and secure desktop runtime)

---

## 🧠 Tech Stack

| Layer | Technology |
|-------|-------------|
| Frontend | React + Vite |
| Backend | Rust (Tauri Framework) |
| UI Toolkit | HTML, CSS, JS |
| Notifications | Tauri Plugin Notification |
| Auto-start | Tauri Plugin Autostart |

---

## ⚙️ Installation & Setup

### 🧩 Prerequisites
Make sure you have the following installed:
- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/) (installed automatically with dependencies)

---

### 🛠️ Local Development

```bash
# Clone this repository
git clone https://github.com/<your-username>/battery-alert.git

# Navigate to the project
cd battery-alert

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

#🧱 Build Production App
npm run tauri build

The built executables will be available inside:

src-tauri/target/release/bundle/
You’ll find both .exe and .msi installers for Windows.

#🧰 Project Structure
battery-alert/
│
├── src/                   # Frontend React code
├── src-tauri/              # Rust backend code
│   ├── main.rs             # Main backend logic
│   ├── Cargo.toml          # Rust dependencies
│   └── icons/              # App icons
│
├── package.json            # Node dependencies
├── vite.config.js          # Vite config
├── README.md               # Project info (this file)
└── .gitignore

#📸 Screenshot
![Battery Alert Screenshot](./assets/battery-alert-v1-output.png)


#🧑‍💻 Author

Saikumar Ravirala
📍 Hyderabad, India
📧 saikumarravirala1@gmail.com

🌐 github.com/saikumar-ravirala