//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use crate::core::RaylibHandle;
use std::ffi::CStr;

impl RaylibHandle {
    /// Checks if a file has been dropped into the window.
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe { ffi::IsFileDropped() }
    }

    /// Gets dropped filenames.
    pub fn load_dropped_files(&self) -> Vec<String> {
        let mut v = Vec::new();
        unsafe {
            // JulianGmp: this changed with the upgrade to raylib 4.2.0, test this as it may be
            // broken to hell
            let dropfiles = ffi::LoadDroppedFiles();
            for i in 0..dropfiles.count {
                let filestr = CStr::from_ptr(*dropfiles.paths.offset(i as isize))
                    .to_str()
                    .unwrap();
                let file = String::from(filestr);
                v.push(file);
            }
        }
        v
    }

    /// Clears dropped files paths buffer.
    #[inline]
    pub fn unload_dropped_files(&mut self, files: &Vec<String>) {
        unsafe {
            // JulianGmp: this needs to be implemented differently, we need to take the String
            // vector and copy its data into a FilePathList for raylib
            panic!("Not implemented");
            // ffi::UnloadDroppedFiles();
        }
    }
}
