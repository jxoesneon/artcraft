//! Integration test for Artcraft hub.

use artcraft_ui_martensite::{ArtcraftApp, ViewMode};

#[test]
fn test_artcraft_hub_workflow() {
    let mut app = ArtcraftApp::new();
    let pid = app.hub.create_project("Concept Art", "photocraft");
    assert_eq!(pid, 1);
    app.set_view(ViewMode::Assets);
    assert_eq!(app.active_view, ViewMode::Assets);
}
