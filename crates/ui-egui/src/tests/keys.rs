//! The keymap: Aptimizer's keys (the default), which AptCAD's mode shares where both have the tool, or the Classic keys.

use super::*;
use crate::keys::Keymap;

/// The letters typed over the view, as they are: `shortcut` gives a Classic key its keymap's.
fn typed(h: &mut Harness<BimApp>, letters: &str) {
    h.event(Event::Text(letters.into()));
    h.run();
}

#[test]
fn aptimizers_keys_are_the_default_and_the_classic_keys_they_replace_do_nothing() {
    let mut h = app(BOX);
    assert_eq!(h.state().keymap, Keymap::Aptimizer);
    typed(&mut h, "WN");
    assert!(matches!(h.state().tool, Tool::Modify), "Classic's Window is no key in Aptimizer's");
    typed(&mut h, "WI");
    assert!(matches!(h.state().tool, Tool::Window));
    key(&mut h, Key::Escape);
    key(&mut h, Key::Escape);
    typed(&mut h, "VV");
    assert!(h.state().vg_dialog.is_none());
    typed(&mut h, "VI");
    assert!(h.state().vg_dialog.is_some(), "Visibility/Graphics");
}

#[test]
fn the_keys_both_modes_share_do_the_same_here_as_in_aptcad() {
    let mut h = app(BOX);
    // Mirror (of the selection) and Split, as AptCAD types Mirror and Break.
    for (letters, tool) in [("MI", "Mirror"), ("BR", "Split")] {
        click(&mut h, P2(0.0, 3000.0));
        typed(&mut h, letters);
        let now = format!("{:?}", h.state().tool);
        assert!(now.starts_with(tool), "{letters}: {now}");
        key(&mut h, Key::Escape);
        key(&mut h, Key::Escape);
    }
    let shown = h.state().docking.shown(crate::dock::Palette::Properties);
    typed(&mut h, "PR");
    assert_ne!(h.state().docking.shown(crate::dock::Palette::Properties), shown, "Properties, shown or hidden");
    let keys: Vec<&str> = crate::keyed_shortcuts(Keymap::Aptimizer).into_iter().map(|(k, _)| k).collect();
    for shared in ["MV", "CO", "RO", "MI", "OF", "TR", "AL", "BR", "MA", "PU", "PR", "DE", "XP", "JO"] {
        assert!(keys.contains(&shared), "{shared}");
    }
}

#[test]
fn classic_keeps_every_classic_key_and_none_of_aptimizers() {
    let mut h = app(BOX);
    h.state_mut().keymap = Keymap::Classic;
    typed(&mut h, "WI");
    assert!(matches!(h.state().tool, Tool::Modify));
    typed(&mut h, "WN");
    assert!(matches!(h.state().tool, Tool::Window));
    key(&mut h, Key::Escape);
    key(&mut h, Key::Escape);
    typed(&mut h, "VG");
    assert!(h.state().vg_dialog.is_some());
}

#[test]
fn every_key_a_keymap_lists_works_and_no_other_does() {
    for keymap in Keymap::ALL {
        let listed: Vec<&str> = crate::keyed_shortcuts(keymap).into_iter().map(|(k, _)| k).collect();
        let mut seen = std::collections::BTreeMap::new();
        for (letters, name) in crate::keyed_shortcuts(keymap) {
            if let Some(other) = seen.insert(letters, name) {
                assert_eq!(letters, "CL", "{keymap:?}: {letters} is both {other} and {name}");
            }
        }
        for a in ('A'..='Z').chain('0'..='9') {
            for b in ('A'..='Z').chain('0'..='9') {
                let mut app = BimApp::new(Session::new());
                app.keymap = keymap;
                app.typed = format!("{a}{b}");
                assert_eq!(app.shortcut(), listed.contains(&format!("{a}{b}").as_str()), "{keymap:?}: {a}{b}");
            }
        }
    }
}

#[test]
fn tooltips_and_find_name_the_keymaps_key() {
    assert_eq!(Keymap::Aptimizer.keyed("Window (WN): click a wall"), "Window (WI): click a wall");
    assert_eq!(Keymap::Classic.keyed("Window (WN): click a wall"), "Window (WN): click a wall");
    assert_eq!(Keymap::Aptimizer.keyed("Visibility/Graphics (VV): what this view hides"), "Visibility/Graphics (VI): what this view hides");
    assert_eq!(Keymap::Aptimizer.keyed("Wall (WA): each click builds a wall"), "Wall (WA): each click builds a wall");
    let mut h = app(BOX);
    shortcut(&mut h, "KS");
    assert!(h.query_by_label("WI").is_some() && h.query_by_label("WN").is_none(), "Keyboard Shortcuts lists Aptimizer's");
}

#[test]
fn the_keymap_is_picked_in_settings_and_the_settings_file_keeps_it() {
    let file = std::env::temp_dir().join(format!("aptbim-ui-keymap-{}", std::process::id())).join("settings.json");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, r#"{"keymap": "from-a-newer-version"}"#).unwrap();
    let mut h = app(BOX);
    h.state_mut().use_settings(file.clone());
    h.run();
    assert_eq!(h.state().keymap, Keymap::Aptimizer);
    press(&mut h, "Settings");
    pick(&mut h, Role::RadioButton, "Classic");
    press(&mut h, "Close");
    assert_eq!(h.state().keymap, Keymap::Classic);
    let kept: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(kept["keymap"], "classic");
    let mut again = app(BOX);
    again.state_mut().use_settings(file.clone());
    again.run();
    assert_eq!(again.state().keymap, Keymap::Classic);
    let _ = std::fs::remove_dir_all(file.parent().unwrap());
}

#[test]
fn a_host_reads_and_sets_the_keymap_by_its_code() {
    let mut h = app(BOX);
    assert_eq!(h.state().keymap_code(), "aptimizer");
    h.state_mut().set_keymap_code("classic");
    assert_eq!((h.state().keymap, h.state().keymap_code()), (Keymap::Classic, "classic"));
    h.state_mut().set_keymap_code("dvorak");
    assert_eq!(h.state().keymap, Keymap::Classic, "a code it doesn't know changes nothing");
}
