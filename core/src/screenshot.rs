//! EX-G-SE Screenshot Module
//!
//! Cross-platform screenshot capture with support for:
//! - Windows: Win32 API (BitBlt)
//! - macOS: Core Graphics
//! - Linux: External tools (gnome-screenshot, scrot, grim)

use anyhow::{Context, Result};
use chrono::Utc;
use image::GenericImageView;
use std::fs;
use std::path::PathBuf;

/// Screenshot information structure
#[derive(Debug, Clone)]
pub struct ScreenshotInfo {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub size: u64,
    pub timestamp: i64,
}

/// Capture a screenshot of the entire screen
pub fn capture_screenshot() -> Result<ScreenshotInfo> {
    let screenshots_dir = get_screenshots_dir()?;
    fs::create_dir_all(&screenshots_dir)?;

    let timestamp = Utc::now().timestamp();
    let filename = format!("screenshot_{}.png", timestamp);
    let path = screenshots_dir.join(&filename);

    #[cfg(target_os = "windows")]
    {
        capture_windows(&path)
    }

    #[cfg(target_os = "macos")]
    {
        capture_macos(&path)
    }

    #[cfg(target_os = "linux")]
    {
        capture_linux(&path)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        anyhow::bail!("Screenshot capture not supported on this platform")
    }
}

/// Capture the active window (currently focused window)
pub fn capture_active_window() -> Result<ScreenshotInfo> {
    let screenshots_dir = get_screenshots_dir()?;
    fs::create_dir_all(&screenshots_dir)?;

    let timestamp = Utc::now().timestamp();
    let filename = format!("window_{}.png", timestamp);
    let path = screenshots_dir.join(&filename);

    #[cfg(target_os = "windows")]
    {
        capture_active_window_windows(&path)
    }

    #[cfg(target_os = "macos")]
    {
        capture_active_window_macos(&path)
    }

    #[cfg(target_os = "linux")]
    {
        capture_linux_active_window(&path)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        anyhow::bail!("Active window capture not supported on this platform")
    }
}

/// Capture IDE window (VSCode, Zed, RustRover, etc.)
pub fn capture_ide_window() -> Result<ScreenshotInfo> {
    // For now, this is an alias to capture_active_window
    // In the future, we could add IDE-specific window detection
    capture_active_window()
}

/// Get the screenshots directory path
fn get_screenshots_dir() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    path.push(".ex-g-se");
    path.push("screenshots");
    Ok(path)
}

// ============================================================================
// Windows Implementation
// ============================================================================

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::*;
    use anyhow::anyhow;
    use image::ImageBuffer;
    use std::ptr;
    use winapi::um::wingdi::{
        BitBlt, CreateCompatibleDC, DeleteDC, DeleteObject, GetDeviceCaps,
        SelectObject, CreateDIBSection,
        BI_RGB, DIB_RGB_COLORS, HORZRES, VERTRES,
    };
    use winapi::um::winuser::{GetDC, ReleaseDC, GetWindowDC, GetForegroundWindow};
    use winapi::shared::windef::RECT;

    /// Capture screenshot on Windows using Win32 API
    pub fn capture_windows(path: &PathBuf) -> Result<ScreenshotInfo> {
        unsafe {
            let hdc = GetDC(ptr::null_mut());
            if hdc.is_null() {
                return Err(anyhow!("Failed to get device context"));
            }

            let width = GetDeviceCaps(hdc, HORZRES);
            let height = GetDeviceCaps(hdc, VERTRES);

            let bmi = winapi::um::wingdi::BITMAPINFO {
                bmiHeader: winapi::um::wingdi::BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height, // Negative for top-down DIB
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [winapi::um::wingdi::RGBQUAD {
                    rgbBlue: 0,
                    rgbGreen: 0,
                    rgbRed: 0,
                    rgbReserved: 0,
                }],
            };

            let mut bits: *mut winapi::ctypes::c_void = ptr::null_mut();
            let hbmp = CreateDIBSection(
                hdc,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits as *mut _ as *mut _,
                ptr::null_mut(),
                0,
            );

            if hbmp.is_null() {
                ReleaseDC(ptr::null_mut(), hdc);
                return Err(anyhow!("Failed to create DIB section"));
            }

            let hdc_mem = CreateCompatibleDC(hdc);
            if hdc_mem.is_null() {
                DeleteObject(hbmp as *mut _);
                ReleaseDC(ptr::null_mut(), hdc);
                return Err(anyhow!("Failed to create compatible DC"));
            }

            let old_obj = SelectObject(hdc_mem, hbmp as *mut _);

            // Capture screen
            BitBlt(hdc_mem, 0, 0, width, height, hdc, 0, 0, 0x00CC0020); // SRCCOPY

            // Convert to image (handle both positive and negative height)
            let buffer_len = (width.abs() * height.abs() * 4) as usize;
            let mut buffer = vec![0u8; buffer_len];
            ptr::copy_nonoverlapping(bits as *const u8, buffer.as_mut_ptr(), buffer_len);

            use image::RgbaImage;
            // Use absolute values for width/height to handle negative height
            let abs_width = width.abs() as u32;
            let abs_height = height.abs() as u32;
            let img: RgbaImage = ImageBuffer::from_raw(abs_width, abs_height, buffer)
                .ok_or_else(|| anyhow!("Failed to create image buffer"))?;

            // Save as PNG
            img.save(path).context("Failed to save screenshot")?;

            // Cleanup
            SelectObject(hdc_mem, old_obj);
            DeleteDC(hdc_mem);
            DeleteObject(hbmp as *mut _);
            ReleaseDC(ptr::null_mut(), hdc);

            let metadata = fs::metadata(path)?;
            Ok(ScreenshotInfo {
                path: path.to_string_lossy().to_string(),
                width: width as u32,
                height: height as u32,
                size: metadata.len(),
                timestamp: Utc::now().timestamp(),
            })
        }
    }

    /// Capture active window on Windows
    pub fn capture_active_window_windows(path: &PathBuf) -> Result<ScreenshotInfo> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_null() {
                return capture_windows(path);
            }

            let hdc = GetWindowDC(hwnd);
            if hdc.is_null() {
                return capture_windows(path);
            }

            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            winapi::um::winuser::GetWindowRect(hwnd, &mut rect);

            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            if width <= 0 || height <= 0 {
                ReleaseDC(hwnd, hdc);
                return capture_windows(path);
            }

            let bmi = winapi::um::wingdi::BITMAPINFO {
                bmiHeader: winapi::um::wingdi::BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [winapi::um::wingdi::RGBQUAD {
                    rgbBlue: 0,
                    rgbGreen: 0,
                    rgbRed: 0,
                    rgbReserved: 0,
                }],
            };

            let mut bits: *mut winapi::ctypes::c_void = ptr::null_mut();
            let hbmp = CreateDIBSection(
                hdc,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits as *mut _ as *mut _,
                ptr::null_mut(),
                0,
            );

            if hbmp.is_null() {
                ReleaseDC(hwnd, hdc);
                return Err(anyhow!("Failed to create DIB section for window"));
            }

            let hdc_mem = CreateCompatibleDC(hdc);
            if hdc_mem.is_null() {
                DeleteObject(hbmp as *mut _);
                ReleaseDC(hwnd, hdc);
                return Err(anyhow!("Failed to create compatible DC for window"));
            }

            let old_obj = SelectObject(hdc_mem, hbmp as *mut _);

            BitBlt(hdc_mem, 0, 0, width, height, hdc, 0, 0, 0x00CC0020);

            let buffer_len = (width.abs() * height.abs() * 4) as usize;
            let mut buffer = vec![0u8; buffer_len];
            ptr::copy_nonoverlapping(bits as *const u8, buffer.as_mut_ptr(), buffer_len);

            use image::RgbaImage;
            let img: RgbaImage = ImageBuffer::from_raw(width as u32, height as u32, buffer)
                .ok_or_else(|| anyhow!("Failed to create window image buffer"))?;

            img.save(path).context("Failed to save window screenshot")?;

            SelectObject(hdc_mem, old_obj);
            DeleteDC(hdc_mem);
            DeleteObject(hbmp as *mut _);
            ReleaseDC(hwnd, hdc);

            let metadata = fs::metadata(path)?;
            Ok(ScreenshotInfo {
                path: path.to_string_lossy().to_string(),
                width: width as u32,
                height: height as u32,
                size: metadata.len(),
                timestamp: Utc::now().timestamp(),
            })
        }
    }
}

#[cfg(target_os = "windows")]
use windows_impl::{capture_windows, capture_active_window_windows};

// ============================================================================
// macOS Implementation
// ============================================================================

#[cfg(target_os = "macos")]
mod macos_impl {
    use super::*;
    use anyhow::anyhow;
    use core_graphics::display::CGDisplay;
    use image::RgbaImage;

    /// Capture screenshot on macOS using Core Graphics
    pub fn capture_macos(path: &PathBuf) -> Result<ScreenshotInfo> {
        let display = CGDisplay::main();

        let width = display.pixels_wide();
        let height = display.pixels_high();

        let image = display.image()
            .ok_or_else(|| anyhow!("Failed to capture display image"))?;

        let bits_per_component = image.bits_per_component();
        let bits_per_pixel = image.bits_per_pixel();

        // Calculate bytes per pixel (bits_per_pixel / 8)
        let bytes_per_pixel = (bits_per_pixel / 8) as usize;

        if bits_per_component != 8 || bytes_per_pixel != 4 {
            return Err(anyhow!("Unsupported image format: {} bits/component, {} bytes/pixel",
                bits_per_component, bytes_per_pixel));
        }

        let data = image.data();

        let raw_data: Vec<u8> = data.bytes().to_vec();

        let width_u32: u32 = width.try_into()
            .map_err(|_| anyhow!("Width {} too large", width))?;
        let height_u32: u32 = height.try_into()
            .map_err(|_| anyhow!("Height {} too large", height))?;

        let img: RgbaImage = image::ImageBuffer::from_raw(width_u32, height_u32, raw_data)
            .ok_or_else(|| anyhow!("Failed to create image buffer"))?;

        img.save(path).context("Failed to save screenshot")?;

        let metadata = fs::metadata(path)?;
        Ok(ScreenshotInfo {
            path: path.to_string_lossy().to_string(),
            width: width_u32,
            height: height_u32,
            size: metadata.len(),
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Capture active window on macOS
    pub fn capture_active_window_macos(path: &PathBuf) -> Result<ScreenshotInfo> {
        // macOS active window capture requires more complex APIs
        // For now, fall back to full screen capture
        capture_macos(path)
    }
}

#[cfg(target_os = "macos")]
use macos_impl::{capture_macos, capture_active_window_macos};

// ============================================================================
// Linux Implementation
// ============================================================================

#[cfg(target_os = "linux")]
fn capture_linux(path: &PathBuf) -> Result<ScreenshotInfo> {
    use std::process::Command;

    // Try gnome-screenshot first
    if Command::new("gnome-screenshot")
        .args(&["-f", &path.to_string_lossy()])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return get_screenshot_info(path);
    }

    // Try scrot
    if Command::new("scrot")
        .arg(&path.to_string_lossy())
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return get_screenshot_info(path);
    }

    // Try grim (Wayland)
    if Command::new("grim")
        .arg(&path.to_string_lossy())
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return get_screenshot_info(path);
    }

    anyhow::bail!(
        "No screenshot tool found. Please install one of: gnome-screenshot, scrot, or grim"
    )
}

#[cfg(target_os = "linux")]
fn capture_linux_active_window(path: &PathBuf) -> Result<ScreenshotInfo> {
    use std::process::Command;

    // Try scrot with window focus
    if Command::new("scrot")
        .args(&["-u", &path.to_string_lossy()])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return get_screenshot_info(path);
    }

    // Try grim with active window
    if Command::new("grim")
        .args(&["-o", &path.to_string_lossy()])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return get_screenshot_info(path);
    }

    // Fallback to full screen
    capture_linux(path)
}

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
fn get_screenshot_info(path: &PathBuf) -> Result<ScreenshotInfo> {
    let img = image::open(path).context("Failed to open screenshot")?;
    let dimensions = img.dimensions();

    let metadata = fs::metadata(path)?;
    Ok(ScreenshotInfo {
        path: path.to_string_lossy().to_string(),
        width: dimensions.0,
        height: dimensions.1,
        size: metadata.len(),
        timestamp: Utc::now().timestamp(),
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_dir_creation() {
        let dir = get_screenshots_dir();
        assert!(dir.is_ok());
        let path = dir.unwrap();
        assert!(path.ends_with(".ex-g-se/screenshots"));
    }

    #[test]
    fn test_screenshot_info_struct() {
        let info = ScreenshotInfo {
            path: "/test/path.png".to_string(),
            width: 1920,
            height: 1080,
            size: 1024,
            timestamp: 1234567890,
        };

        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
        assert_eq!(info.size, 1024);
    }
}
