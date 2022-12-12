use crate::module::model::FieldsDTO;
use crate::page::model::Page;
use crate::page::model::PageModuleDisplayDTO;
use crate::utils::error::HttpErrorCodes;

pub(crate) fn parse_page(page: (Page, FieldsDTO)) -> Result<PageModuleDisplayDTO, HttpErrorCodes> {
    // Get origin page
    let origin_page = page.0;
    let mut res: PageModuleDisplayDTO = origin_page.into();

    // Match categories of page
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

    // Return page
    Ok(res)
}
