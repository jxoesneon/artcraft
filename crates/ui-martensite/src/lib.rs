//! Sovereign creative hub interface for ArtCraft built on Martensite.

pub mod asset_hub;
pub mod command_reg;
pub mod menus;
pub mod theme;

pub struct ArtcraftApp {
    pub hub: asset_hub::ProjectHub,
    pub active_view: ViewMode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    Projects,
    Assets,
    Timeline,
    Settings,
}

impl ArtcraftApp {
    pub fn new() -> Self {
        Self {
            hub: asset_hub::ProjectHub::new(),
            active_view: ViewMode::Projects,
        }
    }

    pub fn set_view(&mut self, view: ViewMode) {
        self.active_view = view;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artcraft_app() {
        let mut app = ArtcraftApp::new();
        assert_eq!(app.active_view, ViewMode::Projects);
        app.set_view(ViewMode::Assets);
        assert_eq!(app.active_view, ViewMode::Assets);
    }
}
