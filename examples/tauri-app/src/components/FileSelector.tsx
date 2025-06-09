import React, { useRef } from 'react';

interface FileSelectorProps {
  onFileSelect: (file: File | null) => void;
  selectedFile: File | null;
}

const FileSelector: React.FC<FileSelectorProps> = ({ onFileSelect, selectedFile }) => {
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const handleButtonClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0] || null;
    onFileSelect(file);
  };

  return (
    <div style={{ margin: '0 auto', maxWidth: '28rem', display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
      <button
        onClick={handleButtonClick}
        style={{
          padding: '0.5rem 1rem',
          backgroundColor: '#2563eb',
          color: 'white',
          borderRadius: '0.375rem',
          border: 'none',
          fontWeight: 500,
          cursor: 'pointer',
        }}
        onMouseOver={(e) => (e.currentTarget.style.backgroundColor = '#1d4ed8')}
        onMouseOut={(e) => (e.currentTarget.style.backgroundColor = '#2563eb')}
      >
        Select Track File
      </button>

      <input
        type="file"
        accept=".trk,.json,.lrb,.sol"
        ref={fileInputRef}
        style={{ display: 'none' }}
        onChange={handleFileChange}
      />

      <div>
        {selectedFile ? (
          <p style={{ color: '#15803d', fontWeight: 500 }}>Selected: {selectedFile.name}</p>
        ) : (
          <p style={{ color: '#6b7280', fontStyle: 'italic' }}>No file selected</p>
        )}
      </div>
    </div>
  );
};

export default FileSelector;
