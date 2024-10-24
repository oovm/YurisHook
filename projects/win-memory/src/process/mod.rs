mod handle;
mod module;
mod tlhelp32;

use crate::MemoryError;
use handle::Handle;
pub use module::{Module, Signature};
use std::{ffi::c_void, mem::size_of, os::windows::process::CommandExt, process::Command};
use tlhelp32::*;
use windows::Win32::{
    Foundation::BOOL,
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS},
        },
        Memory::{PAGE_PROTECTION_FLAGS, VirtualProtect},
        Threading::{CREATE_NO_WINDOW, IsWow64Process},
    },
};

#[derive(Debug, Clone)]

/// contains name, pid and handle of a process
pub struct WindowsProcess {
    /// name of the process
    pub name: String,
    /// unique identifier if the process
    pub pid: u32,
    /// used when desired data is not inside a loaded module
    pub base_address: usize,
    /// either PROCESS_ALL_ACCESS or PROCESS_VM_READ | PROCESS_VM_WRITE
    pub handle: Handle,
    /// is x32 or x64
    pub is_wow64: bool,
}

unsafe impl Send for WindowsProcess {}
unsafe impl Sync for WindowsProcess {}

impl WindowsProcess {
    /// returns the desired process with the provided pid
    ///
    /// ```rust
    /// use win_memory::{MemoryError, WindowsProcess};
    /// let process: Result<WindowsProcess, MemoryError> = WindowsProcess::with_pid(12345);
    /// ```
    pub fn with_pid(pid: u32) -> Result<Self, MemoryError> {
        let h_snap = create_snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut pe32 = create_pe32();
        process_first(&h_snap, &mut pe32)?;

        loop {
            if pid.eq(&pe32.th32ProcessID) {
                let process_name = String::from_utf16_lossy(&pe32.szExeFile).trim_end_matches('\u{0}').to_string();

                let mut proc = WindowsProcess {
                    name: String::from(&process_name),
                    pid: pid,
                    base_address: 0,
                    handle: unsafe { Handle::read_write(pid)? },
                    is_wow64: false,
                };

                proc.base_address = proc.module(&process_name)?.base_address();
                proc.is_wow64 = proc.is_wow64();

                return Ok(proc);
            }

            if !process_next(&h_snap, &mut pe32) {
                break;
            }
        }
        Err(MemoryError::ProcessNotFound)
    }

    /// returns the desired process with the provided name
    ///
    /// ```rust
    /// use win_memory::{MemoryError, WindowsProcess};
    /// let process: Result<WindowsProcess, MemoryError> = WindowsProcess::with_name("process.exe");
    /// ```
    pub fn with_name(name: &str) -> Result<Self, MemoryError> {
        let h_snap = create_snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut pe32 = create_pe32();
        process_first(&h_snap, &mut pe32)?;

        loop {
            let process_name = String::from_utf16_lossy(&pe32.szExeFile).trim_end_matches('\u{0}').to_string();
            if process_name.eq(&name) {
                let mut proc = WindowsProcess {
                    name: String::from(&process_name),
                    pid: pe32.th32ProcessID,
                    base_address: 0,
                    handle: unsafe { Handle::read_write(pe32.th32ProcessID)? },
                    is_wow64: false,
                };

                proc.base_address = proc.module(&process_name)?.base_address();
                proc.is_wow64 = proc.is_wow64();

                return Ok(proc);
            }

            if !process_next(&h_snap, &mut pe32) {
                break;
            }
        }
        Err(MemoryError::ProcessNotFound)
    }

    /// returns a Vec<Process> where all processes share the provided name
    ///
    /// ```rust
    /// use win_memory::{MemoryError, WindowsProcess};
    /// let processes: Result<Vec<WindowsProcess>, MemoryError> =
    ///     WindowsProcess::all_with_name("process.exe");
    /// ```
    pub fn all_with_name(name: &str) -> Result<Vec<WindowsProcess>, MemoryError> {
        let mut results: Vec<WindowsProcess> = Vec::new();
        let h_snap = create_snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut pe32 = create_pe32();
        process_first(&h_snap, &mut pe32)?;

        loop {
            let process_name = String::from_utf16_lossy(&pe32.szExeFile).trim_end_matches('\u{0}').to_string();
            if process_name.eq(&name) {
                let mut proc = WindowsProcess {
                    name: String::from(&process_name),
                    pid: pe32.th32ProcessID,
                    base_address: 0,
                    handle: unsafe { Handle::read_write(pe32.th32ProcessID)? },
                    is_wow64: false,
                };

                proc.base_address = proc.module(&process_name)?.base_address();
                proc.is_wow64 = proc.is_wow64();

                results.push(proc);
            }

            if !process_next(&h_snap, &mut pe32) {
                break;
            }
        }

        match results.is_empty() {
            true => return Err(MemoryError::ProcessNotFound),
            false => return Ok(results),
        }
    }

    /// returns an instance of module including its base address in memory
    ///
    /// ```rust
    /// use win_memory::{MemoryError, Module, WindowsProcess};
    /// let process = WindowsProcess::with_name("process.exe")?;
    /// let module: Result<Module, MemoryError> = process.module("module.dll");
    /// ```
    pub fn module(&self, name: &str) -> Result<Module, MemoryError> {
        let h_snap = create_snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, *self.pid())?;

        let mut me32 = create_me32();
        module_first(&h_snap, &mut me32)?;

        loop {
            let module_name = String::from_utf16_lossy(&me32.szModule).trim_end_matches('\u{0}').to_string();

            if module_name.eq(name) {
                let module_path = String::from_utf16_lossy(&me32.szExePath).trim_end_matches('\u{0}').to_string();
                return Ok(Module::new(
                    module_name,
                    module_path,
                    *self.pid(),
                    me32.modBaseAddr as usize,
                    me32.modBaseSize as usize,
                    &self,
                ));
            }

            if !module_next(&h_snap, &mut me32) {
                break;
            }
        }
        Err(MemoryError::ModuleNotFound)
    }

    /// returns true if the process was terminated, otherwise will return false
    ///
    /// ```rust
    /// use win_memory::WindowsProcess;
    /// let process = WindowsProcess::with_name("process.exe")?;
    /// let did_terminate: bool = process.kill();
    /// ```
    pub fn kill(&self) -> bool {
        let output = Command::new("taskkill.exe")
            .arg("/PID")
            .arg(&self.pid.to_string())
            .arg("/F")
            .creation_flags(CREATE_NO_WINDOW.0)
            .output()
            .expect("");

        if output.status.success() {
            println!("Process with PID {} was terminated", &self.pid);
            true
        }
        else {
            println!("Error killing process with PID {}: {}", &self.pid, String::from_utf8_lossy(&output.stderr));
            false
        }
    }

    /// This function takes a type and the address to read.
    /// On success the read value will be returned.
    /// ```rust
    /// use win_memory::{MemoryError, Module, WindowsProcess};
    /// let chrome = WindowsProcess::with_name("chrome.exe")?;
    /// let module = chrome.module("kernel32.dll")?;
    /// let read_value: Result<T, MemoryError> =
    ///     chrome.read_data_absolute::<T>(module.base_address() + 0x1337);
    /// ```
    pub fn read_data_absolute<T: Default>(&self, offset: usize) -> Result<T, MemoryError> {
        let mut out: T = Default::default();
        unsafe {
            ReadProcessMemory(
                self.handle.wrap,
                offset as *const _,
                &mut out as *mut T as *mut _,
                std::mem::size_of::<T>(),
                None,
            )?;
        }
        Ok(out)
    }

    pub fn read_data<T: Default>(&self, offset: usize) -> Result<T, MemoryError> {
        self.read_data_absolute(self.base_address + offset)
    }

    /// This function takes a type and a Vec of addresses/offsets,
    /// the first entry being the base address to start from.
    /// On success the read value will be returned.
    /// ```rust
    /// use win_memory::{MemoryError, Module, WindowsProcess};
    /// let chrome = WindowsProcess::with_name("chrome.exe")?;
    /// let module = chrome.module("kernel32.dll")?;
    /// let chain: Vec<usize> = vec![module.base_address(), 0xDEA964, 0x100];
    /// let read_value: Result<T, MemoryError> = chrome.read_mem_chain::<T>(chain);
    /// ```
    pub fn read_mem_chain<T: Default>(&self, mut chain: Vec<usize>) -> Result<T, MemoryError> {
        let mut address = chain.remove(0);

        while chain.len() != 1 {
            address += chain.remove(0);
            address = if self.is_wow64 {
                self.read_data_absolute::<u32>(address)? as usize
            }
            else {
                self.read_data_absolute::<u64>(address)? as usize
            }
        }

        let ret = self.read_data_absolute::<T>(address + chain.remove(0))?;

        return Ok(ret);
    }

    /// This function takes a type and a Vec of addresses/offsets,
    /// the first entry being the base address to start from.
    /// On success the address at the end of the chain will be returned.
    /// ```rust
    /// use win_memory::{MemoryError, Module, WindowsProcess};
    /// let some_game = WindowsProcess::with_name("some_game.exe")?;
    /// let module = some_game.module("client.dll")?;
    /// let chain: Vec<usize> = vec![module.base_address(), 0xDEA964, 0x100];
    /// let desired_address: Result<usize, MemoryError> = chrome.read_ptr_chain(chain);
    /// ```
    pub fn read_ptr_chain(&self, mut chain: Vec<usize>) -> Result<usize, MemoryError> {
        let mut address = chain.remove(0);

        while chain.len() != 1 {
            address += chain.remove(0);
            address = if self.is_wow64 {
                self.read_data_absolute::<u32>(address)? as usize
            }
            else {
                self.read_data_absolute::<u64>(address)? as usize
            }
        }

        return Ok(address + chain.remove(0));
    }

    /// This function takes a type and the address to write to.
    /// The returned boolean will be true on success and false on failure
    /// ```rust
    /// use win_memory::{Module, WindowsProcess};
    /// let chrome = WindowsProcess::with_name("chrome.exe")?;
    /// let module = chrome.module("kernel32.dll")?;
    /// let mut value_to_write: i32 = 1337;
    /// let write_result: bool = chrome.write_data(module.base_address() + 0x1337, value_to_write);
    /// ```
    pub fn write_data<T: Default>(&self, address: usize, mut value: T) -> bool {
        unsafe {
            WriteProcessMemory(
                self.handle.wrap,
                address as *mut _,
                &mut value as *mut T as *mut _,
                std::mem::size_of::<T>(),
                None,
            )
            .is_ok()
        }
    }

    /// With this function someone can write multiple bytes to a specified address.
    /// The returned boolean will be true on success and false on failure
    /// ```rust
    /// use win_memory::{Module, WindowsProcess};
    /// let chrome = WindowsProcess::with_name("chrome.exe")?;
    /// let module = chrome.module("kernel32.dll")?;
    /// let mut bytes_to_write: Vec<u8> = [0x48, 0xC7, 0xC0, 0x0A, 0x00, 0x00, 0x00].to_vec();
    /// let write_result: bool = chrome.write_bytes(
    ///     module.base_address() + 0x1337,
    ///     bytes_to_write.as_mut_ptr(),
    ///     bytes_to_write.len(),
    /// );
    /// ```
    pub fn write_bytes(&self, address: usize, buf: *mut u8, size: usize) -> bool {
        unsafe { WriteProcessMemory(self.handle.wrap, address as *mut _, buf as *mut _, size, None).is_ok() }
    }

    /// C style method to read memory
    /// Third argument is the multiplicator of the Size of "T"
    /// for example if someone would want to read multiple bytes
    /// ```rust
    /// use win_memory::{Module, WindowsProcess};
    /// let chrome = WindowsProcess::with_name("chrome.exe")?;
    /// let module = chrome.module("kernel32.dll")?;
    /// let mut value_buffer: i32 = 0;
    /// if !chrome.read_ptr(&mut value_buffer, module.base_address() + 0x1337, None) {
    ///     println!("ReadMemory Failure");
    /// }
    /// else {
    ///     println!("ReadMemory Success");
    /// }
    /// ```
    pub fn read_ptr<T: Copy>(&self, buf: *mut T, address: usize) -> bool {
        unsafe {
            ReadProcessMemory(self.handle.wrap, address as *const c_void, buf as *mut c_void, std::mem::size_of::<T>(), None)
                .is_ok()
        }
    }

    /// C style method to read multiple bytes from memory
    /// ```rust
    /// use win_memory::{Module, WindowsProcess};
    /// let chrome = WindowsProcess::with_name("chrome.exe")?;
    /// let module = chrome.module("kernel32.dll")?;
    ///
    /// let rsize = 10;
    /// let mut bytes_buffer: Vec<u8> = vec![0u8; rsize];
    /// if !chrome.read_bytes(module.base_address() + 0x1337, bytes_buffer.as_mut_ptr(), rsize) {
    ///     println!("ReadMemory Failure");
    /// }
    /// else {
    ///     println!("ReadMemory Success");
    /// }
    /// ```
    pub fn read_bytes(&self, address: usize, buf: *mut u8, size: usize) -> bool {
        unsafe { ReadProcessMemory(self.handle.wrap, address as *const c_void, buf as *mut c_void, size, None).is_ok() }
    }

    /// Returns a string slice of the process name
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the unique identifier aka. process id of the process
    pub fn pid(&self) -> &u32 {
        &self.pid
    }
    // Determines whether the specified process is running under WOW64 or an Intel64 of x64 processor.
    pub fn is_wow64(&self) -> bool {
        let mut tmp = BOOL::from(false);
        unsafe {
            match IsWow64Process(self.handle.wrap, &mut tmp) {
                Ok(_) => tmp.0.ne(&0),
                Err(_) => false,
            }
        }
    }

    /// Returns "TRUE" specified Memory Protection was changed successfully
    pub fn protect_mem(
        &self,
        address: usize,
        size: usize,
        new_protect: PAGE_PROTECTION_FLAGS,
        old_protect: &mut PAGE_PROTECTION_FLAGS,
    ) -> bool {
        unsafe { VirtualProtect(address as *mut c_void, size, new_protect, old_protect).is_ok() }
    }

    fn read_module(&self, address: usize, msize: usize) -> Result<Vec<u8>, MemoryError> {
        let mut out = vec![0u8; msize];
        let out_ptr = out.as_mut_ptr();
        unsafe {
            if ReadProcessMemory(
                self.handle.wrap,
                address as *const c_void,
                out_ptr as *mut c_void,
                size_of::<u8>() * msize,
                None,
            )
            .is_err()
            {
                Err(MemoryError::ReadMemoryError)
            }
            else {
                Ok(out)
            }
        }
    }
}
