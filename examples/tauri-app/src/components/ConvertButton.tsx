import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ConvertButtonProps {
  file: File | null;
  fromFormat: string;
  toFormat: string;
}

const ConvertButton: React.FC<ConvertButtonProps> = ({ file, fromFormat, toFormat }) => {
  const [isLoading, setIsLoading] = useState(false);
  const [convertedData, setConvertedData] = useState<Uint8Array | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleConvert = async () => {
    if (!file) return;

    setIsLoading(true);
    setConvertedData(null);
    setError(null);

    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      const result: number[] = await invoke("convert_files", {
        fileBytes: Array.from(uint8Array),
        fromFormat,
        toFormat,
      });

      setConvertedData(new Uint8Array(result));
    } catch (err: any) {
      setError(err?.toString() ?? "Unknown error occurred during conversion.");
    } finally {
      setIsLoading(false);
    }
  };

const handleDownload = () => {
  if (!convertedData || !file) return;

  // Get the original filename without extension
  const originalName = file.name;
  const dotIndex = originalName.lastIndexOf('.');
  const baseName = dotIndex !== -1 ? originalName.substring(0, dotIndex) : originalName;

  // Create new filename with desired extension
  const newFileName = `${baseName}.${toFormat.toLowerCase()}`;

  const blob = new Blob([convertedData], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = newFileName;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
};

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
      <button
        onClick={handleConvert}
        disabled={!file || isLoading}
        style={{
          padding: '0.5rem 1rem',
          borderRadius: '0.375rem',
          fontWeight: 500,
          fontSize: '1rem',
          color: 'white',
          backgroundColor: !file || isLoading ? '#9ca3af' : '#16a34a',
          cursor: !file || isLoading ? 'not-allowed' : 'pointer',
          border: 'none',
        }}
        onMouseOver={(e) => {
          if (!file || isLoading) return;
          e.currentTarget.style.backgroundColor = '#15803d';
        }}
        onMouseOut={(e) => {
          if (!file || isLoading) return;
          e.currentTarget.style.backgroundColor = '#16a34a';
        }}
      >
        Convert
      </button>

      <div style={{ minHeight: '2.5rem', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
        {isLoading && <p style={{ color: '#ca8a04' }}>Loading...</p>}
        {error && <p style={{ color: '#dc2626' }}>Error: {error}</p>}
        {convertedData && !isLoading && !error && (
          <button
            onClick={handleDownload}
            style={{
              padding: '0.4rem 0.75rem',
              backgroundColor: '#2563eb',
              color: 'white',
              borderRadius: '0.375rem',
              cursor: 'pointer',
              border: 'none',
            }}
            onMouseOver={(e) => (e.currentTarget.style.backgroundColor = '#1d4ed8')}
            onMouseOut={(e) => (e.currentTarget.style.backgroundColor = '#2563eb')}
          >
            Download Converted File
          </button>
        )}
      </div>
    </div>
  );
};

export default ConvertButton;
