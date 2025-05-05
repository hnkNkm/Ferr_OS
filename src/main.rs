#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::fmt::Write;
use uefi::prelude::*;

/// EFI エントリポイント
#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // UEFI ランタイムサービス初期化 (alloc / logger 等)
    if let Err(e) = uefi_services::init(&mut st) {
        // 初期化失敗時でもリターンコードは SUCCESS にする (UEFI 仕様)
        let _ = st.stderr().write_fmt(format_args!("uefi_services init error: {:?}\n", e));
    }

    // 画面をクリアして "Hello, world!" を表示
    let stdout = st.stdout();
    let _ = stdout.reset(false);
    let _ = stdout.write_str("Hello, world from my_os!\n");

    // カーネルとして制御を保持するため無限ループ
    loop {
        // 省電力待機 (optional)
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::asm!("hlt");
        }
    }
} 