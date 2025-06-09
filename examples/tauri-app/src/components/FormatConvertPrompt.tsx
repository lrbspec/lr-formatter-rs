import { useEffect, useState } from "react";

interface FormatConvertPromptProps {
  initialFormatFrom: string | null;
  onFormatFromChange: (format: string) => void;
  onFormatToChange: (format: string) => void;
}

const FormatConvertPrompt: React.FC<FormatConvertPromptProps> = ({
  initialFormatFrom,
  onFormatFromChange,
  onFormatToChange,
}) => {
  const [formatFrom, setFormatFrom] = useState("TRK");
  const [formatTo, setFormatTo] = useState("JSON");

  useEffect(() => {
    if (
      initialFormatFrom &&
      ["TRK", "SOL", "JSON", "LRB"].includes(initialFormatFrom)
    ) {
      setFormatFrom(initialFormatFrom);
      onFormatFromChange(initialFormatFrom);
    }
  }, [initialFormatFrom]);

  const handleFromChange = (value: string) => {
    setFormatFrom(value);
    onFormatFromChange(value);
  };

  const handleToChange = (value: string) => {
    setFormatTo(value);
    onFormatToChange(value);
  };

  const selectStyle: React.CSSProperties = {
    border: '1px solid #ccc',
    borderRadius: '0.375rem',
    padding: '0.25rem 0.5rem',
    fontSize: '1rem',
    fontFamily: 'inherit',
    outline: 'none',
    cursor: 'pointer',
  };

  const containerStyle: React.CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem',
    fontSize: '1rem',
  };

  return (
    <div style={containerStyle}>
      <span>Convert</span>

      <select
        value={formatFrom}
        onChange={(e) => handleFromChange(e.target.value)}
        style={selectStyle}
      >
        <option value="TRK">TRK</option>
        <option value="SOL">SOL</option>
        <option value="JSON">JSON</option>
        <option value="LRB">LRB</option>
      </select>

      <span>to</span>

      <select
        value={formatTo}
        onChange={(e) => handleToChange(e.target.value)}
        style={selectStyle}
      >
        <option value="SOL">SOL</option>
        <option value="JSON">JSON</option>
        <option value="LRB">LRB</option>
      </select>
    </div>
  );
};

export default FormatConvertPrompt;
