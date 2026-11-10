use crate::terminal_probe::DefaultColors;

#[derive(Default)]
pub(super) struct PaletteRefresh {
    query_in_flight: bool,
    foreground: Option<(u8, u8, u8)>,
    background: Option<(u8, u8, u8)>,
    completed: Option<DefaultColors>,
    dirty: bool,
}

impl PaletteRefresh {
    pub(super) fn begin(&mut self) -> bool {
        if self.query_in_flight {
            self.dirty = true;
            return false;
        }
        *self = Self {
            query_in_flight: true,
            ..Self::default()
        };
        true
    }

    pub(super) fn receive(&mut self, slot: u8, rgb: (u8, u8, u8)) {
        if !self.query_in_flight {
            return;
        }
        match slot {
            10 => self.foreground = Some(rgb),
            11 => self.background = Some(rgb),
            _ => (),
        }
    }

    pub(super) fn finish(&mut self) -> bool {
        if !std::mem::take(&mut self.query_in_flight) {
            return false;
        }
        let dirty = std::mem::take(&mut self.dirty);
        if !dirty && let (Some(fg), Some(bg)) = (self.foreground.take(), self.background.take()) {
            self.completed = Some(DefaultColors { fg, bg });
        }
        dirty
    }

    pub(super) fn take_completed(&mut self) -> Option<DefaultColors> {
        self.completed.take()
    }
}

#[cfg(test)]
#[path = "palette_refresh_tests.rs"]
mod tests;
