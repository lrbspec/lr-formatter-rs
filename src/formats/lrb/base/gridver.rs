use crate::{
    formats::{GridVersion, lrb::ModHandler},
    join_flags,
};
use anyhow::bail;
use byteorder::{ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;

// grid version: u8 = the grid algorithm version used by the track

pub static GRIDVER: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: join_flags!(EXTRA_DATA, PHYSICS),
    read: Box::new(|cursor, output| {
        let grid_version_number = cursor.read_u8()?;
        let grid_version = match grid_version_number {
            0 => GridVersion::V6_2,
            1 => GridVersion::V6_1,
            2 => GridVersion::V6_0,
            other => bail!("Invalid grid version number: {}", other),
        };

        output.grid_version = grid_version;

        Ok(())
    }),
    write: Box::new(|buffer, input| {
        let version_number = match input.grid_version {
            GridVersion::V6_0 => 2,
            GridVersion::V6_1 => 1,
            GridVersion::V6_2 => 0,
        };

        buffer.write_u8(version_number)?;

        Ok(())
    }),
});
