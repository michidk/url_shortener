use crate::utils;
use rocket_dyn_templates::Template;
use serde_json::{value::Value, Map};
use utils::{get_rnd_spell, get_app_info};

#[derive(serde::Serialize)]
struct Context<T> {
    name: String,
    spell: String,
    app_info: Map<String, Value>,
    view: T,
}

#[macro_export]
macro_rules! context {
  ($($name:ident : $val:expr,)*) => {
    #[allow(non_camel_case_types)]
    {
      #[derive(serde::Serialize)]
      struct Context<$($name,)*> {
        $($name: $name,)*
      }

      Context {
        $($name: $val,)*
      }
    }
  };
}

pub(crate) fn render<T: serde::Serialize>(name: &'static str, context: T) -> Template {
    Template::render(
        name,
        &Context {
            name: name.to_owned(),
            spell: get_rnd_spell(),
            app_info: get_app_info(),
            view: context,
        },
    )
}
