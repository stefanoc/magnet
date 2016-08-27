use liquid;
pub use liquid::Value;
use liquid::{Renderable, Context};
use magnet_core::{Stack, Request};
use magnet_core::ext::Key;
use std::collections::HashMap;

pub struct Views;

#[derive(Clone)]
pub struct Templates {
    templates: HashMap<String, String>,
}

pub struct View {
    template: String,
    context: Context,
}

impl Key for Views {
    type Value = Templates;
}

impl Templates {
    pub fn new() -> Templates {
        Templates { templates: HashMap::new() }
    }

    pub fn insert<N, T>(&mut self, name: N, template: T)
        where N: Into<String>,
              T: Into<String>
    {
        self.templates.insert(name.into(), template.into());
    }

    pub fn view(&self, name: &str) -> View {
        if let Some(template) = self.templates.get(name) {
            View::new(template.clone())
        } else {
            panic!("Invalid view: {}", name);
        }
    }
}

impl View {
    fn new(template: String) -> View {
        View {
            template: template,
            context: Context::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.context.set_val(key, value);
    }

    pub fn render(&mut self) -> String {
        let t = liquid::parse(&self.template, Default::default()).unwrap();
        t.render(&mut self.context).unwrap().unwrap()
    }
}
