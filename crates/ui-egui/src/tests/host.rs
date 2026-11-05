//! What a host that owns the window (Aptimizer Studio) asks of AptBIM: run a command as the window's
//! own buttons do, and hear why it failed.

use super::*;

#[test]
fn a_host_runs_a_command_as_one_undo_step_and_hears_why_one_fails() {
    let mut h = app(BOX);
    let walls = ids(&h, "wall").len();
    let made = h.state_mut().run_command("wall", json!({"points": [[0, 0], [0, -4000]]}));
    assert!(made.is_ok(), "{made:?}");
    assert_eq!(ids(&h, "wall").len(), walls + 1);
    h.state_mut().run_command("undo", json!({})).unwrap();
    assert_eq!(ids(&h, "wall").len(), walls, "one undo takes it back");
    let refused = h.state_mut().run_command("wall", json!({"points": [[0, 0]]}));
    let e = refused.unwrap_err();
    assert!(!e.is_empty() && h.state().status.error, "the window says why too: {e}");
    assert!(h.state_mut().run_command("no_such_command", json!({})).is_err());
}
