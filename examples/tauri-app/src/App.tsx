import { useEffect, useState } from "react";
import ConvertButton from "./components/ConvertButton";
import FileSelector from "./components/FileSelector";
import FormatConvertPrompt from "./components/FormatConvertPrompt";

function App() {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [formatFrom, setFormatFrom] = useState("TRK");
  const [formatTo, setFormatTo] = useState("JSON");
  const [solIndex, setSolIndex] = useState(0);
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [maxSolIndex, setMaxSolIndex] = useState(0);

  // Update formatFrom if file extension is known and changed
  useEffect(() => {
    const selectedExtension = selectedFile
      ? selectedFile.name.split(".").pop()?.toUpperCase() || null
      : null;
    if (selectedExtension && ["TRK", "SOL", "JSON", "LRB"].includes(selectedExtension)) {
      setFormatFrom(selectedExtension);
    }
  }, [selectedFile]);

  // Detect dark mode preference
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    setIsDarkMode(mq.matches);
    const handler = (e: MediaQueryListEvent) => setIsDarkMode(e.matches);
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  // Root styles (normal/dark mode)
  const rootStyle = {
    fontFamily: "Inter, Avenir, Helvetica, Arial, sans-serif",
    fontSize: 16,
    lineHeight: "24px",
    fontWeight: 400,
    color: isDarkMode ? "#f6f6f6" : "#0f0f0f",
    backgroundColor: isDarkMode ? "#2f2f2f" : "#f6f6f6",
    fontSynthesis: "none" as const,
    textRendering: "optimizeLegibility" as const,
    WebkitFontSmoothing: "antialiased" as const,
    MozOsxFontSmoothing: "grayscale" as const,
    WebkitTextSizeAdjust: "100%" as const,
    minHeight: "100vh",
    margin: 0,
  };

  const containerStyle = {
    margin: 0,
    paddingTop: "10vh",
    display: "flex",
    flexDirection: "column" as const,
    justifyContent: "center",
    textAlign: "center" as const,
    gap: 10,
    maxWidth: 480,
    marginLeft: "auto",
    marginRight: "auto",
  };

  useEffect(() => {
    document.body.style.margin = "0";
    document.body.style.padding = "0";
    document.body.style.height = "100%";
    document.documentElement.style.height = "100%";
  }, []);

  return (
    <main style={rootStyle}>
      <div style={containerStyle}>
        <FileSelector
          selectedFile={selectedFile}
          onFileSelect={setSelectedFile}
          setMaxSolIndex={setMaxSolIndex}
        />
        <FormatConvertPrompt
          fromFormat={formatFrom}
          setFromFormat={setFormatFrom}
          toFormat={formatTo}
          setToFormat={setFormatTo}
          solIndex={solIndex}
          setSolIndex={setSolIndex}
          maxSolIndex={maxSolIndex}
        />
        <ConvertButton
          file={selectedFile}
          fromFormat={formatFrom}
          toFormat={formatTo}
          solIndex={solIndex}
        />
      </div>
    </main>
  );
}

export default App;
