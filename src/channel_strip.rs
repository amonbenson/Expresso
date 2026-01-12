#[derive(Default, Clone, Copy)]
pub struct ChannelStrip {
    current_value: u8,
    previous_value: u8,
}

impl ChannelStrip {
    pub fn process(&mut self, raw_value: u16) {
        // update the new value
        self.previous_value = self.current_value;
        self.current_value = raw_value.min(127) as u8;
    }

    pub fn previous_value(&self) -> u8 {
        self.previous_value
    }

    pub fn value(&self) -> u8 {
        self.current_value
    }

    pub fn changed(&self) -> bool {
        self.current_value != self.previous_value
    }
}
