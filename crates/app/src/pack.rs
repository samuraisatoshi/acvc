use acvc_domain::{PackReport, Result};

use crate::App;

impl App {
    pub fn pack(&self) -> Result<PackReport> {
        self.ports.objects.pack()
    }
}
