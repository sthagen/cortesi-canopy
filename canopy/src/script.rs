use std::{cell::RefCell, collections::HashMap};

use rhai;

use crate::{commands, error, Core, Node, NodeId, NodeName, Result};

#[derive(Debug, Clone)]
pub struct Script {
    ast: rhai::AST,
    source: String,
}

impl Script {
    pub fn source(&self) -> &str {
        &self.source
    }
}

struct ScriptGlobal<'a> {
    core: &'a dyn Core,
    root: &'a mut dyn Node,
    node_id: NodeId,
}

thread_local! {
    static SCRIPT_GLOBAL: RefCell<Option<ScriptGlobal<'static>>> = RefCell::new(None);
}

struct ScriptGuard {}

impl Drop for ScriptGuard {
    fn drop(&mut self) {
        SCRIPT_GLOBAL.with(|g| {
            *g.borrow_mut() = None;
        });
    }
}

unsafe fn extend_lifetime<'b>(r: ScriptGlobal<'b>) -> ScriptGlobal<'static> {
    std::mem::transmute::<ScriptGlobal<'b>, ScriptGlobal<'static>>(r)
}

#[derive(Debug)]
pub struct ScriptHost {
    engine: rhai::Engine,
}

impl ScriptHost {
    pub fn new() -> Self {
        ScriptHost {
            engine: rhai::Engine::new(),
        }
    }

    pub fn load(&mut self, cmds: &[commands::CommandDefinition]) {
        // We can't enable this yet - see:
        //      https://github.com/rhaiscript/rhai/issues/574
        // engine.set_strict_variables(true);
        let mut modules: HashMap<NodeName, rhai::Module> = HashMap::new();
        for i in cmds {
            if !modules.contains_key(&i.node) {
                let m = rhai::Module::new();
                modules.insert(i.node.clone(), m);
            }
            let m = modules.get_mut(&i.node).unwrap();
            let ci = commands::CommandInvocation {
                node: i.node.clone(),
                command: i.command.clone(),
            };
            m.set_raw_fn(
                i.command.to_string(),
                rhai::FnNamespace::Internal,
                rhai::FnAccess::Public,
                &[],
                move |_context, _args| {
                    SCRIPT_GLOBAL.with(|g| {
                        let mut b = g.borrow_mut();
                        let v = b.as_mut().unwrap();
                        commands::dispatch(v.core, v.node_id, v.root, &ci).unwrap();
                    });
                    Ok(())
                },
            );
        }
        for (n, m) in modules {
            self.engine.register_static_module(n.to_string(), m.into());
        }
    }

    pub fn compile(&self, source: &str) -> Result<Script> {
        let ast = self
            .engine
            .compile(source)
            .map_err(|_e| error::Error::Parse(error::ParseError {}))?;
        Ok(Script {
            ast,
            source: source.into(),
        })
    }

    pub fn execute(
        &self,
        core: &dyn Core,
        root: &mut dyn Node,
        node_id: NodeId,
        s: &Script,
    ) -> Result<()> {
        let sg = ScriptGlobal {
            core,
            root,
            node_id,
        };
        let _g = ScriptGuard {};
        SCRIPT_GLOBAL.with(|g| {
            *g.borrow_mut() = Some(unsafe { extend_lifetime(sg) });
        });

        self.engine
            .run_ast(&s.ast)
            .map_err(|e| error::Error::Script(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tutils::*;
    use crate::StatefulNode;

    #[test]
    fn texecute() -> Result<()> {
        run(|c, _, mut root| {
            let scr = c.script_host.compile("bb_la::c_leaf()")?;
            let id = root.a.a.id();
            c.script_host.execute(c, &mut root, id, &scr)?;
            assert_eq!(get_state().path, ["bb_la.c_leaf()"]);
            Ok(())
        })?;
        Ok(())
    }
}