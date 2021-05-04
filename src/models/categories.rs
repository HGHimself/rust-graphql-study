use crate::schema::categories;
use crate::utils::now;
use diesel::prelude::*;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct Category {
    pub id: i32,
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub position: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub ads: Option<String>,
    pub deleted_at: Option<String>,
    pub image: Option<String>,
    pub active: Option<i32>,
    pub slug: Option<String>,
    pub product_recommendation_id: Option<String>,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub position: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub ads: Option<String>,
    pub deleted_at: Option<String>,
    pub image: Option<String>,
    pub active: Option<i32>,
    pub slug: Option<String>,
    pub product_recommendation_id: Option<String>,
}


impl NewCategory {
    pub fn new(
        name: Option<String>,
        slug: Option<String>,
        parent_id: Option<i32>,
        position: Option<i32>,
        image: Option<String>,
        ads: Option<String>,
        product_recommendation_id: Option<String>,
    ) -> Self {
        NewCategory {
            name,
            slug,
            parent_id,
            position,
            image,
            ads,
            created_at: Some(now()),
            updated_at: None,
            deleted_at: None,
            active: Some(1),
            product_recommendation_id,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> Category {
        create(conn, self)
    }
}

pub fn create(conn: &PgConnection, new_category: &NewCategory) -> Category {
    diesel::insert_into(categories::table)
        .values(new_category)
        .get_result(conn)
        .expect("Error saving new category")
}

pub fn read(conn: &PgConnection) -> Vec<Category> {
    categories::table
        .load::<Category>(conn)
        .expect("Error loading category")
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection_test;

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(categories::table).execute(conn).unwrap();
    }

    fn mock_struct() -> NewCategory {
        NewCategory::new(
            Some("Cool Beans Category".to_string()),
            Some("unique asf".to_string()),
            Some(1),
            Some(1),
            Some("hhtps://domain:port/path.png".to_string()),
            Some("ads".to_string()),
            Some("product_recommendation_id".to_string()),
        )
    }

    #[test]
    fn it_creates_a_category() {
        let conn = establish_connection_test();

        create(&conn, &mock_struct());

        let categories = categories::table
            .load::<Category>(&conn)
            .expect("Error loading category");

        assert_eq!(1, categories.len());

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_category() {
        let conn = establish_connection_test();

        let new_category = mock_struct();

        let created_category = diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result::<Category>(&conn)
            .expect("Error saving new category");

        let categories = read(&conn);

        assert!(0 < categories.len());

        let my_category = categories.iter().find(|&x| x.name == new_category.name);
        assert!(
            my_category.is_some(),
            "Could not find the created category in the database!"
        );

        cleanup_table(&conn);
    }
}
