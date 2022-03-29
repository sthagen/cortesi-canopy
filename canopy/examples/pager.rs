use std::env;
use std::fs;

use canopy::{
    backend::crossterm::runloop,
    event::{key, mouse},
    inspector::Inspector,
    style::solarized,
    widgets::{frame, Text},
    BackendControl, Node, NodeState, Outcome, Render, Result, StatefulNode, ViewPort,
};

struct Handle {}

#[derive(StatefulNode)]
struct Root {
    state: NodeState,
    child: frame::Frame<Handle, (), Text<Handle, ()>>,
}

impl Root {
    fn new(contents: String) -> Self {
        Root {
            state: NodeState::default(),
            child: frame::Frame::new(Text::new(&contents)),
        }
    }
}

impl Node<Handle, ()> for Root {
    fn handle_focus(&mut self) -> Result<Outcome<()>> {
        self.set_focus();
        Ok(Outcome::handle())
    }

    fn handle_mouse(
        &mut self,
        _: &mut dyn BackendControl,
        _: &mut Handle,
        k: mouse::Mouse,
    ) -> Result<Outcome<()>> {
        let txt = &mut self.child.child;
        match k {
            c if c == mouse::MouseAction::ScrollDown => txt.update_viewport(&|vp| vp.down()),
            c if c == mouse::MouseAction::ScrollUp => txt.update_viewport(&|vp| vp.up()),
            _ => return Ok(Outcome::ignore()),
        };
        self.taint_tree()?;
        Ok(Outcome::handle())
    }

    fn handle_key(
        &mut self,
        ctrl: &mut dyn BackendControl,
        _: &mut Handle,
        k: key::Key,
    ) -> Result<Outcome<()>> {
        let txt = &mut self.child.child;
        match k {
            c if c == 'g' => txt.update_viewport(&|vp| vp.scroll_to(0, 0)),
            c if c == 'j' || c == key::KeyCode::Down => txt.update_viewport(&|vp| vp.down()),
            c if c == 'k' || c == key::KeyCode::Up => txt.update_viewport(&|vp| vp.up()),
            c if c == 'h' || c == key::KeyCode::Left => txt.update_viewport(&|vp| vp.left()),
            c if c == 'l' || c == key::KeyCode::Up => txt.update_viewport(&|vp| vp.right()),
            c if c == ' ' || c == key::KeyCode::PageDown => {
                txt.update_viewport(&|vp| vp.page_down());
            }
            c if c == key::KeyCode::PageUp => txt.update_viewport(&|vp| vp.page_up()),
            c if c == 'q' => canopy::exit(ctrl, 0),
            _ => return Ok(Outcome::ignore()),
        }
        self.taint_tree()?;
        Ok(Outcome::handle())
    }

    fn render(&mut self, _: &mut Render, vp: ViewPort) -> Result<()> {
        self.child.wrap(vp)
    }

    fn children(&self, f: &mut dyn FnMut(&dyn Node<Handle, ()>) -> Result<()>) -> Result<()> {
        f(&self.child)
    }

    fn children_mut(
        &mut self,
        f: &mut dyn FnMut(&mut dyn Node<Handle, ()>) -> Result<()>,
    ) -> Result<()> {
        f(&mut self.child)
    }
}

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: pager filename");
    } else {
        let colors = solarized::solarized_dark();
        let mut h = Handle {};
        let contents = fs::read_to_string(args[1].clone())?;
        let mut root = Inspector::new(key::Ctrl + key::KeyCode::Right, Root::new(contents));
        runloop(colors, &mut root, &mut h)?;
    }
    Ok(())
}
