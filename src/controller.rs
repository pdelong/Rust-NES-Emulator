

pub struct Controller {
    buttons: [bool; 8],
    pub index: usize,
    pub strobe: bool,
    strobeenable: bool,
}

impl Controller {
    pub fn new(strobeenable: bool) -> Controller {
        Controller{buttons:[false; 8], index:0, strobe: true, strobeenable: strobeenable}
    }

    pub fn read(&mut self) -> u8 {
        let mut value = 0;
        if self.index < 8 {
            value = if self.buttons[self.index] { 1 } else { 0 };
        }

        self.index += 1;
        if self.strobe {
            self.index = 0;
        }
	    value
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data != 0 && self.strobeenable;
        if self.strobe {
            self.index = 0;
        }
    }

    pub fn set_all(&mut self, data: [bool; 8]) {
        if self.strobe {
            self.buttons = data;
        }
    }
}
