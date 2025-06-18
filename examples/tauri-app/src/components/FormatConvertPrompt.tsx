interface FormatConvertPromptProps {
  fromFormat: string;
  setFromFormat: (format: string) => void;
  toFormat: string;
  setToFormat: (format: string) => void;
  solIndex: number;
  setSolIndex: (solIndex: number) => void;
  maxSolIndex: number;
}

const FormatConvertPrompt: React.FC<FormatConvertPromptProps> = ({
  fromFormat,
  setFromFormat,
  toFormat,
  setToFormat,
  solIndex,
  setSolIndex,
  maxSolIndex,
}) => {
  const selectStyle: React.CSSProperties = {
    border: "1px solid #ccc",
    borderRadius: "0.375rem",
    padding: "0.25rem 0.5rem",
    fontSize: "1rem",
    fontFamily: "inherit",
    outline: "none",
    cursor: "pointer",
  };

  const containerStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    gap: "0.5rem",
    fontSize: "1rem",
  };

  return (
    <div style={containerStyle}>
      <span>Convert</span>

      <select
        value={fromFormat}
        onChange={(e) => setFromFormat(e.target.value)}
        style={selectStyle}
      >
        <option value="TRK">TRK</option>
        <option value="SOL">SOL</option>
        <option value="JSON">JSON</option>
        <option value="LRB">LRB</option>
      </select>

      {fromFormat === "SOL" && (
        <>
          <span>track</span>
          <input
            type="number"
            min={0}
            max={maxSolIndex}
            step={1}
            value={solIndex}
            onChange={(e) => setSolIndex(parseInt(e.target.value))}
          >
          </input>
        </>
      )}

      <span>to</span>

      <select
        value={toFormat}
        onChange={(e) => setToFormat(e.target.value)}
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
