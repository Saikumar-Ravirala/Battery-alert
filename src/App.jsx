// // import { useEffect, useState } from "react";
// // import { invoke } from "@tauri-apps/api/tauri";

// // function App() {
// //   const [batteryInfo, setBatteryInfo] = useState("Checking battery status...");
// //   const [error, setError] = useState(null);
// //   const [lastUpdated, setLastUpdated] = useState(null);

// //   const checkBattery = async () => {
// //     try {
// //       const info = await invoke("get_battery_info");
// //       setBatteryInfo(info);
// //       setError(null);
// //       setLastUpdated(new Date().toLocaleTimeString());
// //     } catch (err) {
// //       setError(`Error: ${err.message || 'Failed to get battery info'}`);
// //       console.error('Battery check failed:', err);
// //     }
// //   };

// //   useEffect(() => {
// //     // Initial check
// //     checkBattery();
    
// //     // Set up interval for periodic checks
// //     const interval = setInterval(checkBattery, 30000); // every 30 seconds
    
// //     // Clean up interval on component unmount
// //     return () => clearInterval(interval);
// //   }, []);

// //   return (
// //     <div className="min-h-screen bg-gray-100 flex flex-col items-center justify-center p-4">
// //       <div className="bg-white p-6 rounded-lg shadow-md w-full max-w-md">
// //         <h1 className="text-2xl font-bold text-gray-800 mb-4 flex items-center justify-center">
// //           <span className="mr-2">🔋</span> Battery Monitor
// //         </h1>
        
// //         <div className="bg-gray-50 p-4 rounded-md mb-4">
// //           <p className="text-lg font-medium text-center">
// //             {error || batteryInfo}
// //           </p>
// //         </div>
        
// //         {lastUpdated && (
// //           <p className="text-sm text-gray-500 text-center">
// //             Last updated: {lastUpdated}
// //           </p>
// //         )}
        
// //         <button
// //           onClick={checkBattery}
// //           className="mt-4 w-full bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 px-4 rounded-md transition-colors"
// //         >
// //           Refresh Now
// //         </button>
// //       </div>
// //     </div>
// //   );
// // }

// // export default App;
// // src/App.jsx
// import React, { useEffect, useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
// import { listen } from "@tauri-apps/api/event";

// export default function App() {
//   const [percent, setPercent] = useState(null);
//   const [state, setState] = useState("Unknown");

//   useEffect(() => {
//     // initial fetch
//     getStatus();

//     // listen to background events
//     const unlisten = listen("battery-update", (event) => {
//       const payload = event.payload;
//       if (payload) {
//         setPercent(Math.round(payload.percent));
//         setState(payload.state);
//       }
//     });

//     return () => {
//       unlisten.then(f => f());
//     };
//   }, []);

//   async function getStatus() {
//     try {
//       const res = await invoke("get_battery_status");
//       if (res) {
//         setPercent(Math.round(res.percent));
//         setState(res.state);
//       }
//     } catch (e) {
//       console.error("get_battery_status error", e);
//     }
//   }

//   return (
//     <div style={{
//       width: 300,
//       height: 200,
//       display: "flex",
//       flexDirection: "column",
//       alignItems: "center",
//       justifyContent: "center",
//       fontFamily: "Arial, sans-serif",
//       gap: 12,
//       padding: 12
//     }}>
//       <h3>Battery Monitor</h3>
//       <div style={{ fontSize: 36 }}>
//         {percent === null ? "—" : `${percent}%`}
//       </div>
//       <div style={{ color: "#666" }}>{state}</div>
//       <div style={{ display: "flex", gap: 8 }}>
//         <button onClick={getStatus} style={{ padding: "6px 12px" }}>Check Now</button>
//       </div>
//     </div>
//   );
// }
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { enable } from '@tauri-apps/plugin-autostart';
enable();


function App() {
  const [battery, setBattery] = useState({ percent: 0, state: "Unknown" });

  const fetchBattery = async () => {
    const status = await invoke("get_battery_status");
    setBattery(status);
  };

  useEffect(() => {
    fetchBattery();
    const interval = setInterval(fetchBattery, 60000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ textAlign: "center", padding: "20px" }}>
      <h2>Battery Monitor</h2>
      <p>Percentage: {battery.percent.toFixed(0)}%</p>
      <p>Status: {battery.state}</p>
    </div>
  );
}

export default App;
