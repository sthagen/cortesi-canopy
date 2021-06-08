use std::io::Write;
use std::marker::PhantomData;

use crate as canopy;
use crate::geom::Rect;
use crate::Canopy;
use crate::Node;
use anyhow::Result;

/// Panes manages a set of child nodes arranged in a 2d grid.
pub struct Panes<S, N: canopy::Node<S>> {
    _marker: PhantomData<S>,
    pub children: Vec<Vec<N>>,
    pub state: canopy::NodeState,
    pub rect: Option<Rect>,
}

impl<S, N: canopy::Node<S>> Panes<S, N> {
    pub fn new(n: N) -> Self {
        Panes {
            children: vec![vec![n]],
            state: canopy::NodeState::default(),
            rect: None,
            _marker: PhantomData,
        }
    }
    /// Get the offset of the current focus in the children vector.
    pub fn focus_coords(&mut self, app: &Canopy) -> Option<(usize, usize)> {
        for (x, col) in self.children.iter_mut().enumerate() {
            for (y, row) in col.iter_mut().enumerate() {
                if app.on_focus_path(row) {
                    return Some((x, y));
                }
            }
        }
        None
    }
    /// Delete the focus node. If a column ends up empty, it is removed.
    pub fn delete_focus(&mut self, app: &mut Canopy) -> Result<()> {
        if let Some((x, y)) = self.focus_coords(app) {
            app.focus_next(self)?;
            self.children[x].remove(y);
            if self.children[x].is_empty() {
                self.children.remove(x);
            }
            self.layout(app, self.rect, None)?;
            app.taint_tree(self)?;
        }
        Ok(())
    }
    /// Insert a node, splitting vertically. If we have a focused node, the new
    /// node is inserted in a row beneath it. If not, a new column is added.
    pub fn insert_row(&mut self, app: &Canopy, n: N) -> Result<()>
    where
        N: canopy::Node<S>,
    {
        if let Some((x, y)) = self.focus_coords(app) {
            self.children[x].insert(y, n);
        } else {
            self.children.push(vec![n]);
        }
        app.taint_tree(self)
    }
    /// Insert a node in a new column. If we have a focused node, the new node
    /// is added in a new column to the right.
    pub fn insert_col(&mut self, app: &mut Canopy, mut n: N) -> Result<()>
    where
        N: canopy::Node<S>,
    {
        let coords = self.focus_coords(app);
        app.focus_next(&mut n)?;
        if let Some((x, _)) = coords {
            self.children.insert(x + 1, vec![n])
        } else {
            self.children.push(vec![n])
        }
        app.taint_tree(self)
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

impl<S, N: canopy::Node<S>> Node<S> for Panes<S, N> {
    fn rect(&self) -> Option<Rect> {
        self.rect
    }
    fn state(&mut self) -> &mut canopy::NodeState {
        &mut self.state
    }
    fn layout(&mut self, app: &mut Canopy, rect: Option<Rect>, _virt: Option<Rect>) -> Result<()> {
        self.rect = rect;
        if let Some(a) = rect {
            let l = a.split_panes(self.shape())?;
            for (ci, col) in self.children.iter_mut().enumerate() {
                for (ri, row) in col.iter_mut().enumerate() {
                    row.layout(app, Some(l[ci][ri]), None)?;
                }
            }
        }
        Ok(())
    }
    fn children(
        &mut self,
        f: &mut dyn FnMut(&mut dyn canopy::Node<S>) -> Result<()>,
    ) -> Result<()> {
        for col in &mut self.children {
            for row in col {
                f(row)?
            }
        }
        Ok(())
    }
    fn render(&mut self, _: &mut Canopy, _: &mut dyn Write) -> Result<()> {
        // FIXME - this should probably clear the area if the last node is
        // deleted.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tutils::utils;
    use anyhow::Result;

    #[test]
    fn tlayout() -> Result<()> {
        let mut app = Canopy::new();
        let tn = utils::TBranch::new("a");
        let mut p: Panes<utils::State, utils::TBranch> = Panes::new(tn);
        assert_eq!(p.shape(), vec![1]);
        let tn = utils::TBranch::new("b");
        p.insert_col(&mut app, tn)?;
        assert_eq!(p.shape(), vec![1, 1]);
        app.set_focus(&mut p.children[0][0].a)?;
        let tn = utils::TBranch::new("c");
        assert_eq!(p.focus_coords(&mut app), Some((0, 0)));
        p.insert_row(&mut app, tn)?;
        assert_eq!(p.shape(), vec![2, 1]);
        app.set_focus(&mut p.children[1][0].a)?;
        assert_eq!(p.focus_coords(&mut app), Some((1, 0)));
        Ok(())
    }
}
