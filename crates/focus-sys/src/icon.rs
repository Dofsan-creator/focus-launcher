//! Icône haute définition d'un fichier, telle que l'Explorateur l'affiche.
//! Le shell de Windows lit les ressources du fichier : il n'est jamais exécuté.

use crate::wide::wide;
use std::path::Path;
use windows::core::PCWSTR;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, BITMAP, BITMAPINFO,
    BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HGDIOBJ,
};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Shell::{
    IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_BIGGERSIZEOK, SIIGBF_ICONONLY,
};

/// Image RGBA, alpha non prémultiplié.
#[derive(Debug, Clone)]
pub struct Rgba {
    /// Largeur.
    pub width: u32,
    /// Hauteur.
    pub height: u32,
    /// Pixels, 4 octets chacun.
    pub pixels: Vec<u8>,
}

/// Icône d'un exécutable, d'un raccourci ou de tout autre fichier.
pub fn file_icon(path: &Path, size: i32) -> Result<Rgba, windows::core::Error> {
    // Le shell refuse les chemins qui mélangent / et \ (chemins venus du registre).
    let normalized = path.to_string_lossy().replace('/', "\\");
    let path = wide(&normalized);
    // SAFETY: le tampon vit jusqu'à la fin ; l'image est libérée dans tous les cas.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let factory: IShellItemImageFactory =
            SHCreateItemFromParsingName(PCWSTR(path.as_ptr()), None)?;
        let bitmap = factory.GetImage(
            SIZE { cx: size, cy: size },
            SIIGBF_ICONONLY | SIIGBF_BIGGERSIZEOK,
        )?;
        let result = read_bitmap(bitmap);
        let _ = DeleteObject(HGDIOBJ(bitmap.0));
        result
    }
}

/// Copie les pixels d'une image GDI 32 bits.
///
/// # Safety
/// `bitmap` doit être une image GDI valide.
unsafe fn read_bitmap(bitmap: HBITMAP) -> Result<Rgba, windows::core::Error> {
    // SAFETY: l'appelant garantit une image GDI valide ; les tampons sont locaux.
    unsafe {
        let mut info = BITMAP::default();
        let read = GetObjectW(
            HGDIOBJ(bitmap.0),
            std::mem::size_of::<BITMAP>() as i32,
            Some((&mut info as *mut BITMAP).cast()),
        );
        if read == 0 || info.bmWidth <= 0 || info.bmHeight == 0 {
            return Err(windows::core::Error::from_win32());
        }
        let (width, height) = (info.bmWidth, info.bmHeight.abs());
        let mut header = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                // Hauteur négative : lignes de haut en bas.
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut pixels = vec![0_u8; (width as usize) * (height as usize) * 4];
        let dc = CreateCompatibleDC(None);
        let lines = GetDIBits(
            dc,
            bitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr().cast()),
            &mut header,
            DIB_RGB_COLORS,
        );
        let _ = DeleteDC(dc);
        if lines == 0 {
            return Err(windows::core::Error::from_win32());
        }
        unpremultiply(&mut pixels);
        Ok(Rgba {
            width: width as u32,
            height: height as u32,
            pixels,
        })
    }
}

/// BGRA prémultiplié (GDI) vers RGBA droit. Une image sans aucun alpha est opaque.
fn unpremultiply(pixels: &mut [u8]) {
    let no_alpha = pixels.as_chunks::<4>().0.iter().all(|pixel| pixel[3] == 0);
    for pixel in pixels.as_chunks_mut::<4>().0 {
        pixel.swap(0, 2);
        let alpha = u32::from(pixel[3]);
        if no_alpha {
            pixel[3] = 255;
        } else if alpha > 0 && alpha < 255 {
            for channel in &mut pixel[..3] {
                *channel = ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premultiplied_pixels_should_become_straight_rgba() {
        // Bleu prémultiplié à moitié transparent, en BGRA.
        let mut pixels = vec![64, 0, 0, 128];
        unpremultiply(&mut pixels);
        assert_eq!(pixels, vec![0, 0, 128, 128]);
    }

    #[test]
    fn image_without_alpha_should_be_opaque() {
        let mut pixels = vec![10, 20, 30, 0, 40, 50, 60, 0];
        unpremultiply(&mut pixels);
        assert_eq!(pixels, vec![30, 20, 10, 255, 60, 50, 40, 255]);
    }

    #[test]
    fn system_file_should_give_a_square_icon() -> Result<(), Box<dyn std::error::Error>> {
        let notepad = Path::new(r"C:\Windows\System32\notepad.exe");
        if !notepad.is_file() {
            return Ok(());
        }
        let icon = file_icon(notepad, 256)?;
        assert!(icon.width >= 32 && icon.width == icon.height);
        assert_eq!(icon.pixels.len(), (icon.width * icon.height * 4) as usize);
        Ok(())
    }
}
