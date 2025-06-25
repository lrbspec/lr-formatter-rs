use crate::formats::{
    internal::{Line, LineType, SimulationLine},
    lrb::{ModHandler, mod_flags},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;

// count: u32 = the amount of lines written
// lines: simline[count] = [
//   id: u32 = the line's ID
//   flags: u8 = Line flags 0000DCBA
//   x1: f64 = the x position of the 1st point
//   y1: f64 = the y position of the 1st point
//   x2: f64 = the x position of the 2nd point
//   y2: f64 = the y position of the 2nd point
// ]
// Line flag defs: A = Red line, B = inverted, C = left extension, D = right extension

pub const RED: u8 = 1 << 0;
pub const INVERTED: u8 = 1 << 1;
pub const LEFT_EXTENSION: u8 = 1 << 2;
pub const RIGHT_EXTENSION: u8 = 1 << 3;

pub(in crate::formats::lrb) static SIMLINE: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA | mod_flags::PHYSICS | mod_flags::SCENERY,
    read: Box::new(|cursor, output| {
        let num_lines = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..num_lines {
            let id = cursor.read_u32::<LittleEndian>()?;
            let line_flags = cursor.read_u8()?;
            let x1 = cursor.read_f64::<LittleEndian>()?;
            let y1 = cursor.read_f64::<LittleEndian>()?;
            let x2 = cursor.read_f64::<LittleEndian>()?;
            let y2 = cursor.read_f64::<LittleEndian>()?;
            let line_type = if line_flags & RED != 0 {
                LineType::RED
            } else {
                LineType::BLUE
            };
            let flipped = line_flags & INVERTED != 0;
            let left_extension = line_flags & LEFT_EXTENSION != 0;
            let right_extension = line_flags & RIGHT_EXTENSION != 0;
            let base_line = Line {
                id,
                x1,
                y1,
                x2,
                y2,
                line_type,
            };
            output.simulation_lines.push(SimulationLine {
                base_line,
                flipped,
                left_extension,
                right_extension,
                multiplier: None,
            });
        }

        Ok(())
    }),
    write: Box::new(|buffer, internal| {
        buffer.write_u32::<LittleEndian>(internal.simulation_lines.len() as u32)?;
        for simulation_line in &internal.simulation_lines {
            let mut line_flags: u8 = 0;
            if simulation_line.base_line.line_type == LineType::RED {
                line_flags |= RED;
            }
            if simulation_line.flipped {
                line_flags |= INVERTED;
            }
            if simulation_line.left_extension {
                line_flags |= LEFT_EXTENSION;
            }
            if simulation_line.right_extension {
                line_flags |= RIGHT_EXTENSION;
            }

            buffer.write_u32::<LittleEndian>(simulation_line.base_line.id)?;
            buffer.write_u8(line_flags)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.x1)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.y1)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.x2)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.y2)?;
        }

        Ok(())
    }),
});
