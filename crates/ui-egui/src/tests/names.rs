//! Tools AptBIM now names its own way (View Visibility, once Visibility/Graphics) answer to their older names where a user's
//! files hold them: the shortcuts a settings file keeps, and a shortcuts file loaded in Keyboard Shortcuts (Revit's among them).

use super::*;
use crate::shortcuts::{ACTIONS, RENAMED, resolve_action};

#[test]
fn every_older_name_stands_for_an_action_the_window_has_by_another_name() {
    for (old, new) in RENAMED {
        assert!(ACTIONS.contains(new), "{new}");
        assert!(!ACTIONS.contains(old), "{old} is still an action's name");
        assert_eq!(resolve_action(old), Some(*new));
        assert_eq!(resolve_action(&old.to_uppercase()), Some(*new), "in any case");
    }
}

#[test]
fn shortcuts_kept_or_loaded_under_an_older_name_reach_the_tool_it_is_now() {
    let dir = std::env::temp_dir().join(format!("aptbim-ui-names-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let settings = dir.join("settings.json");
    std::fs::write(&settings, r#"{"shortcuts": {"QV": "Visibility/Graphics", "QT": "Tag by Category"}}"#).unwrap();
    let mut h = app(BOX);
    h.state_mut().use_settings(settings);
    h.run();
    assert_eq!(h.state().custom_shortcuts.get("QV").map(String::as_str), Some("View Visibility"));
    assert_eq!(h.state().custom_shortcuts.get("QT").map(String::as_str), Some("Tag Element"));
    h.event(Event::Text("QV".into()));
    h.run();
    assert!(h.state().vg_dialog.is_some(), "the shortcut opens View Visibility");
    // A shortcuts file in Revit's own form, naming a tool by its older name.
    let file = dir.join("shortcuts.xml");
    std::fs::write(&file, r#"<Shortcut CommandName="Purge Unused" CommandId="ID_PURGE" Shortcuts="QP" />"#).unwrap();
    h.state_mut().load_shortcuts_from_file(&file);
    assert_eq!(h.state().custom_shortcuts.get("QP").map(String::as_str), Some("Clean Up Unused"));
    let _ = std::fs::remove_dir_all(&dir);
}
