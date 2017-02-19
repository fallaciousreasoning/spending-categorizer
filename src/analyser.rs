#[derive(Debug, Clone)]
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

pub fn initDefaultCategorizers() -> Vec<Categorizer> {
    let mut result : Vec<Categorizer> = vec![];

    result.push(containsMapping("NEW WORLD".to_string(), Category::Supermarket));
    result.push(containsMapping("COUNTDOWN".to_string(), Category::Supermarket));
    result.push(containsMapping("PAKNSAVE".to_string(), Category::Supermarket));

    result.push(containsMapping("KINDLE".to_string(), Category::Books));
    result.push(containsMapping("AMAZON".to_string(), Category::Amazon));

    return result;
}

pub fn containsMapping(c : String, category : Category) -> Categorizer {
    let func : Box<Fn(String) -> Option<Category>> = Box::new(move |desc| {
        if desc.contains(&c.clone()[..]) {
            return Some(category.clone());
        }
        return None;
    });
    return func;
}