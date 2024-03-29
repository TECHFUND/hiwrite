use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::module::model::CategoryDTO;
use crate::module::model::FieldsDTO;
use crate::module::model::Module;
use crate::module::model::ModuleCategory;
use crate::schema::category;
use crate::schema::modules;
use crate::schema::pages;
use crate::utils::model_manager::Model;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
#[primary_key(uuid)]
pub struct Page {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Clone)]
#[table_name = "pages"]
pub struct MutPage {
    pub uuid: Option<String>,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageModuleDisplayDTO {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    pub fields: HashMap<String, Module>,
    pub array_fields: HashMap<String, Vec<Module>>,
}

impl From<Page> for PageModuleDisplayDTO {
    fn from(origin_page: Page) -> Self {
        Self {
            page_name: origin_page.page_name.to_string(),
            uuid: origin_page.uuid.to_string(),
            page_url: origin_page.page_url.to_string(),
            page_title: origin_page.page_title.to_string(),
            time_created: origin_page.time_created,
            fields: HashMap::new(),
            array_fields: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageModuleDTO {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    pub fields: FieldsDTO,
}

impl From<Page> for PageModuleDTO {
    fn from(origin_page: Page) -> Self {
        Self {
            page_name: origin_page.page_name.to_string(),
            uuid: origin_page.uuid,
            page_url: origin_page.page_url.to_string(),
            page_title: origin_page.page_title.to_string(),
            time_created: origin_page.time_created,
            fields: FieldsDTO::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageDTO {
    pub uuid: String,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
}

impl From<Page> for PageDTO {
    fn from(origin_page: Page) -> Self {
        Self {
            uuid: origin_page.uuid.to_string(),
            page_name: origin_page.page_name.to_string(),
            page_url: origin_page.page_url.to_string(),
            page_title: origin_page.page_title.to_string(),
            time_created: origin_page.time_created,
        }
    }
}

impl Model<Page, MutPage, String, PageDTO> for Page {
    fn create(new_page: &MutPage, db: &PgConnection) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_into(pages::table)
            .values(new_page)
            .execute(db)?)
    }

    fn read_one(_id: String, db: &PgConnection) -> Result<PageDTO, diesel::result::Error> {
        use pages::dsl::uuid;

        let res = pages::table.filter(uuid.eq(_id)).first::<Self>(db)?.into();

        Ok(res)
    }

    fn read_all(db: &PgConnection) -> Result<Vec<PageDTO>, diesel::result::Error> {
        let res = pages::table
            .load::<Self>(db)?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(res)
    }

    fn update(
        _id: String,
        new_page: &MutPage,
        db: &PgConnection,
    ) -> Result<usize, diesel::result::Error> {
        use pages::dsl::uuid;

        Ok(diesel::update(pages::table.filter(uuid.eq(_id)))
            .set(new_page)
            .execute(db)?)
    }

    fn delete(_id: String, db: &PgConnection) -> Result<usize, diesel::result::Error> {
        use pages::dsl::uuid;

        Ok(diesel::delete(pages::table.filter(uuid.eq(_id))).execute(db)?)
    }
}

impl Page {
    pub fn read_one_join_on(
        _id: String,
        db: &PgConnection,
    ) -> Result<PageModuleDTO, diesel::result::Error> {
        use modules::dsl::category_uuid;
        use pages::dsl::uuid;

        let filtered_page = pages::table.filter(uuid.eq(_id)).first::<Page>(db)?;

        let modules_no_category = Module::belonging_to(&filtered_page)
            .filter(category_uuid.is_null())
            .load::<Module>(db)?;

        let categories = ModuleCategory::belonging_to(&filtered_page).load::<ModuleCategory>(db)?;

        let module_array: Vec<(Vec<Module>, ModuleCategory)> = Module::belonging_to(&categories)
            .load::<Module>(db)?
            .grouped_by(&categories)
            .iter()
            .map(|a| a.clone())
            .zip(categories)
            .collect::<Vec<_>>();

        let category_dtos: Vec<CategoryDTO> = module_array
            .iter()
            .map(|a| CategoryDTO {
                title: a.1.title.clone(),
                modules: a.0.clone(),
                uuid: a.1.uuid.clone(),
            })
            .collect::<Vec<_>>();

        let module_dto = FieldsDTO {
            modules: modules_no_category.into_iter().map(|m| m.into()).collect(),
            categories: Some(category_dtos),
        };

        let mut page_dto: PageModuleDTO = filtered_page.into();

        page_dto.fields = module_dto;

        Ok(page_dto)
    }

    pub fn read_one_join_on_url(
        id: String,
        db: &PgConnection,
    ) -> Result<(Self, FieldsDTO), diesel::result::Error> {
        use crate::schema::pages::dsl::page_url;

        let filtered_page = pages::table.filter(page_url.eq(id)).first::<Page>(db)?;

        let modules = Module::belonging_to(&filtered_page).load::<Module>(db)?;

        let categories: Vec<ModuleCategory> = Module::belonging_to(&filtered_page)
            .inner_join(category::table)
            .select(category::all_columns)
            .load::<ModuleCategory>(db)?;

        let module_array: Vec<(Vec<Module>, ModuleCategory)> = Module::belonging_to(&categories)
            .load::<Module>(db)?
            .grouped_by(&categories)
            .into_iter()
            .zip(categories)
            .collect::<Vec<_>>();

        let category_dtos: Vec<CategoryDTO> = module_array
            .iter()
            .map(|a| CategoryDTO {
                uuid: a.1.uuid.clone(),
                title: a.1.title.clone(),
                modules: a.0.clone().into_iter().map(|m| m.into()).collect(),
            })
            .collect::<Vec<_>>();

        let module_dto = FieldsDTO {
            modules: modules.into_iter().map(|m| m.into()).collect(),
            categories: Some(category_dtos),
        };

        Ok((filtered_page, module_dto))
    }
}
