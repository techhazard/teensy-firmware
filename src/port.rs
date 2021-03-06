use core;

#[derive(Clone, Copy)]
pub enum PortName {
    C
}


#[repr(C, packed)]
pub struct Port {
    /// Port Control Registers (numbered)
    pcr: [u32; 32],
    /// Global Pin Control Low Register
    gpclr: u32,
    /// Global Pin Control High Register
    gpchr: u32,
    reserved_0: [u8; 24],
    /// Interrupt Status Flag Register
    isfr: u32,
}



impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        &mut * match name {
            PortName::C => 0x4004B000 as *mut Port
        }
    }


    pub unsafe fn set_pin_mode(&mut self, port_number: usize, mut mode: u32) {
        let mut port_control_register = core::ptr::read_volatile(&self.pcr[port_number]);
        port_control_register &= 0xFFFFF8FF;
        mode &= 0x00000007;
        mode <<= 8;
        port_control_register |= mode;
        core::ptr::write_volatile(&mut self.pcr[port_number], port_control_register);
    }
}

pub struct Pin {
    port: *mut Port,
    pin: usize,
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin { port: self, pin: p }
    }
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

    pub fn low(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).psor[self.pin], 0);
        }
    }
}
