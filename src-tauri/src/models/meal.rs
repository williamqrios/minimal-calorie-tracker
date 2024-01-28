use sqlx::FromRow; 
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, FromRow)] 
pub struct Meal {
    pub id: i32, 
    pub meal_type: String, 
}

impl Meal {
    pub fn new(id: i32, meal_type: String) -> Self {
        Self { id, meal_type  }
    }
}