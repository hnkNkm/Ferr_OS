use lazy_static::lazy_static;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use x86_64::registers::segmentation::CS;
use x86_64::instructions::segmentation::Segment;

/// ダブルフォルト用 IST のインデックス
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // ダブルフォルト用スタックを確保
        static mut STACK: [u8; 4096 * 5] = [0; 4096 * 5];
        unsafe {
            let stack_start = VirtAddr::from_ptr(STACK.as_ptr() as *const u8);
            let stack_end = stack_start + STACK.len();
            tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;
        }
        tss
    };

    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
} 