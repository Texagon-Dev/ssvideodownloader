import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import "./output.css";

function Downloader() {
  const [title, setTitle] = useState("");
  const [url, setUrl] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [thumbnail, setThumbnail] = useState("");

  async function getMetadata(e) {
    setIsLoading(true);
    console.log("Running getMetadata");
    const metadata = await invoke("get_title", { url });
    const [title, thumb] = metadata;
    setTitle(title);
    setThumbnail(thumb);

    console.log(thumb);
  }

  useEffect(() => {
    setIsLoading(false);
  }, [title, thumbnail]);

  // Simulate loading an image
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsLoading(false); // Simulate image load complete after 3 seconds
    }, 3000);
    return () => clearTimeout(timer);
  }, []);

  return (
    <main className="flex flex-col items-center justify-center">
      <h1 className="text-4xl text-white p-10 text-center">
        {title ? `${title}` : `Super Simple Video Downloader`}
      </h1>

      {/* Image Placeholder Container */}
      <div className="max-w-128 max-h-128 w-1/2 h-1/2 bg-gray-700 rounded-md shadow-lg flex items-center justify-center">
        {isLoading ? (
          <div className="animate-pulse bg-gray-600 rounded-md w-3/4 h-3/4 flex items-center justify-center">
            <span className="text-lg text-white">Loading...</span>
          </div>
        ) : (
          <img
            src={
              thumbnail === "" ? `src/assets/placeholder.jpg` : `${thumbnail}`
            } // Replace with your actual image URL
            alt="Loaded"
            className="w-full h-full object-cover rounded-md"
          />
        )}
      </div>

      <div className="flex p-5 max-w-lg w-full text-white">
        <Input
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Enter Video URL"
        />
        <Button
          type="submit"
          onClick={() => {
            getMetadata();
          }}
          className="bg-gray-500 transition duration-300 ease-in-out  hover:bg-slate-600 text-white p-1 pl-2 pr-2 ml-2 rounded"
        >
          Download
        </Button>
      </div>
    </main>
  );
}

export default Downloader;
