use crate::{event::key::Key, event::mouse::Mouse, Canopy, Result};

struct KeyBinding {
    key: Key,
    mode: String,
    path: String,
    script: String,
}

struct MouseBinding {
    mouse: Mouse,
    mode: String,
    path: String,
    script: String,
}

pub struct MapBuilder {
    keys: Vec<KeyBinding>,
    mice: Vec<MouseBinding>,
    mode: String,
    path_filter: String,
}

impl MapBuilder {
    pub fn new() -> Self {
        MapBuilder {
            keys: vec![],
            mice: vec![],
            mode: "".into(),
            path_filter: "".into(),
        }
    }

    pub fn with_mode(mut self, m: &str) -> Self {
        self.mode = m.to_string();
        self
    }

    pub fn with_path(mut self, m: &str) -> Self {
        self.path_filter = m.into();
        self
    }

    pub fn key<K>(mut self, key: K, script: &str) -> Self
    where
        Key: From<K>,
    {
        self.keys.push(KeyBinding {
            key: key.into(),
            script: script.into(),
            mode: self.mode.clone(),
            path: self.path_filter.clone(),
        });
        self
    }

    pub fn mouse<K>(mut self, m: K, script: &str) -> Self
    where
        Mouse: From<K>,
    {
        self.mice.push(MouseBinding {
            mouse: m.into(),
            script: script.into(),
            mode: self.mode.clone(),
            path: self.path_filter.clone(),
        });
        self
    }

    pub fn build(self, c: &mut Canopy) -> Result<()> {
        for m in self.mice {
            c.bind_mode_mouse(m.mouse, &m.mode, &m.path, &m.script)?;
        }
        for k in self.keys {
            c.bind_mode_key(k.key, &k.mode, &k.path, &k.script)?;
        }
        Ok(())
    }
}
