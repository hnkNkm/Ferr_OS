#![no_std]
#![no_main]
#![feature(alloc_error_handler, abi_x86_interrupt)]

extern crate alloc;

use core::fmt::Write;
use uefi::prelude::*;
use uefi::table::boot::MemoryType;
mod interrupts;
mod serial;
mod gdt;

/// EFI エントリポイント
#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // UEFI ランタイムサービス初期化 (alloc / logger 等)
    if let Err(e) = uefi_services::init(&mut st) {
        // 初期化失敗時でもリターンコードは SUCCESS にする (UEFI 仕様)
        let _ = st.stderr().write_fmt(format_args!("uefi_services init error: {:?}\n", e));
    }

    {
        let stdout = st.stdout();
        let _ = stdout.reset(false);
        let _ = stdout.write_str("Hello, world from my_os! (before exit BS)\n");
    }

    // Exit UEFI Boot Services (consumes st)
    let (_st_rt, _memory_map) = unsafe { st.exit_boot_services(MemoryType::LOADER_DATA) };

    // ここから先は UEFI Boot Services なし

    // GDT / IDT
    gdt::init();
    interrupts::init();

    x86_64::instructions::interrupts::enable();

    // ハンドラ確認: 故意に int3
    unsafe { core::arch::asm!("int3"); }

    // カーネルとして制御を保持するため無限ループ
    loop {
        // 省電力待機 (optional)
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::asm!("hlt");
        }
    }
} 