use std::time::Duration;

use canopy::{
    backend::crossterm::runloop,
    derive_commands,
    event::{key, mouse},
    fit,
    geom::Expanse,
    inspector::Inspector,
    style::solarized,
    widgets::{frame, list::*, Text},
    BackendControl, Node, NodeState, Outcome, Render, Result, StatefulNode,
};

#[derive(StatefulNode)]
struct IntervalItem {
    state: NodeState,
    child: Text,
    selected: bool,
    value: u64,
}

#[derive_commands]
impl IntervalItem {
    fn new() -> Self {
        IntervalItem {
            state: NodeState::default(),
            child: Text::new("0"),
            selected: false,
            value: 0,
        }
    }
    fn inc(&mut self) {
        self.value += 1;
        self.child = Text::new(&format!("{}", self.value))
    }
}

impl ListItem for IntervalItem {
    fn set_selected(&mut self, state: bool) {
        self.selected = state
    }
}

impl Node for IntervalItem {
    fn poll(&mut self) -> Option<Duration> {
        self.inc();
        self.taint();
        Some(Duration::from_secs(1))
    }

    fn fit(&mut self, target: Expanse) -> Result<Expanse> {
        self.child.fit(target)
    }

    fn children(&mut self, f: &mut dyn FnMut(&mut dyn Node) -> Result<()>) -> Result<()> {
        f(&mut self.child)
    }

    fn render(&mut self, r: &mut Render) -> Result<()> {
        let vp = self.vp();
        fit(&mut self.child, vp)?;
        if self.selected {
            r.style.push_layer("blue");
        }
        Ok(())
    }
}

#[derive(StatefulNode)]
struct StatusBar {
    state: NodeState,
}

#[derive_commands]
impl StatusBar {}

impl Node for StatusBar {
    fn render(&mut self, r: &mut Render) -> Result<()> {
        r.style.push_layer("statusbar");
        r.text(
            "statusbar/text",
            self.vp().view_rect().first_line(),
            "intervals",
        )?;
        Ok(())
    }
}

#[derive(StatefulNode)]
struct Root {
    state: NodeState,
    content: frame::Frame<List<IntervalItem>>,
    statusbar: StatusBar,
}

#[derive_commands]
impl Root {
    fn new() -> Self {
        Root {
            state: NodeState::default(),
            content: frame::Frame::new(List::new(vec![])),
            statusbar: StatusBar {
                state: NodeState::default(),
            },
        }
    }

    fn add_item(&mut self) -> Result<Outcome> {
        let lst = &mut self.content.child;
        lst.append(IntervalItem::new());
        self.taint();
        Ok(Outcome::Handle)
    }
}

impl Node for Root {
    fn render(&mut self, _: &mut Render) -> Result<()> {
        let (a, b) = self.vp().carve_vend(1);
        fit(&mut self.statusbar, b)?;
        fit(&mut self.content, a)?;
        Ok(())
    }

    fn accept_focus(&mut self) -> bool {
        true
    }

    fn handle_mouse(&mut self, _: &mut dyn BackendControl, k: mouse::Mouse) -> Result<Outcome> {
        let v = &mut self.content.child;
        match k {
            c if c == mouse::MouseAction::ScrollDown => v.update_viewport(&|vp| vp.down()),
            c if c == mouse::MouseAction::ScrollUp => v.update_viewport(&|vp| vp.up()),
            _ => return Ok(Outcome::Ignore),
        };
        Ok(Outcome::Handle)
    }

    fn handle_key(&mut self, ctrl: &mut dyn BackendControl, k: key::Key) -> Result<Outcome> {
        let lst = &mut self.content.child;
        match k {
            c if c == 'a' => {
                self.add_item()?;
            }
            c if c == 'd' => {
                lst.delete_selected();
            }
            c if c == 'g' => lst.select_first(),
            c if c == 'j' || c == key::KeyCode::Down => lst.select_next(),
            c if c == 'k' || c == key::KeyCode::Up => lst.select_prev(),
            c if c == ' ' || c == key::KeyCode::PageDown => lst.page_down(),
            c if c == key::KeyCode::PageUp => lst.page_up(),
            c if c == 'q' => ctrl.exit(0),
            _ => return Ok(Outcome::Ignore),
        };
        canopy::taint_tree(self);
        Ok(Outcome::Handle)
    }

    fn children(self: &mut Self, f: &mut dyn FnMut(&mut dyn Node) -> Result<()>) -> Result<()> {
        f(&mut self.statusbar)?;
        f(&mut self.content)?;
        Ok(())
    }
}

pub fn main() -> Result<()> {
    let mut colors = solarized::solarized_dark();
    colors.add(
        "statusbar/text",
        Some(solarized::BASE02),
        Some(solarized::BASE1),
        None,
    );
    let mut root = Inspector::new(key::Ctrl + key::KeyCode::Right, Root::new());
    runloop(&mut colors, &mut root)?;
    Ok(())
}
