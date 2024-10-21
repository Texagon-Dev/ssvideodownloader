import Downloader from "./Downloader";
import { invoke } from "@tauri-apps/api/core";

import "./output.css";

function App() {
  return (
    <main className="overflow-hidden min-h-screen animate-gradient bg-[length:300%_300%] bg-gradient-to-br from-gray-800 via-red-900 to-gray-800">
      <Downloader />
    </main>
  );
}

export default App;
