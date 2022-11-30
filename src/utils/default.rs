use actix_web::web::Data;
use handlebars::{
    to_json, Context, Handlebars, Helper, HelperDef, JsonRender, Output, RenderContext,
    RenderError, ScopedJson,
};
use std::sync::Mutex;

fn get(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let module_title = h
        .param(0)
        .ok_or(RenderError::new(
            "No module title provided to helper function.",
        ))?
        .render();

    let field = (|| -> Result<String, RenderError> {
        let values = ctx
            .data()
            .get("fields")
            .ok_or(RenderError::new("No fields exist on this page."))?
            .get(module_title.clone())
            .ok_or(RenderError::new(&format!(
                "Field `{}` does not exist on the page.",
                module_title
            )))?
            .get("content")
            .unwrap()
            .render();

        Ok(values)
    })();

    out.write(&field.unwrap_or_else(|e| e.desc))?;
    Ok(())
}

#[derive(Clone, Copy)]
pub struct ArrayHelper;

impl HelperDef for ArrayHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let module_title = h
            .param(0)
            .ok_or(RenderError::new(
                "No module title provided to helper function.",
            ))?
            .render();

        let fields = (|| -> Result<ScopedJson, RenderError> {
            let values = ctx
                .data()
                .get("array_fields")
                .ok_or(RenderError::new("No fields exist on this page."))?
                .get(module_title.clone())
                .ok_or(RenderError::new(&format!(
                    "Field `{}` does not exist on the page.",
                    module_title
                )))?
                .clone()
                .into();

            Ok(values)
        })();
        let empty_array: Vec<String> = Vec::new();
        Ok(Some(fields.unwrap_or(to_json(empty_array).into())))
    }
}

pub static ARRAY_HELPER: ArrayHelper = ArrayHelper;

pub fn register_helpers(handlebars: Data<Mutex<Handlebars<'_>>>) {
    handlebars
        .lock()
        .unwrap()
        .register_helper("get", Box::new(get));
    handlebars
        .lock()
        .unwrap()
        .register_helper("getarray", Box::new(ARRAY_HELPER));
}
