
#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}


#[repr(C, packed)]
pub struct Sim {
    /// System Options Register 1
	sopt1: u32,
    /// System Option Register 1 configuration
    sopt1_cfg: u32,
    _pad0: [u32; 1023],
    /// System Options Register 2
	sopt2: u32,
    _pad1: u32,
    /// System Options Register 4
	sopt4: u32,
    /// System Options Register 5
	sopt5: u32,
    _pad2: u32,
    /// System Options Register 7
	sopt7: u32,
    _pad3: [u32; 2],
    /// System Device Identification Register
	sdid: u32,
    pad4: [u32; 3],
    /// System Clock Gating Control Register 4
	scgc4: u32,
    /// System Clock Gating Control Register 5
	scgc5: u32,
    /// System Clock Gating Control Register 6
	scgc6: u32,
    /// System Clock Gating Control Register 7
	scgc7: u32,
    /// System Clock Divider Register 1
	clkdiv1: u32,
    /// System Clock Divider Register 2
	clkdiv2: u32,
    /// Flash Configuration Register 1
	fcfg1: u32,
    /// Flash Configuration Register 2
	fcfg2: u32,
    /// Unique Identification Register High
	uidh: u32,
    /// Unique Identification Register Mid-High
	uidmh: u32,
    /// Unique Identification Register Mid Low
	uidml: u32,
    /// Unique Identification Register Low
	uidl: u32,
}


impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        &mut *(0x40047000 as *mut Sim)
    }


    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    let mut scgc = core::ptr::read_volatile(&self.scgc5);
                    scgc |= 0x00000800;
                    core::ptr::write_volatile(&mut self.scgc5, scgc);
                }
            }
        }
    }
}
