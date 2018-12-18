// manual: https://www.pjrc.com/teensy/K20P64M72SF1RM.pdf

#[repr(C,packed)]
struct Gpio {
    /// Port Data Output Register (see section 49.2.1 on page 1335)
	pdor: u32,
    /// Port Set Output Register (see section 49.2.2 on page 1335)
	psor: u32,
    /// Port Clear Output Register (see section 49.2.3 on page 1336)
	pcor: u32,
    /// Port Toggle Output Register (see section 49.2.4 on page 1336)
	ptor: u32,
    /// Port Data Input Register (see section 49.2.5 on page 1337)
	pdir: u32,
    /// Port Data Direction Register (see section 49.2.6 on page 1337)
	pddr: u32,
}

#[repr(C,packed)]
struct GpioBitband {
    pdor: [u32; 32],
    psor: [u32; 32],
    pcor: [u32; 32],
    ptor: [u32; 32],
    pdir: [u32; 32],
    pddr: [u32; 32]
}

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            0x4004B000 => PortName::C,
            _ => unreachable!()
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            PortName::C => 0x43FE1000 as *mut GpioBitband
        };

        Gpio { gpio, pin }
    }
    pub fn output(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).pddr[self.pin], 1);
        }
    }

    pub fn high(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).psor[self.pin], 1);
        }
    }
}
