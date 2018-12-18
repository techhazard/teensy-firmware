use core::arch::arm::__NOP;


#[repr(C,packed)]
pub struct Watchdog {
    /// Status and Control Register High
    stctrlh: u16,
    /// Status and Control Register Low
    stctrll: u16,
    /// Time-out Value Register High
    tovalh: u16,
    /// Time-out Value Register Low
    tovall: u16,
    /// Window Register High
    winh: u16,
    /// Window Register Low
    winl: u16,
    /// Refresh Register
    refresh: u16,
    /// Wtachdog Unlock Register
    unlock: u16,
    /// Timer Output Register High
    tmrouth: u16,
    /// Timer Output Register Low
    tmroutl: u16,
    /// Reset Count Register
    rstcnt: u16,
    /// Prescaler Register
    presc: u16,
}


impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x40052000 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            core::ptr::write_volatile(&mut self.unlock, 0xD928);
            __NOP();
            __NOP();
            let mut ctrl = core::ptr::read_volatile(&self.stctrlh);
            ctrl &= !(0x00000001);
            core::ptr::write_volatile(&mut self.stctrlh, ctrl);
        }
    }
}
