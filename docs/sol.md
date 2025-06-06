# SOL Format

- Uses the [Action Message Format v0](https://rtmp.veriskope.com/pdf/amf0-file-format-specification.pdf) for the actual data
- Assume all values are BigEndian
- This format specification does not detail every flash mod, only Beta 2 v6.0 - v6.2 and LRA

# Header

- **0x00:** Magic number 0x00BF, version of SOL
- **0x02:** Unsigned 32 bit integer, number of bytes that follow
- **0x06:** Tag 0x5443534F, spelling out "TCSO"
- **0x0A:** Marker 0x000400000000
- **0x10:** Unsigned 16 bit integer, SOL name length (0x0A)
- **0x12:** SOL name "savedLines"
- **0x1C:** Padding 0x00000000
- **0x20:** Unsigned 16 bit integer, data name length (0x09)
- **0x22:** Data name "trackList"

# Data

- Default physics version is 6.0, which is before the "version" field was added
- `startLine` is just the [x, y] values of `startPosition`
- `level` is the total number of lines
- `data` is the list of track lines, as line arrays: [x1, y1, x2, y2, ext, flipped, prevLine, nextLine, id, type]
  - `flipped` is a number in the beta 2 writer, but a boolean in the LRA writer (and .com writer, which is disabled), so parse carefully
  - `ext` Has bit flags representing line extensions, 0: None, 1: Left, 2: Right, 3: Both
- `trackData` is a property added by LRA that (at present) only describes whether zero start is enabled

[
  {
    label: String,
    version?: String,
    startLine: [Number (f64), Number (f64)],
    level: Number (u32),
    data: [
      [
        Number (f64),
        Number (f64),
        Number (f64),
        Number (f64),
        Number (0 - 3),
        Number (0 - 1) | Boolean,
        Number (u32),
        Number (u32),
        Number (u32),
        Number (0 - 2)
      ],
      ...previous lines
    ],
    trackData: [
      null,
      [null, null, null],
      [null, null, null, null, true]
    ]
  },
  ...previous tracks
]
