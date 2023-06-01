use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu};

/// Make a menu
/// Refer to Menu::os_default() for reference.
pub fn make_menu() -> Menu {
    Menu::new()
        .add_submenu(app_submenu())
        .add_submenu(file_submenu())
        .add_submenu(edit_submenu())
        .add_submenu(window_submenu())
}

fn app_submenu() -> Submenu {
    Submenu::new(
        "app".to_string(),
        Menu::new()
            .add_native_item(MenuItem::About("Jomai".to_string(), AboutMetadata::new()))
            .add_item(CustomMenuItem::new("acknowledgment", "Acknowledgment"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    )
}

fn file_submenu() -> Submenu {
    Submenu::new(
        "File",
        Menu::new()
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    )
}

fn edit_submenu() -> Submenu {
    Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::EnterFullScreen),
    )
}

fn window_submenu() -> Submenu {
    Submenu::new(
        "Window",
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::CloseWindow),
    )
}
