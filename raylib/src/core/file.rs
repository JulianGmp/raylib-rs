//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use crate::core::RaylibHandle;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::vec;

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
        // Here we convert the Vec<String> into a vector of char pointers for C
        // This vector is later passed as a pointer to the C function, since it expects a char**
        // Note that we take some safe measures here:
        // - files_ptrs is initialized with one extra element, which remains a null pointer. While we do pass the length
        //   to the raylib function, so that it should not read beyond the vectors memory, we add that null pointer at
        //   the end to make it safe, or at least detectable, that the reader is exceeding our memory.
        // - each string in files is copied, since raylib's C interface expects non-const char pointers. While I trust
        //   raylib enough to not modify these strings, ensuring it with by copying is a trade off I am willing to make
        let mut files_ptrs =
            vec![std::ptr::null_mut() as *mut ::std::os::raw::c_char; files.len() + 1];

        for i in 0..files.len() {
            let cstring;
            unsafe {
                cstring = CString::from_vec_unchecked(files[i].as_bytes().to_vec());
            }
            files_ptrs[i] = cstring.as_ptr() as *mut ::std::os::raw::c_char;
        }

        let files_length: u32 = files.len().try_into().unwrap();
        let file_path_list = raylib_sys::FilePathList {
            capacity: files_length,
            count: files_length,
            paths: files_ptrs.as_mut_ptr(),
        };

        unsafe {
            ffi::UnloadDroppedFiles(file_path_list);
        }
    }
}
