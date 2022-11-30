use crate::module::model::FieldsDTO;
use crate::page::model::Page;
use crate::page::model::PageModuleDisplayDTO;
use crate::utils::error::CustomHttpError;

pub(crate) fn parse_page(page: (Page, FieldsDTO)) -> Result<PageModuleDisplayDTO, CustomHttpError> {
    let origin_page = page.0;
    let mut res: PageModuleDisplayDTO = origin_page.into();
    match page.1.categories {
        Some(modules) => {
            for module in modules {
                res.array_fields.insert(module.title, module.modules);
            }
        }
        None => {}
    };
    for module in page.1.modules {
        res.fields.insert(module.title.clone(), module);
    }
    Ok(res)
}
