

pub struct Controller {
    buttons: [bool; 8],
    index: usize,
    strobe: bool,
}

impl Controller {
    pub fn new() -> Controller {
        Controller{buttons:[false; 8], index:0, strobe: true}
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
        self.strobe = data != 0;
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
