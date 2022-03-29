use duplicate::duplicate_item;
use std::marker::PhantomData;

use crate as canopy;
use crate::{
    state::{NodeState, StatefulNode},
    Actions, Node, Render, Result, ViewPort,
};

/// Panes manages a set of child nodes arranged in a 2d grid.
#[derive(StatefulNode)]
pub struct Panes<S, A: Actions, N: Node<S, A>> {
    _marker: PhantomData<(S, A)>,
    pub children: Vec<Vec<N>>,
    pub state: NodeState,
}

impl<S, A: Actions, N> Panes<S, A, N>
where
    N: Node<S, A>,
{
    pub fn new(n: N) -> Self {
        Panes {
            children: vec![vec![n]],
            state: NodeState::default(),
            _marker: PhantomData,
        }
    }

    /// Get the offset of the current focus in the children vector.
    pub fn focus_coords(&mut self) -> Option<(usize, usize)> {
        for (x, col) in self.children.iter_mut().enumerate() {
            for (y, row) in col.iter_mut().enumerate() {
                if canopy::on_focus_path(row) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Delete the focus node. If a column ends up empty, it is removed.
    pub fn delete_focus(&mut self) -> Result<()> {
        if let Some((x, y)) = self.focus_coords() {
            canopy::focus_next(self)?;
            self.children[x].remove(y);
            if self.children[x].is_empty() {
                self.children.remove(x);
            }
            self.taint_tree()?;
        }
        Ok(())
    }

    /// Insert a node, splitting vertically. If we have a focused node, the new
    /// node is inserted in a row beneath it. If not, a new column is added.
    pub fn insert_row(&mut self, n: N) -> Result<()>
    where
        N: Node<S, A>,
    {
        if let Some((x, y)) = self.focus_coords() {
            self.children[x].insert(y, n);
        } else {
            self.children.push(vec![n]);
        }
        self.taint_tree()
    }

    /// Insert a node in a new column. If we have a focused node, the new node
    /// is added in a new column to the right.
    pub fn insert_col(&mut self, mut n: N) -> Result<()>
    where
        N: Node<S, A>,
    {
        let coords = self.focus_coords();
        canopy::focus_next(&mut n)?;
        if let Some((x, _)) = coords {
            self.children.insert(x + 1, vec![n])
        } else {
            self.children.push(vec![n])
        }
        self.taint_tree()
    }

    /// Returns the shape of the current child grid
    fn shape(&self) -> Vec<u16> {
        let mut ret = vec![];
        for i in &self.children {
            ret.push(i.len() as u16)
        }
        ret
    }
}

impl<S, A: Actions, N: Node<S, A>> Node<S, A> for Panes<S, A, N> {
    #[duplicate_item(
        method          reference(type);
        [children]      [& type];
        [children_mut]  [&mut type];
    )]
    fn method(
        self: reference([Self]),
        f: &mut dyn FnMut(reference([dyn Node<S, A>])) -> Result<()>,
    ) -> Result<()> {
        for col in reference([self.children]) {
            for row in col {
                f(row)?
            }
        }
        Ok(())
    }

    fn render(&mut self, _rndr: &mut Render, vp: ViewPort) -> Result<()> {
        let l = vp.screen_rect().split_panes(&self.shape())?;
        for (ci, col) in self.children.iter_mut().enumerate() {
            for (ri, row) in col.iter_mut().enumerate() {
                row.place(l[ci][ri])?;
            }
        }
        // FIXME - this should probably clear the area if the last node is
        // deleted.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backend::test::TestRender,
        geom::{Point, Rect},
        tutils::utils,
    };

    #[test]
    fn tlayout() -> Result<()> {
        let (_, mut tr) = TestRender::create();
        let (_, _) = utils::tcanopy(&mut tr);
        let tn = utils::TBranch::new("a");
        let mut p: Panes<utils::State, utils::TActions, utils::TBranch> = Panes::new(tn);
        let r = Rect {
            tl: Point::zero(),
            w: 100,
            h: 100,
        };
        p.place(r)?;

        assert_eq!(p.shape(), vec![1]);
        let tn = utils::TBranch::new("b");
        p.insert_col(tn)?;
        p.place(r)?;

        assert_eq!(p.shape(), vec![1, 1]);
        p.children[0][0].a.handle_focus()?;
        p.place(r)?;

        let tn = utils::TBranch::new("c");
        assert_eq!(p.focus_coords(), Some((0, 0)));
        p.insert_row(tn)?;
        p.place(r)?;

        assert_eq!(p.shape(), vec![2, 1]);

        p.children[1][0].a.handle_focus()?;
        assert_eq!(p.focus_coords(), Some((1, 0)));
        Ok(())
    }
}
