use crate::Ctx;
use pdf_writer::{Filter, Name, Pdf, Ref};
use zpl_state_manager::DrawInstruction;

/// A ^GF bitmap stored as a 1-bit /ImageMask XObject.
///
/// `data` holds `height` rows of `bytes_per_row` bytes each, MSB first, where a set
/// bit means a black dot (ZPL semantics).
pub(crate) struct ImageMask {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl ImageMask {
    /// Build a mask from decoded ^GF data. A trailing partial row is zero-padded so
    /// no data is lost. Returns `None` when there is nothing to draw.
    pub fn from_rows(bytes_per_row: u32, data: &[u8]) -> Option<Self> {
        if bytes_per_row == 0 || data.is_empty() {
            return None;
        }
        let bpr = bytes_per_row as usize;
        let height = data.len().div_ceil(bpr);
        let mut rows = data.to_vec();
        rows.resize(height * bpr, 0);
        Some(Self { width: bytes_per_row * 8, height: height as u32, data: rows })
    }

    /// Writes the XObject; `compressed` is `crate::compress(&self.data)`.
    pub fn write(&self, pdf: &mut Pdf, alloc: &mut Ref, compressed: &[u8]) -> Ref {
        let id = alloc.bump();
        let mut image = pdf.image_xobject(id, compressed);
        image.filter(Filter::FlateDecode);
        image.width(self.width as i32);
        image.height(self.height as i32);
        image.image_mask(true);
        image.bits_per_component(1);
        // Default decode paints sample 0; invert so set bits (black dots) paint.
        image.decode([1.0, 0.0]);
        id
    }
}

pub(crate) fn render(ctx: &mut Ctx, instruction: &DrawInstruction) {
    let DrawInstruction::DrawGraphicField { x, y, bytes_per_row, data, .. } = instruction else {
        return;
    };
    let Some(mask) = ImageMask::from_rows(*bytes_per_row, data) else {
        return;
    };
    let (w, h) = (mask.width as f32, mask.height as f32);
    let name = format!("Im{}", ctx.images.len());
    ctx.images.push(mask);

    // Image space is the unit square with row 0 at the top (y = 1). The page CTM is
    // y-down, so flip: unit (0,1) -> (x, y) and (0,0) -> (x, y + h).
    ctx.content
        .save_state()
        .transform([w, 0.0, 0.0, -h, *x as f32, *y as f32 + h])
        .set_fill_gray(0.0)
        .x_object(Name(name.as_bytes()))
        .restore_state();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pads_partial_rows() {
        let m = ImageMask::from_rows(2, &[0xFF, 0x00, 0x80]).unwrap();
        assert_eq!((m.width, m.height), (16, 2));
        assert_eq!(m.data, vec![0xFF, 0x00, 0x80, 0x00]);
    }

    #[test]
    fn empty_is_none() {
        assert!(ImageMask::from_rows(0, &[1, 2]).is_none());
        assert!(ImageMask::from_rows(4, &[]).is_none());
    }

    #[test]
    fn writes_image_mask_xobject() {
        let m = ImageMask::from_rows(1, &[0xAA, 0x55]).unwrap();
        let mut pdf = Pdf::new();
        let mut alloc = Ref::new(10);
        let id = m.write(&mut pdf, &mut alloc, &crate::compress(&m.data));
        assert_eq!(id, Ref::new(10));
        let bytes = pdf.finish();
        let s = String::from_utf8_lossy(&bytes);
        assert!(s.contains("/ImageMask true"));
        assert!(s.contains("/BitsPerComponent 1"));
        assert!(s.contains("/Decode [1 0]"));
        assert!(s.contains("/FlateDecode"));
        assert!(s.contains("/Width 8"));
        assert!(s.contains("/Height 2"));
    }
}
