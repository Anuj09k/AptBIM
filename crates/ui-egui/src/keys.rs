//! The keymap: which two-letter keys typed over a view reach which tool. Aptimizer's (the default) gives the tools both of
//! Aptimizer Studio's modes have the keys AptCAD types for them (MI Mirror, BR Split as AptCAD's Break, XP Explode, JO Join,
//! PR Properties), and a few others the first letters of their names; Classic keeps the keys AptBIM had before. Ctrl and
//! function keys are the same in both.
//!
//! The window dispatches Classic keys (`SHORTCUTS` in `lib.rs`, the commands' aliases): [`Keymap::classic`] says which one a
//! typed key stands for, and [`Keymap::keyed`] puts the keymap's keys in the tooltips that name Classic's.

use std::borrow::Cow;
use std::cell::Cell;

use aptbim_i18n::msg;

/// The keymaps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Keymap {
    #[default]
    Aptimizer,
    Classic,
}

/// Aptimizer's keys where they differ from Classic's: each with the Classic keys it takes the place of. The first of those
/// is the one it does; any other did the same, and is no key in Aptimizer's.
pub const MOVED: [(&str, &[&str]); 10] = [
    ("WI", &["WN"]),
    ("LE", &["LL"]),
    ("XS", &["CR"]),
    ("MI", &["MM"]),
    ("BR", &["SL"]),
    ("VI", &["VV", "VG"]),
    ("PR", &["PP", "VP"]),
    ("PB", &["NP"]),
    ("XP", &["EX"]),
    ("JO", &["PJ"]),
];

thread_local! {
    /// The keymap of the window drawing now: the tooltips name its keys.
    static NOW: Cell<Keymap> = const { Cell::new(Keymap::Aptimizer) };
}

/// The keymap of the window drawing now.
pub fn now() -> Keymap {
    NOW.with(Cell::get)
}

/// Makes `keymap` the one the tooltips drawn from now on name.
pub fn follow(keymap: Keymap) {
    NOW.with(|k| k.set(keymap));
}

impl Keymap {
    pub const ALL: [Keymap; 2] = [Keymap::Aptimizer, Keymap::Classic];

    /// Its name, in English: Aptimizer's is a name, Classic is translated.
    pub fn name(self) -> &'static str {
        match self {
            Keymap::Aptimizer => "Aptimizer",
            Keymap::Classic => msg!("Classic"),
        }
    }

    /// What a settings file calls it.
    pub fn code(self) -> &'static str {
        match self {
            Keymap::Aptimizer => "aptimizer",
            Keymap::Classic => "classic",
        }
    }

    pub fn from_code(code: &str) -> Option<Keymap> {
        Keymap::ALL.into_iter().find(|k| k.code() == code)
    }

    /// The Classic key `typed` stands for in this keymap: itself, the Classic key an Aptimizer key does, or none for a Classic
    /// key Aptimizer's moved.
    pub fn classic(self, typed: &str) -> Option<&str> {
        if self == Keymap::Classic {
            return Some(typed);
        }
        if let Some((_, classic)) = MOVED.iter().find(|(key, _)| *key == typed) {
            return classic.first().copied();
        }
        let moved = MOVED.iter().any(|(_, classic)| classic.contains(&typed));
        (!moved).then_some(typed)
    }

    /// The key this keymap has for Classic `key`: its own, or none where it has no other (a second Classic key for the same).
    pub fn shown(self, key: &'static str) -> Option<&'static str> {
        if self == Keymap::Classic {
            return Some(key);
        }
        match MOVED.iter().find(|(_, classic)| classic.contains(&key)) {
            Some((new, classic)) => (classic.first() == Some(&key)).then_some(*new),
            None => Some(key),
        }
    }

    /// The key this keymap has for the same as Classic `key`, whichever of the Classic keys for it that is.
    pub fn key_for(self, key: &str) -> &str {
        match self {
            Keymap::Classic => key,
            Keymap::Aptimizer => MOVED.iter().find(|(_, classic)| classic.contains(&key)).map_or(key, |(new, _)| *new),
        }
    }

    /// `text` with each Classic key it names in parentheses, as tooltips do ("Window (WN): click a wall"), this keymap's.
    pub fn keyed(self, text: &str) -> Cow<'_, str> {
        if self == Keymap::Classic || !text.contains('(') {
            return Cow::Borrowed(text);
        }
        let mut out = Cow::Borrowed(text);
        for (new, classic) in MOVED {
            for old in classic {
                for after in [")", ",", ":", ";", " "] {
                    let from = format!("({old}{after}");
                    if out.contains(&from) {
                        out = Cow::Owned(out.replace(&from, &format!("({new}{after}")));
                    }
                }
            }
        }
        out
    }
}
