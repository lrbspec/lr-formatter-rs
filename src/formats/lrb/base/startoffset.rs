use crate::formats::{
    internal::Vec2,
    lrb::{ModHandler, mod_flags},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;

// X: f64 = the X coordinate of the start offset
// Y: f64 = the Y coordinate of the start offset (remember +Y is down)

pub(in crate::formats::lrb) static STARTOFFSET: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA | mod_flags::PHYSICS,
    read: Box::new(|cursor, output| {
        let x = cursor.read_f64::<LittleEndian>()?;
        let y = cursor.read_f64::<LittleEndian>()?;
        output.start_position = Vec2 { x, y };

        Ok(())
    }),
    write: Box::new(|buffer, internal| {
        buffer.write_f64::<LittleEndian>(internal.start_position.x)?;
        buffer.write_f64::<LittleEndian>(internal.start_position.y)?;
        Ok(())
    }),
});
