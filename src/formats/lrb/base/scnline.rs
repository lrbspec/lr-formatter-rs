use crate::{
    formats::lrb::{ModHandler, mod_flags},
    track_builder::{Line, LineType, SceneryLine},
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

pub(in crate::formats::lrb) static SCNLINE: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA | mod_flags::SCENERY,
    read: Box::new(|cursor, internal| {
        let num_lines = cursor.read_u32::<LittleEndian>()?;

        for _ in 0..num_lines {
            let id = cursor.read_u32::<LittleEndian>()?;
            let x1 = cursor.read_f64::<LittleEndian>()?;
            let y1 = cursor.read_f64::<LittleEndian>()?;
            let x2 = cursor.read_f64::<LittleEndian>()?;
            let y2 = cursor.read_f64::<LittleEndian>()?;

            internal.scenery_lines.push(SceneryLine {
                base_line: Line {
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    line_type: LineType::Scenery,
                },
                width: None,
            });
        }

        Ok(())
    }),
    write: Box::new(|cursor, internal| {
        cursor.write_u32::<LittleEndian>(internal.scenery_lines.len() as u32)?;
        for scenery_line in &internal.scenery_lines {
            cursor.write_u32::<LittleEndian>(scenery_line.base_line.id)?;
            cursor.write_f64::<LittleEndian>(scenery_line.base_line.x1)?;
            cursor.write_f64::<LittleEndian>(scenery_line.base_line.y1)?;
            cursor.write_f64::<LittleEndian>(scenery_line.base_line.x2)?;
            cursor.write_f64::<LittleEndian>(scenery_line.base_line.y2)?;
        }

        Ok(())
    }),
});
