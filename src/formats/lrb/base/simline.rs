use crate::{
    formats::{Line, LineType, SimulationLine, lrb::ModHandler},
    join_flags,
};
use anyhow::Context;
use bitflags::bitflags;
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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SimLineFlags: u8 {
        const RED = 1 << 0;
        const INVERTED = 1 << 1;
        const LEFT_EXTENSION = 1 << 2;
        const RIGHT_EXTENSION = 1 << 3;
    }
}

pub static SIMLINE: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: join_flags!(EXTRA_DATA, PHYSICS, SCENERY),
    read: Box::new(|cursor, output| {
        let num_lines = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..num_lines {
            let id = cursor.read_u32::<LittleEndian>()?;
            let line_flags = SimLineFlags::from_bits(cursor.read_u8()?)
                .context("Read invalid simulation line flags!")?;
            let x1 = cursor.read_f64::<LittleEndian>()?;
            let y1 = cursor.read_f64::<LittleEndian>()?;
            let x2 = cursor.read_f64::<LittleEndian>()?;
            let y2 = cursor.read_f64::<LittleEndian>()?;
            let line_type = if line_flags.contains(SimLineFlags::RED) {
                LineType::RED
            } else {
                LineType::BLUE
            };
            let flipped = line_flags.contains(SimLineFlags::INVERTED);
            let left_extension = line_flags.contains(SimLineFlags::LEFT_EXTENSION);
            let right_extension = line_flags.contains(SimLineFlags::RIGHT_EXTENSION);
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
            let mut line_flags = SimLineFlags::empty();
            if simulation_line.base_line.line_type == LineType::RED {
                line_flags.insert(SimLineFlags::RED);
            }
            if simulation_line.flipped {
                line_flags.insert(SimLineFlags::INVERTED);
            }
            if simulation_line.left_extension {
                line_flags.insert(SimLineFlags::LEFT_EXTENSION);
            }
            if simulation_line.right_extension {
                line_flags.insert(SimLineFlags::RIGHT_EXTENSION);
            }

            buffer.write_u32::<LittleEndian>(simulation_line.base_line.id)?;
            buffer.write_u8(line_flags.bits())?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.x1)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.y1)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.x2)?;
            buffer.write_f64::<LittleEndian>(simulation_line.base_line.y2)?;
        }

        Ok(())
    }),
});
