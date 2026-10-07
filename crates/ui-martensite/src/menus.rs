//! Artcraft menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "File", items: &["hub.new_project", "hub.open_project"] },
];
