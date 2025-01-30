use noto_sans_mono_bitmap::
{get_raster_width, FontWeight, RasterHeight};

pub mod font_constants {
    use super::*;

    /// Height of each character raster. The font size is ~0.84% of this. 
    /// This is the line height that enables multiple characters to be side-by-side 
    /// and appear optically in one line in a natural way.
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    /// The width of each single symbol of the monospace font.
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    /// Backup character if a desired symbol is not available by the font.
    /// The `'\u{FFFD}'` is the Unicode replacement character.
    pub const BACKUP_CHAR: char = '\u{FFFD}';

    /// The font weight to be used.
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

    /// The backspace character.
    pub const BACKSPACE: char = '\u{0008}';
}
