import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import placeholder from "@/assets/placeholder-thumbnail.png";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import { open } from "@tauri-apps/plugin-dialog";

import "./output.css";

function Downloader() {
  const [title, setTitle] = useState("");
  const [url, setUrl] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [thumbnail, setThumbnail] = useState("");

  async function getStuff() {
    const res = await invoke("get_ffmpeg_path", {});
    console.log(res);
  }
  useEffect(() => {
    getStuff();
  }, []);

  const setFilePath = async () => {
    try {
      const file = await open({
        multiple: false,
        directory: true,
      });
      return file;
    } catch (err) {
      console.error(err);
    }
  };
  const callDownload = async (file) => {
    try {
      const res = await invoke("get_video", { url, path: file });
      console.log(res);
    } catch (e) {
      toast.error(e, {
        position: "bottom-center",
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: "dark",
      });
    }
  };

  const handleSaveFile = async () => {
    const filePath = await setFilePath();
    if (filePath === "" || filePath === null || filePath === undefined) {
      toast.error("No directory selected!", {
        position: "bottom-center",
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: "dark",
      });
      return;
    }
    try {
      await getMetadata();
    } catch (e) {
      return;
    }

    await toast.promise(callDownload(filePath), {
      position: "bottom-center",
      pending: "Downloading video 🤔",
      success: "Download Done ! 👌",
      // error: 'Promise rejected 🤯'
    });
  };

  async function getMetadata() {
    if (url === "") {
      toast.error("URL is empty!", {
        position: "bottom-center",
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: "dark",
      });
      throw new Error("URL is empty!");
    }
    await toast.promise(callMetadata(), {
      position: "bottom-center",
      pending: "Downloading metadata 🤔",
      // success: "here you go 👌",
      // error: 'Promise rejected 🤯'
    });
  }

  async function callMetadata() {
    setIsLoading(true);
    var metadata;
    try {
      metadata = await invoke("get_title", { url });
    } catch (e) {
      toast.error("Invalid URL!", {
        position: "bottom-center",
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: "dark",
      });
      setIsLoading(false);
      return;
    }
    const [title, thumb] = metadata;
    setTitle(title);
    setThumbnail(thumb);

    console.log(thumb);
    setIsLoading(false);
  }

  useEffect(() => {
    setIsLoading(false);
  }, [title, thumbnail]);

  return (
    <main className="flex flex-col items-center justify-center">
      <ToastContainer
        position="bottom-center"
        autoClose={1000}
        hideProgressBar={false}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="dark"
      />
      <h1 className="text-4xl text-white p-10 text-center">
        {title ? `${title}` : `Super Simple Video Downloader`}
      </h1>

      {/* Image Placeholder Container */}
      <div className="max-w-128 max-h-128 w-1/2 h-1/2 bg-gray-700 rounded-md shadow-lg flex items-center justify-center">
        {isLoading ? (
          <img
            src={thumbnail === "" ? placeholder : `${thumbnail}`} // Replace with your actual image URL
            alt={thumbnail}
            className="w-full h-full object-cover rounded-md animate-bounce"
          />
        ) : (
          <img
            src={thumbnail === "" ? placeholder : `${thumbnail}`} // Replace with your actual image URL
            alt={thumbnail}
            className="w-full h-full object-cover rounded-md"
          />
        )}
      </div>

      <div className="flex p-5 max-w-lg w-full text-white">
        <Input
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Enter Video URL"
        />
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                data-tooltip-target="fetch-tooltip"
                type="submit"
                onClick={getMetadata}
                className="bg-gray-500 transition duration-300 ease-in-out  hover:bg-slate-600 text-white p-1 pl-2 pr-2 ml-2 rounded"
              >
                Fetch
              </Button>
            </TooltipTrigger>

            <TooltipContent>
              <p className="bg-gray-500 text-white z-10 rounded p-2">
                Get metadata from the video URL
              </p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>

        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                type="submit"
                onClick={handleSaveFile}
                className="bg-gray-500 transition duration-300 ease-in-out  hover:bg-slate-600 text-white p-1 pl-2 pr-2 ml-2 rounded"
              >
                Download
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p className="bg-gray-500 text-white z-10 rounded p-2">
                Download video
              </p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>
    </main>
  );
}

export default Downloader;
