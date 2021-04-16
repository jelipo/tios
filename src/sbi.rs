pub fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret = 0;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile"
        );
    }
    ret
}

#[inline]
fn sbi_ext_call_asm(ext: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> (i8, usize) {
    let mut ret;
    let mut value;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret),"={x11}" (value)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2),"{x16}" (fid), "{x17}" (ext)
            : "memory"
            : "volatile"
        );
    }
    (ret, value)
}

#[derive(Debug)]
pub enum SbiError {
    SbiSuccess = 0,
    SbiErrFailed = -1,
    SbiErrNotSupported = -2,
    SbiErrInvalidParam = -3,
    SbiErrDenied = -4,
    SbiErrInvalidAddress = -5,
    SbiErrAlreadyAvailable = -6,
}


pub fn sbi_ext_call(ext: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> Result<usize, SbiError> {
    let (code, value) = sbi_ext_call_asm(ext, fid, arg0, arg1, arg2);
    match code {
        0 => Ok(value),
        -1 => Err(SbiError::SbiErrFailed),
        -2 => Err(SbiError::SbiErrNotSupported),
        -3 => Err(SbiError::SbiErrInvalidParam),
        -4 => Err(SbiError::SbiErrDenied),
        -5 => Err(SbiError::SbiErrInvalidAddress),
        -6 => Err(SbiError::SbiErrAlreadyAvailable),
        _ => Err(SbiError::SbiErrInvalidParam)
    }
}

// pub fn get_spec_version() -> Result<String, SbiError> {
//     let value = sbi_ext_call(0x10, 0, 0, 0, 0)?;
//     Ok(alloc::format!("{}", value))
// }
