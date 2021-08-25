use std::marker::PhantomData;

use crate as canopy;
use crate::{
    event::key, widgets::tabs, Actions, Canopy, Node, NodeState, Outcome, Result, StatefulNode,
    ViewPort,
};

#[derive(StatefulNode)]

pub struct Logs<S, A: Actions, N>
where
    N: Node<S, A>,
{
    state: NodeState,
    _marker: PhantomData<(S, A, N)>,
}

impl<S, A: Actions, N> Node<S, A> for Logs<S, A, N>
where
    N: Node<S, A>,
{
    fn render(&mut self, app: &mut Canopy<S, A>, vp: ViewPort) -> Result<()> {
        app.render.fill("", vp.view_rect(), ' ')?;
        Ok(())
    }
}

impl<S, A: Actions, N> Logs<S, A, N>
where
    N: Node<S, A>,
{
    pub fn new() -> Self {
        Logs {
            state: NodeState::default(),
            _marker: PhantomData,
        }
    }
}

/// View contains the body of the inspector.
#[derive(StatefulNode)]

pub struct View<S, A: Actions, N>
where
    N: Node<S, A>,
{
    tabs: tabs::Tabs<S, A>,
    logs: Logs<S, A, N>,
    state: NodeState,
    _marker: PhantomData<(S, A, N)>,
}

impl<S, A: Actions, N> Node<S, A> for View<S, A, N>
where
    N: Node<S, A>,
{
    fn focus(&mut self, app: &mut Canopy<S, A>) -> Result<Outcome<A>> {
        app.set_focus(self);
        Ok(Outcome::handle())
    }

    fn handle_key(&mut self, app: &mut Canopy<S, A>, _: &mut S, k: key::Key) -> Result<Outcome<A>> {
        match k {
            c if c == key::KeyCode::Tab => self.tabs.next(app),
            _ => return Ok(Outcome::ignore()),
        };
        Ok(Outcome::handle())
    }

    fn render(&mut self, app: &mut Canopy<S, A>, vp: ViewPort) -> Result<()> {
        let parts = vp.carve_vstart(1)?;
        self.tabs.wrap(app, parts[0])?;
        self.logs.wrap(app, parts[1])?;
        Ok(())
    }

    fn children(&self, f: &mut dyn FnMut(&dyn Node<S, A>) -> Result<()>) -> Result<()> {
        f(&self.tabs)?;
        f(&self.logs)
    }

    fn children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Node<S, A>) -> Result<()>) -> Result<()> {
        f(&mut self.tabs)?;
        f(&mut self.logs)
    }
}

impl<S, A: Actions, N> View<S, A, N>
where
    N: Node<S, A>,
{
    pub fn new() -> Self {
        View {
            state: NodeState::default(),
            _marker: PhantomData,
            tabs: tabs::Tabs::new(vec!["Nodes".into(), "Events".into(), "Logs".into()]),
            logs: Logs::new(),
        }
    }
}