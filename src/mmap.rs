use std::os::fd::AsRawFd;

#[cfg(target_os = "linux")]
pub fn new(file: std::fs::File) -> Result<&'static [u8], std::io::Error> {
    let len = file.metadata()?.len();

    let mapped = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            len as usize,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            file.as_raw_fd(),
            0,
        )
    };

    if mapped == libc::MAP_FAILED {
        return Err(std::io::Error::last_os_error());
    }

    Ok(unsafe { std::slice::from_raw_parts(mapped as *const u8, len as usize) })
}

#[cfg(not(target_os = "linux"))]
pub fn new(file: std::fs::File) -> Result<&'static [u8], std::io::Error> {
    use std::io::Read;

    let mut data = Vec::new();
    file.take(u64::MAX).read_to_end(&mut data)?;

    Ok(Box::leak(data.into_boxed_slice()))
}

pub fn new_path(path: &str) -> Result<&'static [u8], std::io::Error> {
    let file = std::fs::File::open(path)?;
    new(file)
}
