use crate::{
    formats::{SceneryLine, lrb::ModHandler},
    join_flags,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;

// count: u32 = the amount of lines written
// lines: scnline[count] = [
//   id: u32 = the line's ID
//   x1: f64 = the x position of the 1st point
//   y1: f64 = the y position of the 1st point
//   x2: f64 = the x position of the 2nd point
//   y2: f64 = the y position of the 2nd point
// ]

pub static SCNLINE: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: join_flags!(OPTIONAL, EXTRA_DATA, SCENERY),
    optional_message: Some("contains scenery lines"),
    read: Box::new(|cursor, output| {
        let num_lines = cursor.read_u32::<LittleEndian>()?;

        for _ in 0..num_lines {
            let id = cursor.read_u32::<LittleEndian>()?;
            let x1 = cursor.read_f64::<LittleEndian>()?;
            let y1 = cursor.read_f64::<LittleEndian>()?;
            let x2 = cursor.read_f64::<LittleEndian>()?;
            let y2 = cursor.read_f64::<LittleEndian>()?;

            output.scenery_lines.push(SceneryLine {
                base_line: crate::formats::Line {
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    line_type: crate::formats::LineType::GREEN,
                },
                width: None,
            });
        }

        Ok(())
    }),
    write: Box::new(|buffer, internal| {
        buffer.write_u32::<LittleEndian>(internal.scenery_lines.len() as u32)?;
        for scenery_line in &internal.scenery_lines {
            buffer.write_u32::<LittleEndian>(scenery_line.base_line.id)?;
            buffer.write_f64::<LittleEndian>(scenery_line.base_line.x1)?;
            buffer.write_f64::<LittleEndian>(scenery_line.base_line.y1)?;
            buffer.write_f64::<LittleEndian>(scenery_line.base_line.x2)?;
            buffer.write_f64::<LittleEndian>(scenery_line.base_line.y2)?;
        }

        Ok(())
    }),
});
