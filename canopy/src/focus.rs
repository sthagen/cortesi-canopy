use crate::{
    geom::{Direction, Rect},
    global::STATE,
    locate,
    node::{postorder, preorder, Node, Walk},
    Outcome, Result, Walker,
};

/// Is the specified node on the focus path? A node is on the focus path if it
/// has focus, or if it's the ancestor of a node with focus.
pub fn is_on_path(n: &mut dyn Node) -> bool {
    let mut onpath = false;
    walk(n, &mut |_| -> Result<()> {
        onpath = true;
        Ok(())
    })
    // We're safe to unwrap, because our closure can't return an error.
    .unwrap();
    onpath
}

/// Return the focus path for the subtree under `root`.
pub fn path(root: &mut dyn Node) -> String {
    let mut path = Vec::new();
    walk(root, &mut |n| -> Result<()> {
        path.insert(0, n.name().to_string());
        Ok(())
    })
    .unwrap();
    "/".to_string() + &path.join("/")
}

/// Call a closure on the currently focused node and all its ancestors to the
/// root.
pub fn walk<R: Walker + Default>(
    root: &mut dyn Node,
    f: &mut dyn FnMut(&mut dyn Node) -> Result<R>,
) -> Result<R> {
    let mut focus_seen = false;
    let mut ret = R::default();
    let focus_gen = STATE.with(|global_state| -> u64 { global_state.borrow().focus_gen });
    postorder(root, &mut |x| -> Result<Walk<()>> {
        Ok(if focus_seen {
            ret = ret.join(f(x)?);
            Walk::Continue
        } else if x.is_hidden() {
            // Hidden nodes don't hold focus
            Walk::Continue
        } else if x.state().focus_gen == focus_gen {
            focus_seen = true;
            ret = ret.join(f(x)?);
            Walk::Skip
        } else {
            Walk::Continue
        })
    })?;
    Ok(ret)
}

/// Find the area of the current terminal focus node under the specified `root`.
pub fn get_area(root: &mut dyn Node) -> Option<Rect> {
    let mut ret = None;
    walk(root, &mut |x| -> Result<()> {
        if ret == None {
            ret = Some(x.vp().screen_rect());
        }
        Ok(())
    })
    // We're safe to unwrap, because our closure can't return an error.
    .unwrap();
    ret
}

/// Move focus in a specified direction within the subtree at root.
pub fn shift_direction(root: &mut dyn Node, dir: Direction) -> Result<Outcome> {
    let mut seen = false;
    if let Some(start) = get_area(root) {
        start.search(dir, &mut |p| -> Result<bool> {
            if !root.vp().screen_rect().contains_point(p) {
                return Ok(true);
            }
            locate(root, p, &mut |x| -> Result<Walk<()>> {
                if !seen && x.accept_focus() {
                    x.set_focus();
                    seen = true;
                };
                Ok(Walk::Continue)
            })?;
            Ok(seen)
        })?
    }
    Ok(Outcome::handle())
}

/// Move focus to the right of the currently focused node within the subtree at root.
pub fn shift_right(root: &mut dyn Node) -> Result<Outcome> {
    shift_direction(root, Direction::Right)
}

/// Move focus to the left of the currently focused node within the subtree at root.
pub fn shift_left(root: &mut dyn Node) -> Result<Outcome> {
    shift_direction(root, Direction::Left)
}

/// Move focus upward of the currently focused node within the subtree at root.
pub fn shift_up(root: &mut dyn Node) -> Result<Outcome> {
    shift_direction(root, Direction::Up)
}

/// Move focus downward of the currently focused node within the subtree at root.
pub fn shift_down(root: &mut dyn Node) -> Result<Outcome> {
    shift_direction(root, Direction::Down)
}

/// Focus the first node that accepts focus in the pre-order traversal of
/// the subtree at root.
pub fn shift_first(root: &mut dyn Node) -> Result<Outcome> {
    let mut focus_set = false;
    preorder(root, &mut |x| -> Result<Walk<()>> {
        Ok(if !focus_set && x.accept_focus() {
            x.set_focus();
            focus_set = true;
            Walk::Skip
        } else {
            Walk::Continue
        })
    })?;
    Ok(Outcome::handle())
}

/// A node is on the focus path if it does not have focus itself, but some
/// node below it does.
pub fn is_focus_ancestor(n: &mut dyn Node) -> bool {
    if n.is_focused() {
        false
    } else {
        is_on_path(n)
    }
}

/// Focus the next node in the pre-order traversal of root. If no node with
/// focus is found, we focus the first node we can find instead.
pub fn shift_next(root: &mut dyn Node) -> Result<Outcome> {
    let mut focus_set = false;
    let mut focus_seen = false;
    preorder(root, &mut |x| -> Result<Walk<()>> {
        if !focus_set {
            if focus_seen {
                if x.accept_focus() {
                    x.set_focus();
                    focus_set = true;
                }
            } else if x.is_focused() {
                focus_seen = true;
            }
        }
        Ok(Walk::Continue)
    })?;
    if !focus_set {
        shift_first(root)
    } else {
        Ok(Outcome::handle())
    }
}

/// Focus the previous node in the pre-order traversal of `root`. If no node
/// with focus is found, we focus the first node we can find instead.
pub fn shift_prev(root: &mut dyn Node) -> Result<Outcome> {
    let current = STATE.with(|global_state| -> u64 { global_state.borrow().focus_gen });
    let mut focus_seen = false;
    let mut first = true;
    preorder(root, &mut |x| -> Result<Walk<()>> {
        // We skip the first node in the traversal
        if first {
            first = false
        } else if !focus_seen {
            if x.state().focus_gen == current {
                focus_seen = true;
            } else {
                if x.accept_focus() {
                    x.set_focus();
                }
            }
        }
        Ok(Walk::Continue)
    })?;
    Ok(Outcome::handle())
}

/// Returns the focal depth of the specified node. If the node is not part
/// of the focus chain, the depth is 0. If the node is a leaf focus, the
/// depth is 1.
pub fn focus_depth(n: &mut dyn Node) -> usize {
    let mut total = 0;
    walk(n, &mut |_| -> Result<()> {
        total += 1;
        Ok(())
    })
    // We're safe to unwrap, because our closure can't return an error.
    .unwrap();
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backend::test::TestRender, geom::Expanse, set_root_size, tutils::utils::*, StatefulNode,
    };

    fn run_test(func: impl FnOnce(TestRender, TRoot) -> Result<()>) -> Result<()> {
        let (_, tr) = TestRender::create();
        let mut root = TRoot::new();
        set_root_size(Expanse::new(100, 100), &mut root)?;
        reset_state();
        func(tr, root)
    }

    #[test]
    fn tpath() -> Result<()> {
        run_test(|_, mut root| {
            assert_eq!(path(&mut root), "/".to_string());
            shift_next(&mut root)?;
            assert_eq!(path(&mut root), "/r".to_string());
            shift_next(&mut root)?;
            assert_eq!(path(&mut root), "/r/ba".to_string());
            shift_next(&mut root)?;
            assert_eq!(path(&mut root), "/r/ba/ba_la".to_string());
            Ok(())
        })?;
        Ok(())
    }

    #[test]
    fn tshift_next() -> Result<()> {
        run_test(|_, mut root| {
            assert!(!root.is_focused());
            shift_next(&mut root)?;
            assert!(root.is_focused());

            shift_next(&mut root)?;
            assert!(root.a.is_focused());
            assert!(is_focus_ancestor(&mut root));
            assert!(!is_focus_ancestor(&mut root.a));

            shift_next(&mut root)?;
            assert!(root.a.a.is_focused());
            assert!(is_focus_ancestor(&mut root.a));
            shift_next(&mut root)?;
            assert!(root.a.b.is_focused());
            assert!(is_focus_ancestor(&mut root.a));
            shift_next(&mut root)?;
            assert!(root.b.is_focused());

            root.b.b.set_focus();
            assert!(is_focus_ancestor(&mut root.b));
            shift_next(&mut root)?;
            assert!(root.is_focused());
            Ok(())
        })?;
        Ok(())
    }

    #[test]
    fn t_shift_prev() -> Result<()> {
        run_test(|_, mut root| {
            assert!(!root.is_focused());
            shift_prev(&mut root)?;
            assert!(root.b.b.is_focused());

            shift_prev(&mut root)?;
            assert!(root.b.a.is_focused());

            shift_prev(&mut root)?;
            assert!(root.b.is_focused());

            root.set_focus();
            shift_prev(&mut root)?;
            assert!(root.b.b.is_focused());

            Ok(())
        })?;
        Ok(())
    }

    #[test]
    fn tshift_right() -> Result<()> {
        run_test(|mut tr, mut root| {
            tr.render(&mut root)?;
            root.a.a.set_focus();
            shift_right(&mut root)?;
            assert!(root.b.a.is_focused());
            shift_right(&mut root)?;
            assert!(root.b.a.is_focused());

            root.a.b.set_focus();
            shift_right(&mut root)?;
            assert!(root.b.b.is_focused());
            shift_right(&mut root)?;
            assert!(root.b.b.is_focused());
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn tfoci() -> Result<()> {
        run_test(|_, mut root| {
            assert_eq!(path(&mut root), "/".to_string());

            assert!(!is_on_path(&mut root));
            assert!(!is_on_path(&mut root.a));

            root.a.a.set_focus();
            assert!(is_on_path(&mut root));
            assert!(is_on_path(&mut root.a));
            assert!(!is_on_path(&mut root.b));
            assert_eq!(path(&mut root), "/r/ba/ba_la".to_string());

            root.a.set_focus();
            assert_eq!(path(&mut root), "/r/ba".to_string());

            root.set_focus();
            assert_eq!(path(&mut root), "/r".to_string());

            root.b.a.set_focus();
            assert_eq!(path(&mut root), "/r/bb/bb_la".to_string());
            Ok(())
        })?;

        Ok(())
    }
}
