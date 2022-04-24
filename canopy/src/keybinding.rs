use crate::{Command, Commands};

/// The Keybindings struct manages the global set of key bindings for the app.
pub struct Keybindings {}

impl Keybindings {
    pub fn new() -> Self {
        Keybindings {}
    }

    fn load(&mut self, f: fn() -> Vec<Command>) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as canopy;
    use crate::{command, derive_commands, Result};

    #[test]
    fn kb_load() -> Result<()> {
        #[derive(canopy::StatefulNode)]
        struct Foo {
            state: canopy::NodeState,
            a_triggered: bool,
            b_triggered: bool,
        }

        impl canopy::Node for Foo {}

        #[derive_commands]

        impl Foo {
            #[command]
            /// This is a comment.
            /// Multiline too!
            fn a(&mut self) -> canopy::Result<()> {
                self.a_triggered = true;
                Ok(())
            }
            #[command]
            fn b(&mut self) -> canopy::Result<()> {
                self.b_triggered = true;
                Ok(())
            }
        }

        let mut kb = Keybindings::new();
        kb.load(Foo::commands);

        Ok(())
    }
}
