const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

/// Execute system call service.
///
/// arguments:
/// - `id` the system call ID
/// - `args` the arguments of syscall service
///
/// return value:
/// - the return value of service
#[inline(always)]
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
        "ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id,
        );
    }
    ret
}

/// Exit application and pass return value to system
///
/// arguments:
/// - `xstate` the return value of application
///
/// return value:
/// - this exit code
///
/// **syscall ID: 93**
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

/// Write the data in memory buffer to file.
///
/// arguments:
/// - `fd` the file descriptor of file to be write
/// - `buffer` the slice of memory buffer
///
/// return value:
/// - the length of successfully write
///
/// **syscall ID: 64**
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
