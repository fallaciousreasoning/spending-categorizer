pub enum Category {
    Supermarket,
    Movies,
    Transport,
    Amazon,
    Books
}

type Categorizer = Box<Fn(String) -> Option<Category>>;

pub fn categorize(description : String, categorizers : Vec<Categorizer>) -> Vec<Category> {
     let mut result = Vec::new();

     for categorizer in categorizers {
        match categorizer(description.to_string()) {
            Some(category) => result.push(category),
            None => {}
        }
     }

     return result;
}

pub fn initDefaultCategorizers(to : Vec<Categorizer>) {
    to.push(Box(|s| -> Some(Category::Supermarket)));
}