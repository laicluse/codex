use super::*;

impl App {
    pub(super) fn refresh_terminal_palette(&mut self, tui: &mut tui::Tui) {
        if tui.take_terminal_palette_refresh_request()
            && let Err(error) = tui.request_terminal_palette()
        {
            tracing::warn!(%error, "failed to request terminal colors");
        }
        if tui.apply_terminal_palette_update() {
            self.restore_runtime_theme_from_config();
            self.refresh_status_line();
        }
    }
}
