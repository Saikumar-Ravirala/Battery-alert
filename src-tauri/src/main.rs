// // // // // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// // // // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // // // use battery::Manager as BatteryManager;
// // // // use tauri::Manager;

// // // // #[tauri::command]
// // // // fn get_battery_info() -> Result<String, String> {
// // // //     let manager = BatteryManager::new().map_err(|e| e.to_string())?;
// // // //     let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

// // // //     if let Some(battery) = batteries.next() {
// // // //         let b = battery.map_err(|e| e.to_string())?;
// // // //         let percent = b.state_of_charge().value * 100.0;
// // // //         let status = format!("{:?}", b.state());
// // // //         return Ok(format!("Battery: {:.0}% | Status: {}", percent, status));
// // // //     }

// // // //     Err("No battery found".into())
// // // // }

// // // // fn main() {
// // // //     tauri::Builder::default()
// // // //         .invoke_handler(tauri::generate_handler![get_battery_info])
// // // //         .run(tauri::generate_context!())
// // // //         .expect("error while running tauri application");
// // // // }
// // // // src-tauri/src/main.rs// src-tauri/src/main.rs
// // // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // // use std::{thread, time::Duration};
// // // use tauri::{
// // //   Manager,
// // //   WindowUrl,
// // //   get_window, // helper alias
// // // };
// // // use tauri::tray::TrayIconBuilder;
// // // use tauri::menu::{Menu, MenuItem};
// // // use tauri::Window;
// // // use tauri_plugin_autostart::MacosLauncher;
// // // use tauri_plugin_notification::NotificationExt;
// // // use battery::{Manager as BatteryManager, State};
// // // use serde_json::json;

// // // // Helper to show notifications via the notification plugin
// // // fn show_notification(app: &tauri::AppHandle, title: &str, body: &str) {
// // //   let _ = app
// // //     .notification()
// // //     .builder()
// // //     .title(title)
// // //     .body(body)
// // //     .show();
// // // }

// // // fn main() {
// // //   tauri::Builder::default()
// // //     .setup(|app| {
// // //       // Create a (hidden) main webview window which will load your frontend index.html
// // //       // If you already create window in tauri.conf.json, you can skip creating it here.
// // //       let _ = tauri::WindowBuilder::new(
// // //         app,
// // //         "main", // window label
// // //         WindowUrl::App("index.html".into()),
// // //       )
// // //       .title("Battery Monitor")
// // //       .visible(false) // keep hidden at startup
// // //       .inner_size(300.0, 200.0)
// // //       .build()?;

// // //       // Build a native menu for the tray (Rust side)
// // //       let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
// // //       let toggle_item = MenuItem::with_id(app, "toggle", "Show/Hide", true, None::<&str>)?;
// // //       let autostart_item = MenuItem::with_id(app, "autostart", "Autostart Info", true, None::<&str>)?;
// // //       let menu = Menu::with_items(app, &[&toggle_item, &autostart_item, &quit_item])?;

// // //       // Build tray icon (uses your app default window icon if you set one)
// // //       let tray = TrayIconBuilder::new()
// // //         .menu(&menu)
// // //         .menu_on_left_click(true)
// // //         .icon(app.default_window_icon().unwrap().clone()) // make sure you have an icon set in tauri.conf.json
// // //         .tooltip("Battery Monitor")
// // //         .build(app)?;

// // //       // Start a background thread to monitor battery (safe simple approach)
// // //       let app_handle = app.handle();
// // //       thread::spawn(move || {
// // //         // repeat protections
// // //         let mut low_notified = false;
// // //         let mut full_notified = false;
// // //         let mut low_repeat = 0u8;
// // //         let mut full_repeat = 0u8;

// // //         loop {
// // //           match BatteryManager::new() {
// // //             Ok(manager) => {
// // //               if let Ok(mut batteries) = manager.batteries() {
// // //                 while let Some(b_res) = batteries.next() {
// // //                   if let Ok(battery) = b_res {
// // //                     let percent = battery.state_of_charge().value * 100.0;
// // //                     let state = battery.state();

// // //                     // Emit an event to the frontend windows (if any)
// // //                     // app_handle.emit_all is used to broadcast event to all windows
// // //                     let _ = app_handle.emit_all("battery-update", json!({
// // //                       "percent": percent,
// // //                       "state": format!("{:?}", state)
// // //                     }));

// // //                     // Low notification: <=30% and not charging
// // //                     if percent <= 30.0 && state != State::Charging {
// // //                       if !low_notified || (low_repeat < 3) {
// // //                         show_notification(&app_handle, "Battery Low ⚠️", &format!("Battery: {}%", percent as i32));
// // //                         low_notified = true;
// // //                         low_repeat = low_repeat.saturating_add(1);
// // //                       }
// // //                       // reset full flags
// // //                       full_notified = false;
// // //                       full_repeat = 0;
// // //                     } else {
// // //                       // reset low when condition not met
// // //                       low_notified = false;
// // //                       low_repeat = 0;
// // //                     }

// // //                     // High (charging) notification: >=90% while charging
// // //                     if percent >= 90.0 && state == State::Charging {
// // //                       if !full_notified || (full_repeat < 3) {
// // //                         show_notification(&app_handle, "Battery High 🔋", &format!("Battery: {}% (charging)", percent as i32));
// // //                         full_notified = true;
// // //                         full_repeat = full_repeat.saturating_add(1);
// // //                       }
// // //                       // reset low flags
// // //                       low_notified = false;
// // //                       low_repeat = 0;
// // //                     } else {
// // //                       if state != State::Charging || percent < 90.0 {
// // //                         full_notified = false;
// // //                         full_repeat = 0;
// // //                       }
// // //                     }
// // //                   }
// // //                 }
// // //               }
// // //             }
// // //             Err(e) => {
// // //               eprintln!("Battery manager error: {}", e);
// // //             }
// // //           }

// // //           // Sleep between checks (60 seconds)
// // //           thread::sleep(Duration::from_secs(60));
// // //         }
// // //       });

// // //       Ok(())
// // //     })
// // //     // handle tray events (menu click and left click)
// // //     .on_system_tray_event(|app, event| {
// // //       use tauri::tray::{TrayIconEvent, MouseButton, MouseButtonState};
// // //       match event {
// // //         // Left click: toggle main window visibility
// // //         TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
// // //           if let Some(win) = app.get_webview_window("main") {
// // //             match win.is_visible() {
// // //               Ok(true) => { let _ = win.hide(); }
// // //               _ => { let _ = win.show(); let _ = win.set_focus(); }
// // //             }
// // //           }
// // //         }
// // //         // Menu item click
// // //         tauri::tray::TrayIconEvent::MenuItemClick { id, .. } => {
// // //           match id.as_ref() {
// // //             "quit" => {
// // //               app.exit(0);
// // //             }
// // //             "toggle" => {
// // //               if let Some(win) = app.get_webview_window("main") {
// // //                 match win.is_visible() {
// // //                   Ok(true) => { let _ = win.hide(); }
// // //                   _ => { let _ = win.show(); let _ = win.set_focus(); }
// // //                 }
// // //               }
// // //             }
// // //             "autostart" => {
// // //               // quick informational notification
// // //               let _ = app.notification().builder()
// // //                 .title("Autostart")
// // //                 .body("Build the app and add a shortcut to the Windows Startup folder (shell:startup) to enable autostart.")
// // //                 .show();
// // //             }
// // //             _ => {}
// // //           }
// // //         }
// // //         _ => {}
// // //       }
// // //     })
// // //     // plugins
// // //     .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
// // //     .plugin(tauri_plugin_notification::init())
// // //     // allow frontend to call this command
// // //     .invoke_handler(tauri::generate_handler![get_battery_status])
// // //     .run(tauri::generate_context!())
// // //     .expect("error while running tauri app");
// // // }

// // // // Command callable from the frontend to get the current status immediately
// // // #[tauri::command]
// // // fn get_battery_status() -> Result<serde_json::Value, String> {
// // //   let manager = BatteryManager::new().map_err(|e| e.to_string())?;
// // //   let mut batteries = manager.batteries().map_err(|e| e.to_string())?;
// // //   if let Some(b_res) = batteries.next() {
// // //     let battery = b_res.map_err(|e| e.to_string())?;
// // //     let percent = battery.state_of_charge().value * 100.0;
// // //     let state = format!("{:?}", battery.state());
// // //     return Ok(json!({ "percent": percent, "state": state }));
// // //   }
// // //   Err("No battery found".into())
// // // }
// // // // src-tauri/src/main.rs
// // // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // // use std::{thread, time::Duration};
// // // use tauri::{
// // //   Manager,
// // //   WindowUrl,
// // //   get_window, // helper alias
// // // };
// // // use tauri::tray::TrayIconBuilder;
// // // use tauri::menu::{Menu, MenuItem};
// // // use tauri::Window;
// // // use tauri_plugin_autostart::MacosLauncher;
// // // use tauri_plugin_notification::NotificationExt;
// // // use battery::{Manager as BatteryManager, State};
// // // use serde_json::json;

// // // // Helper to show notifications via the notification plugin
// // // fn show_notification(app: &tauri::AppHandle, title: &str, body: &str) {
// // //   let _ = app
// // //     .notification()
// // //     .builder()
// // //     .title(title)
// // //     .body(body)
// // //     .show();
// // // }

// // // fn main() {
// // //   tauri::Builder::default()
// // //     .setup(|app| {
// // //       // Create a (hidden) main webview window which will load your frontend index.html
// // //       // If you already create window in tauri.conf.json, you can skip creating it here.
// // //       let _ = tauri::WindowBuilder::new(
// // //         app,
// // //         "main", // window label
// // //         WindowUrl::App("index.html".into()),
// // //       )
// // //       .title("Battery Monitor")
// // //       .visible(false) // keep hidden at startup
// // //       .inner_size(300.0, 200.0)
// // //       .build()?;

// // //       // Build a native menu for the tray (Rust side)
// // //       let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
// // //       let toggle_item = MenuItem::with_id(app, "toggle", "Show/Hide", true, None::<&str>)?;
// // //       let autostart_item = MenuItem::with_id(app, "autostart", "Autostart Info", true, None::<&str>)?;
// // //       let menu = Menu::with_items(app, &[&toggle_item, &autostart_item, &quit_item])?;

// // //       // Build tray icon (uses your app default window icon if you set one)
// // //       let tray = TrayIconBuilder::new()
// // //         .menu(&menu)
// // //         .menu_on_left_click(true)
// // //         .icon(app.default_window_icon().unwrap().clone()) // make sure you have an icon set in tauri.conf.json
// // //         .tooltip("Battery Monitor")
// // //         .build(app)?;

// // //       // Start a background thread to monitor battery (safe simple approach)
// // //       let app_handle = app.handle();
// // //       thread::spawn(move || {
// // //         // repeat protections
// // //         let mut low_notified = false;
// // //         let mut full_notified = false;
// // //         let mut low_repeat = 0u8;
// // //         let mut full_repeat = 0u8;

// // //         loop {
// // //           match BatteryManager::new() {
// // //             Ok(manager) => {
// // //               if let Ok(mut batteries) = manager.batteries() {
// // //                 while let Some(b_res) = batteries.next() {
// // //                   if let Ok(battery) = b_res {
// // //                     let percent = battery.state_of_charge().value * 100.0;
// // //                     let state = battery.state();

// // //                     // Emit an event to the frontend windows (if any)
// // //                     // app_handle.emit_all is used to broadcast event to all windows
// // //                     let _ = app_handle.emit_all("battery-update", json!({
// // //                       "percent": percent,
// // //                       "state": format!("{:?}", state)
// // //                     }));

// // //                     // Low notification: <=30% and not charging
// // //                     if percent <= 30.0 && state != State::Charging {
// // //                       if !low_notified || (low_repeat < 3) {
// // //                         show_notification(&app_handle, "Battery Low ⚠️", &format!("Battery: {}%", percent as i32));
// // //                         low_notified = true;
// // //                         low_repeat = low_repeat.saturating_add(1);
// // //                       }
// // //                       // reset full flags
// // //                       full_notified = false;
// // //                       full_repeat = 0;
// // //                     } else {
// // //                       // reset low when condition not met
// // //                       low_notified = false;
// // //                       low_repeat = 0;
// // //                     }

// // //                     // High (charging) notification: >=90% while charging
// // //                     if percent >= 90.0 && state == State::Charging {
// // //                       if !full_notified || (full_repeat < 3) {
// // //                         show_notification(&app_handle, "Battery High 🔋", &format!("Battery: {}% (charging)", percent as i32));
// // //                         full_notified = true;
// // //                         full_repeat = full_repeat.saturating_add(1);
// // //                       }
// // //                       // reset low flags
// // //                       low_notified = false;
// // //                       low_repeat = 0;
// // //                     } else {
// // //                       if state != State::Charging || percent < 90.0 {
// // //                         full_notified = false;
// // //                         full_repeat = 0;
// // //                       }
// // //                     }
// // //                   }
// // //                 }
// // //               }
// // //             }
// // //             Err(e) => {
// // //               eprintln!("Battery manager error: {}", e);
// // //             }
// // //           }

// // //           // Sleep between checks (60 seconds)
// // //           thread::sleep(Duration::from_secs(60));
// // //         }
// // //       });

// // //       Ok(())
// // //     })
// // //     // handle tray events (menu click and left click)
// // //     .on_system_tray_event(|app, event| {
// // //       use tauri::tray::{TrayIconEvent, MouseButton, MouseButtonState};
// // //       match event {
// // //         // Left click: toggle main window visibility
// // //         TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
// // //           if let Some(win) = app.get_webview_window("main") {
// // //             match win.is_visible() {
// // //               Ok(true) => { let _ = win.hide(); }
// // //               _ => { let _ = win.show(); let _ = win.set_focus(); }
// // //             }
// // //           }
// // //         }
// // //         // Menu item click
// // //         tauri::tray::TrayIconEvent::MenuItemClick { id, .. } => {
// // //           match id.as_ref() {
// // //             "quit" => {
// // //               app.exit(0);
// // //             }
// // //             "toggle" => {
// // //               if let Some(win) = app.get_webview_window("main") {
// // //                 match win.is_visible() {
// // //                   Ok(true) => { let _ = win.hide(); }
// // //                   _ => { let _ = win.show(); let _ = win.set_focus(); }
// // //                 }
// // //               }
// // //             }
// // //             "autostart" => {
// // //               // quick informational notification
// // //               let _ = app.notification().builder()
// // //                 .title("Autostart")
// // //                 .body("Build the app and add a shortcut to the Windows Startup folder (shell:startup) to enable autostart.")
// // //                 .show();
// // //             }
// // //             _ => {}
// // //           }
// // //         }
// // //         _ => {}
// // //       }
// // //     })
// // //     // plugins
// // //     .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
// // //     .plugin(tauri_plugin_notification::init())
// // //     // allow frontend to call this command
// // //     .invoke_handler(tauri::generate_handler![get_battery_status])
// // //     .run(tauri::generate_context!())
// // //     .expect("error while running tauri app");
// // // }

// // // // Command callable from the frontend to get the current status immediately
// // // #[tauri::command]
// // // fn get_battery_status() -> Result<serde_json::Value, String> {
// // //   let manager = BatteryManager::new().map_err(|e| e.to_string())?;
// // //   let mut batteries = manager.batteries().map_err(|e| e.to_string())?;
// // //   if let Some(b_res) = batteries.next() {
// // //     let battery = b_res.map_err(|e| e.to_string())?;
// // //     let percent = battery.state_of_charge().value * 100.0;
// // //     let state = format!("{:?}", battery.state());
// // //     return Ok(json!({ "percent": percent, "state": state }));
// // //   }
// // //   Err("No battery found".into())
// // // // }
// // // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // // use std::time::{Duration, Instant};
// // // // use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayEvent, AppHandle};
// // // use tauri::{CustomMenuItem, AppHandle};
// // // use tauri_plugin_system_tray::{SystemTray, SystemTrayMenu, SystemTrayEvent};   
// // // use tauri::Emitter;   
// // // use tauri_plugin_autostart::MacosLauncher;
// // // use tauri_plugin_notification::NotificationExt;
// // // use battery::{Manager as BatteryManager, State};
// // // use serde_json::json;

// // // fn main() {
// // //     // Tray menu items
// // //     let quit = CustomMenuItem::new("quit".to_string(), "Quit");
// // //     let tray_menu = SystemTrayMenu::new().add_item(quit);
// // //     let system_tray = SystemTray::new().with_menu(tray_menu);

// // //     tauri::Builder::default()
// // //         .setup(|app| {
// // //             let app_handle = app.handle();

// // //             // Spawn battery monitoring in background
// // //             tauri::async_runtime::spawn(monitor_battery(app_handle.clone()));

// // //             Ok(())
// // //         })
// // //         .system_tray(system_tray)
// // //         .on_system_tray_event(|app, event| {
// // //             match event {
// // //                 SystemTrayEvent::MenuItemClick { id, .. } => {
// // //                     if id.as_str() == "quit" {
// // //                         std::process::exit(0);
// // //                     }
// // //                 }
// // //                 _ => {}
// // //             }
// // //         })
// // //         .plugin(tauri_plugin_autostart::init(
// // //             MacosLauncher::LaunchAgent,
// // //             None,
// // //         ))
// // //         .plugin(tauri_plugin_notification::init())
// // //         .invoke_handler(tauri::generate_handler![get_battery_status])
// // //         .run(tauri::generate_context!())
// // //         .expect("error while running tauri app");
// // // }

// // // async fn monitor_battery(app: AppHandle) {
// // //     let mut low_sent_count = 0;
// // //     let mut high_sent_count = 0;
// // //     let mut last_low: Option<Instant> = None;
// // //     let mut last_high: Option<Instant> = None;

// // //     loop {
// // //         if let Ok(manager) = BatteryManager::new() {
// // //             if let Ok(mut batteries) = manager.batteries() {
// // //                 while let Some(Ok(battery)) = batteries.next() {
// // //                     let percent = battery.state_of_charge().value * 100.0;
// // //                     let state = battery.state();

// // //                     // Low battery <30%
// // //                     if percent <= 30.0 && state != State::Charging {
// // //                         let should_notify = if low_sent_count < 5 {
// // //                             last_low.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
// // //                         } else { false };

// // //                         if should_notify {
// // //                             show_notification(
// // //                                 &app,
// // //                                 "Battery Dying! 😫",
// // //                                 &format!("Bro, battery dying. Plug in fast! 🔋😫 ({:.0}%)", percent)
// // //                             );
// // //                             low_sent_count += 1;
// // //                             last_low = Some(Instant::now());
// // //                         }
// // //                     } else {
// // //                         low_sent_count = 0;
// // //                         last_low = None;
// // //                     }

// // //                     // High battery >=90%
// // //                     if percent >= 90.0 && state == State::Charging {
// // //                         let should_notify = if high_sent_count < 5 {
// // //                             last_high.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
// // //                         } else { false };

// // //                         if should_notify {
// // //                             show_notification(
// // //                                 &app,
// // //                                 "Battery Full! 😎",
// // //                                 &format!("Unplug yaar, battery already full! 🔌😎 ({:.0}%)", percent)
// // //                             );
// // //                             high_sent_count += 1;
// // //                             last_high = Some(Instant::now());
// // //                         }
// // //                     } else {
// // //                         high_sent_count = 0;
// // //                         last_high = None;
// // //                     }

// // //                     // Emit battery status to frontend
// // //                     let _ = app.emit("battery-update", json!({
// // //                         "percent": percent,
// // //                         "state": format!("{:?}", state)
// // //                     }));
// // //                 }
// // //             }
// // //         }

// // //         tokio::time::sleep(Duration::from_secs(60)).await;
// // //     }
// // // }

// // // fn show_notification(app: &AppHandle, title: &str, body: &str) {
// // //     let _ = app.notification()
// // //         .title(title)
// // //         .body(body)
// // //         .show();
// // // }

// // // #[tauri::command]
// // // fn get_battery_status() -> Result<serde_json::Value, String> {
// // //     let manager = BatteryManager::new().map_err(|e| e.to_string())?;
// // //     let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

// // //     if let Some(Ok(battery)) = batteries.next() {
// // //         let percent = battery.state_of_charge().value * 100.0;
// // //         let state = format!("{:?}", battery.state());
// // //         return Ok(json!({ "percent": percent, "state": state }));
// // //     }

// // //     Err("No battery found".into())
// // // }
// // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // use std::time::{Duration, Instant};
// // use tauri::{AppHandle, Manager, Wry}; // ADD Wry and Manager for trait context
// // use tauri::menu::{Menu, MenuItem}; // V2 uses tauri::menu instead of system_tray::* for the menu items
// // use tauri::tray::{TrayIconBuilder, TrayIconEvent}; // V2 uses tauri::tray for the tray icon
// // use tauri_plugin_autostart::MacosLauncher;
// // use tauri_plugin_notification::NotificationExt;
// // use battery::{Manager as BatteryManager, State};
// // use serde_json::json;

// // // Function to show a notification using the NotificationExt trait
// // fn show_notification(app: &AppHandle<Wry>, title: &str, body: &str) {
// //     let _ = app.notification()
// //         .builder()
// //         .title(title)
// //         .body(body)
// //         .show();
// // }

// // fn main() {
// //     tauri::Builder::default()
// //         .setup(|app| {
// //             let app_handle = app.handle().clone(); // Clone for the async task

// //             // V2 TRAY SETUP
// //             // ------------------------------------------------------------------
// //             let quit_item = MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap();
// //             let tray_menu = Menu::with_items(&app_handle, &[&quit_item]).unwrap();
            
// //             // Build the tray icon
// //             let _tray_icon = TrayIconBuilder::new()
// //                 .with_id("main-tray")
// //                 .menu(&tray_menu)
// //                 .on_menu_event(move |app_handle, event| {
// //                     if event.id.as_ref() == "quit" {
// //                         app_handle.exit(0);
// //                     }
// //                 })
// //                 .build(app)?; // Use ? to propagate the error

// //             // ------------------------------------------------------------------

// //             // Spawn battery monitoring in background
// //             tauri::async_runtime::spawn(monitor_battery(app_handle.clone()));

// //             Ok(())
// //         })
// //         // REMOVED: .system_tray(system_tray)
// //         // REMOVED: .on_system_tray_event(...) - Handled by .on_menu_event above
// //         .plugin(tauri_plugin_autostart::init(
// //             MacosLauncher::LaunchAgent,
// //             None,
// //         ))
// //         .plugin(tauri_plugin_notification::init())
// //         .invoke_handler(tauri::generate_handler![get_battery_status])
// //         .run(tauri::generate_context!())
// //         .expect("error while running tauri app");
// // }

// // async fn monitor_battery(app: AppHandle<Wry>) { // ADD Wry
// //     let mut low_sent_count = 0;
// //     let mut high_sent_count = 0;
// //     let mut last_low: Option<Instant> = None;
// //     let mut last_high: Option<Instant> = None;

// //     loop {
// //         // FIX: Moved manager creation inside the loop to avoid non-Send errors
// //         if let Ok(manager) = BatteryManager::new() { 
// //             if let Ok(mut batteries) = manager.batteries() {
// //                 while let Some(Ok(battery)) = batteries.next() {
// //                     let percent = battery.state_of_charge().value * 100.0;
// //                     let state = battery.state();

// //                     // Low battery <30%
// //                     if percent <= 30.0 && state != State::Charging {
// //                         let should_notify = if low_sent_count < 5 {
// //                             last_low.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
// //                         } else { false };

// //                         if should_notify {
// //                             show_notification(
// //                                 &app,
// //                                 "Battery Dying! 😫",
// //                                 &format!("Bro, battery dying. Plug in fast! 🔋😫 ({:.0}%)", percent)
// //                             );
// //                             low_sent_count += 1;
// //                             last_low = Some(Instant::now());
// //                         }
// //                     } else {
// //                         low_sent_count = 0;
// //                         last_low = None;
// //                     }

// //                     // High battery >=90%
// //                     if percent >= 90.0 && state == State::Charging {
// //                         let should_notify = if high_sent_count < 5 {
// //                             last_high.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
// //                         } else { false };

// //                         if should_notify {
// //                             show_notification(
// //                                 &app,
// //                                 "Battery Full! 😎",
// //                                 &format!("Unplug yaar, battery already full! 🔌😎 ({:.0}%)", percent)
// //                             );
// //                             high_sent_count += 1;
// //                             last_high = Some(Instant::now());
// //                         }
// //                     } else {
// //                         high_sent_count = 0;
// //                         last_high = None;
// //                     }

// //                     // Emit battery status to frontend
// //                     let _ = app.emit("battery-update", json!({
// //                         "percent": percent,
// //                         "state": format!("{:?}", state)
// //                     }));
// //                 }
// //             }
// //         }
// //         // Async sleep
// //         tokio::time::sleep(Duration::from_secs(60)).await;
// //     }
// // }

// // // Ensure the AppHandle has the Wry type parameter
// // #[tauri::command]
// // fn get_battery_status() -> Result<serde_json::Value, String> {
// //     // This command is okay as it doesn't cross an await point
// //     let manager = BatteryManager::new().map_err(|e| e.to_string())?;
// //     let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

// //     if let Some(Ok(battery)) = batteries.next() {
// //         let percent = battery.state_of_charge().value * 100.0;
// //         let state = format!("{:?}", battery.state());
// //         return Ok(json!({ "percent": percent, "state": state }));
// //     }

// //     Err("No battery found".into())
// // // }
// // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // use std::time::{Duration, Instant};
// // use tauri::{AppHandle, Wry, Emitter}; // Added Emitter, removed Manager
// // use tauri::menu::{Menu, MenuItem}; // V2 Core Menu
// // use tauri::tray::TrayIconBuilder; // V2 Core Tray Icon
// // use tauri_plugin_autostart::MacosLauncher;
// // use tauri_plugin_notification::NotificationExt;
// // use battery::{Manager as BatteryManager, State};
// // use serde_json::json;

// // // Function to show a notification using the NotificationExt trait
// // fn show_notification(app: &AppHandle<Wry>, title: &str, body: &str) {
// //     let _ = app.notification()
// //         .builder()
// //         .title(title)
// //         .body(body)
// //         .show();
// // }

// // fn main() {
// //     tauri::Builder::default()
// //         .setup(|app| {
// //             let app_handle = app.handle().clone();

// //             // V2 TRAY SETUP using tauri::menu and tauri::tray
// //             // ------------------------------------------------------------------
// //             let quit_item = MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap();
// //             let tray_menu = Menu::with_items(&app_handle, &[&quit_item]).unwrap();
            
// //             // Build the tray icon
// //             // FIX: Use associated function syntax for TrayIconBuilder::with_id
// //             let _tray_icon = TrayIconBuilder::new()
// //                 .with_id("main-tray", app)? // Calling associated function with app handle reference
// //                 .menu(&tray_menu)
// //                 .on_menu_event(move |app_handle, event| {
// //                     if event.id.as_ref() == "quit" {
// //                         app_handle.exit(0);
// //                     }
// //                 })
// //                 .build(app)?;

// //             // ------------------------------------------------------------------

// //             // Spawn battery monitoring in background
// //             tauri::async_runtime::spawn(monitor_battery(app_handle.clone()));

// //             Ok(())
// //         })
// //         // The old .system_tray and .on_system_tray_event calls are now removed 
// //         // as the tray is initialized in .setup and events are handled in .on_menu_event.
// //         .plugin(tauri_plugin_autostart::init(
// //             MacosLauncher::LaunchAgent,
// //             None,
// //         ))
// //         .plugin(tauri_plugin_notification::init())
// //         .invoke_handler(tauri::generate_handler![get_battery_status])
// //         .run(tauri::generate_context!())
// //         .expect("error while running tauri app");
// // }

// // async fn monitor_battery(app: AppHandle<Wry>) {
// //     let mut low_sent_count = 0;
// //     let mut high_sent_count = 0;
// //     let mut last_low: Option<Instant> = None;
// //     let mut last_high: Option<Instant> = None;

// //     loop {
// //         // Create the manager inside the loop to avoid non-Send errors across await
// //         if let Ok(manager) = BatteryManager::new() { 
// //             if let Ok(mut batteries) = manager.batteries() {
// //                 while let Some(Ok(battery)) = batteries.next() {
// //                     let percent = battery.state_of_charge().value * 100.0;
// //                     let state = battery.state();

// //                     // Low battery <30%
// //                     if percent <= 30.0 && state != State::Charging {
// //                         let should_notify = if low_sent_count < 5 {
// //                             // Only notify if it has been 5 minutes since the last notification
// //                             last_low.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
// //                         } else { false };

// //                         if should_notify {
// //                             show_notification(
// //                                 &app,
// //                                 "Battery Dying! 😫",
// //                                 &format!("Bro, battery dying. Plug in fast! 🔋😫 ({:.0}%)", percent)
// //                             );
// //                             low_sent_count += 1;
// //                             last_low = Some(Instant::now());
// //                         }
// //                     } else {
// //                         low_sent_count = 0;
// //                         last_low = None;
// //                     }

// //                     // High battery >=90%
// //                     if percent >= 90.0 && state == State::Charging {
// //                         let should_notify = if high_sent_count < 5 {
// //                             // Only notify if it has been 5 minutes since the last notification
// //                             last_high.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
// //                         } else { false };

// //                         if should_notify {
// //                             show_notification(
// //                                 &app,
// //                                 "Battery Full! 😎",
// //                                 &format!("Unplug yaar, battery already full! 🔌😎 ({:.0}%)", percent)
// //                             );
// //                             high_sent_count += 1;
// //                             last_high = Some(Instant::now());
// //                         }
// //                     } else {
// //                         high_sent_count = 0;
// //                         last_high = None;
// //                     }

// //                     // Emit battery status to frontend
// //                     let _ = app.emit("battery-update", json!({ // This line is now fixed by the 'use tauri::Emitter' import
// //                         "percent": percent,
// //                         "state": format!("{:?}", state)
// //                     }));
// //                 }
// //             }
// //         }
// //         // Async sleep
// //         tokio::time::sleep(Duration::from_secs(60)).await;
// //     }
// // }

// // #[tauri::command]
// // fn get_battery_status() -> Result<serde_json::Value, String> {
// //     let manager = BatteryManager::new().map_err(|e| e.to_string())?;
// //     let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

// //     if let Some(Ok(battery)) = batteries.next() {
// //         let percent = battery.state_of_charge().value * 100.0;
// //         let state = format!("{:?}", battery.state());
// //         return Ok(json!({ "percent": percent, "state": state }));
// //     }

// //     Err("No battery found".into())
// // }

// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::time::{Duration, Instant};
// use tauri::{AppHandle, Wry, Emitter}; 
// use tauri::menu::{Menu, MenuItem}; // V2 Core Menu
// use tauri::tray::TrayIconBuilder; // V2 Core Tray Icon
// use tauri_plugin_autostart::MacosLauncher;
// use tauri_plugin_notification::NotificationExt;
// use battery::{Manager as BatteryManager, State};
// use serde_json::json;

// // Function to show a notification using the NotificationExt trait
// fn show_notification(app: &AppHandle<Wry>, title: &str, body: &str) {
//     let _ = app.notification()
//         .builder()
//         .title(title)
//         .body(body)
//         .show();
// }

// fn main() {
//     tauri::Builder::default()
//         .setup(|app| {
//             let app_handle = app.handle().clone();

//             // V2 TRAY SETUP using tauri::menu and tauri::tray
//             // ------------------------------------------------------------------
//             let quit_item = MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap();
//             let tray_menu = Menu::with_items(&app_handle, &[&quit_item]).unwrap();
            
//             // Build the tray icon
//             // FIX: Use .with_id("main-tray"). This is the correct setter for the builder pattern,
//             // which returns Self (the builder) and allows chaining to .menu() and .build().
//             let _tray_icon = TrayIconBuilder::new()
//                 .with_id("main-tray") // CORRECTED: Using the setter method
//                 .menu(&tray_menu)
//                 .on_menu_event(move |app_handle, event| {
//                     if event.id.as_ref() == "quit" {
//                         app_handle.exit(0);
//                     }
//                 })
//                 .build(app)?;

//             // ------------------------------------------------------------------

//             // Spawn battery monitoring in background
//             tauri::async_runtime::spawn(monitor_battery(app_handle.clone()));

//             Ok(())
//         })
//         // The old .system_tray and .on_system_tray_event calls are now removed 
//         // as the tray is initialized in .setup and events are handled in .on_menu_event.
//         .plugin(tauri_plugin_autostart::init(
//             MacosLauncher::LaunchAgent,
//             None,
//         ))
//         .plugin(tauri_plugin_notification::init())
//         .invoke_handler(tauri::generate_handler![get_battery_status])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri app");
// }

// async fn monitor_battery(app: AppHandle<Wry>) {
//     let mut low_sent_count = 0;
//     let mut high_sent_count = 0;
//     let mut last_low: Option<Instant> = None;
//     let mut last_high: Option<Instant> = None;

//     loop {
//         // Create the manager inside the loop to avoid non-Send errors across await
//         if let Ok(manager) = BatteryManager::new() { 
//             if let Ok(mut batteries) = manager.batteries() {
//                 while let Some(Ok(battery)) = batteries.next() {
//                     let percent = battery.state_of_charge().value * 100.0;
//                     let state = battery.state();

//                     // Low battery <30%
//                     if percent <= 30.0 && state != State::Charging {
//                         let should_notify = if low_sent_count < 5 {
//                             // Only notify if it has been 5 minutes since the last notification
//                             last_low.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
//                         } else { false };

//                         if should_notify {
//                             show_notification(
//                                 &app,
//                                 "Battery Dying! 😫",
//                                 &format!("Bro, battery dying. Plug in fast! 🔋😫 ({:.0}%)", percent)
//                             );
//                             low_sent_count += 1;
//                             last_low = Some(Instant::now());
//                         }
//                     } else {
//                         low_sent_count = 0;
//                         last_low = None;
//                     }

//                     // High battery >=90%
//                     if percent >= 90.0 && state == State::Charging {
//                         let should_notify = if high_sent_count < 5 {
//                             // Only notify if it has been 5 minutes since the last notification
//                             last_high.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
//                         } else { false };

//                         if should_notify {
//                             show_notification(
//                                 &app,
//                                 "Battery Full! 😎",
//                                 &format!("Unplug yaar, battery already full! 🔌😎 ({:.0}%)", percent)
//                             );
//                             high_sent_count += 1;
//                             last_high = Some(Instant::now());
//                         }
//                     } else {
//                         high_sent_count = 0;
//                         last_high = None;
//                     }

//                     // Emit battery status to frontend
//                     let _ = app.emit("battery-update", json!({
//                         "percent": percent,
//                         "state": format!("{:?}", state)
//                     }));
//                 }
//             }
//         }
//         // Async sleep
//         tokio::time::sleep(Duration::from_secs(60)).await;
//     }
// }

// #[tauri::command]
// fn get_battery_status() -> Result<serde_json::Value, String> {
//     let manager = BatteryManager::new().map_err(|e| e.to_string())?;
//     let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

//     if let Some(Ok(battery)) = batteries.next() {
//         let percent = battery.state_of_charge().value * 100.0;
//         let state = format!("{:?}", battery.state());
//         return Ok(json!({ "percent": percent, "state": state }));
//     }

//     Err("No battery found".into())
// }


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{Duration, Instant};
use tauri::{AppHandle, Wry, Emitter}; 
use tauri::menu::{Menu, MenuItem}; // V2 Core Menu
use tauri::tray::TrayIconBuilder; // V2 Core Tray Icon
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_notification::NotificationExt;
use battery::{Manager as BatteryManager, State};
use serde_json::json;

// Function to show a notification using the NotificationExt trait
fn show_notification(app: &AppHandle<Wry>, title: &str, body: &str) {
    let _ = app.notification()
        .builder()
        .title(title)
        .body(body)
        .show();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            // V2 TRAY SETUP using tauri::menu and tauri::tray
            // ------------------------------------------------------------------
            let quit_item = MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap();
            let tray_menu = Menu::with_items(&app_handle, &[&quit_item]).unwrap();
            
            // Build the tray icon
            // FIX: The compiler indicates 'with_id' is an associated function, not a method. 
            // We use TrayIconBuilder::with_id() directly as the constructor, replacing TrayIconBuilder::new().
            let _tray_icon = TrayIconBuilder::with_id("main-tray") // CORRECTED: Use associated function as constructor
                .menu(&tray_menu)
                .on_menu_event(move |app_handle, event| {
                    if event.id.as_ref() == "quit" {
                        app_handle.exit(0);
                    }
                })
                .build(app)?;

            // ------------------------------------------------------------------

            // Spawn battery monitoring in background
            tauri::async_runtime::spawn(monitor_battery(app_handle.clone()));

            Ok(())
        })
        // The old .system_tray and .on_system_tray_event calls are now removed 
        // as the tray is initialized in .setup and events are handled in .on_menu_event.
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![get_battery_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}

async fn monitor_battery(app: AppHandle<Wry>) {
    let mut low_sent_count = 0;
    let mut high_sent_count = 0;
    let mut last_low: Option<Instant> = None;
    let mut last_high: Option<Instant> = None;

    loop {
        // Create the manager inside the loop to avoid non-Send errors across await
        if let Ok(manager) = BatteryManager::new() { 
            if let Ok(mut batteries) = manager.batteries() {
                while let Some(Ok(battery)) = batteries.next() {
                    let percent = battery.state_of_charge().value * 100.0;
                    let state = battery.state();

                    // Low battery <30%
                    if percent <= 30.0 && state != State::Charging {
                        let should_notify = if low_sent_count < 5 {
                            // Only notify if it has been 5 minutes since the last notification
                            last_low.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
                        } else { false };

                        if should_notify {
                            show_notification(
                                &app,
                                "Battery Dying! 😫",
                                &format!("Bro, battery dying. Plug in fast! 🔋😫 ({:.0}%)", percent)
                            );
                            low_sent_count += 1;
                            last_low = Some(Instant::now());
                        }
                    } else {
                        low_sent_count = 0;
                        last_low = None;
                    }

                    // High battery >=90%
                    if percent >= 90.0 && state == State::Charging {
                        let should_notify = if high_sent_count < 5 {
                            // Only notify if it has been 5 minutes since the last notification
                            last_high.map_or(true, |t| t.elapsed() >= Duration::from_secs(300))
                        } else { false };

                        if should_notify {
                            show_notification(
                                &app,
                                "Battery Full! 😎",
                                &format!("Unplug yaar, battery already full! 🔌😎 ({:.0}%)", percent)
                            );
                            high_sent_count += 1;
                            last_high = Some(Instant::now());
                        }
                    } else {
                        high_sent_count = 0;
                        last_high = None;
                    }

                    // Emit battery status to frontend
                    let _ = app.emit("battery-update", json!({
                        "percent": percent,
                        "state": format!("{:?}", state)
                    }));
                }
            }
        }
        // Async sleep
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

#[tauri::command]
fn get_battery_status() -> Result<serde_json::Value, String> {
    let manager = BatteryManager::new().map_err(|e| e.to_string())?;
    let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

    if let Some(Ok(battery)) = batteries.next() {
        let percent = battery.state_of_charge().value * 100.0;
        let state = format!("{:?}", battery.state());
        return Ok(json!({ "percent": percent, "state": state }));
    }

    Err("No battery found".into())
}
