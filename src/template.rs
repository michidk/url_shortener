use crate::utils;
use rocket_contrib::templates::Template;
use utils::get_rnd_spell;

#[derive(serde::Serialize)]
struct Context<T> {
    spell: String,
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
            spell: get_rnd_spell(),
            view: context,
        },
    )
}
