//! batch subsystem

use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use lazy_static::*;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;


#[repr(align(4096))]
/// The struct for kernel stack, which is just a fixed-size static byte array.
/// 
/// `#[repr(align(4096))]` forces the compiler to align the type to 4096=0x1000 bytes
/// in memory.
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
/// The struct for user stack, which is just a fixed-size static byte array.
/// 
/// `#[repr(align(4096))]` forces the compiler to align th type to 4096=0x1000 bytes
/// in memory.
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

/// The kernel stack. It is a global instance store in the bss segment.
static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

/// The user stack. It is a global instance store in the bss segment.
/// 
/// It stores the register state when context switch happens. 
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    /// Get the stack pointer. 
    /// 
    /// For now, we only need to save one TrapContext at a time, so we always push context
    /// at the bottom of the stack. This is actually the stack base.
    /// As the stack is growing down toward lower addresses, it is the highest address.
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    /// Push the context at the buttom of the stack. 
    /// 
    /// Note that it is above the `sp`, so `__restore` first `mv sp, a0` to make it
    /// point to the `TrapContext`, i.e., the stack top.
    /// 
    /// The return type has lifetime `'static`,
    /// which is a special lifetime: the lifetime of the entire process. The funtion 
    /// return a reference that remains valid until the entire process ends.
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    /// Get the stack base. 
    /// 
    /// We need an empty stack at the start of a new user program execution.
    /// As the stack is growing down toward lower addresses, it is the highest address.
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

/// Struct for APP_MANAGER.
/// 
/// Have the info about
/// - the total number of user application
/// - the current app running
/// - the starting address of each application and the end of the last one
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1]
}

lazy_static! {
    /// The global instance that keeps track of which app is currently running.
    /// 
    /// We want a global variable, but it also need to be safe, so we can't simply use `static mut`.
    ///
    /// ### `lazy_static!`
    /// > It ensures the value is initialized exactly once when first used, 
    /// and prevents reads from uninitialized memory.
    /// 
    /// Usaully, static variable need to be initialized at the compiling stage. However, 
    /// `APP_MANAGER`'s initialization depends on the runtime value `_num_app()`.
    /// By using the`lazy_static!` macro, the static variable will be initailized when it is first used.
    /// It also ensures every read happens after it’s initialized. 
    /// 
    /// ### `UPSafeCell`
    /// > It wraps the value with RefCell to enforce Rust’s borrowing rules at runtime, 
    /// preventing multiple mutable borrows, even on a uniprocessor.
    /// 
    /// `static mut` is allowed to be borrowed multiple times, which may lead to unpredictable behavior,
    /// as the Rust compiler does not enforce safety guarantees.
    /// To address this issue, we use the `UPSafeCell` container, which triggers a panic
    /// if multiple borrows occur.
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            unsafe extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

impl AppManager {
    /// Print the location where the application binary is stored in memory.
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    /// Copy the application binary to `0x80400000`
    /// 
    /// 1. Clear the memory, including instruction cache. Because we're changing the memory content where the CPU
    /// is pointing at, there will be inconsistency between memory and cache.
    /// 2. Find the location (address in memory) of the target application binary. 
    /// This information comes from external symbol in `link_app.S` and
    /// is store in the global instance `APP_MANAGER`.
    /// 3. Do the copy.
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            println!("All application completed!");
            crate::sbi::shutdown();
        }
        println!("[kernel] Loading app_{}", app_id);
        // clear icache
        unsafe { core::arch::asm!("fence.i"); }
        // clear app area
        unsafe {
            core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
            let app_src = core::slice::from_raw_parts(
                self.app_start[app_id] as *const u8,
                self.app_start[app_id + 1] - self.app_start[app_id],
            );
            let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);
        }
    }

    /// Get the current app index
    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    /// Increment the current app index
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print apps info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

/// run next app
/// 
/// To use `APP_MANAGER`, we get a mutable reference of it. 
/// And we have to drop it manually
/// 
/// Set the physical context for the user program as if we
/// are returning to it from a trap.
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually
    // and release the resources
    unsafe extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}