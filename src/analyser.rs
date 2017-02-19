extern crate regex;

use std::collections::HashMap;

use statement;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Category {
    Supermarket,
    Movies,
    Transport,
    Amazon,
    Books
}

pub struct Analysis {
    categories : HashMap<Category, Vec<statement::StatementRow>>,
    all_categories : Vec<Category>,
    categorizers : Vec<Categorizer>,
}

impl Analysis {
    pub fn new(rows : &Vec<statement::StatementRow>, categorizers : Vec<Categorizer>) -> Analysis {
        let mut result = Analysis {
            all_categories: vec![],
            categories: HashMap::new(),
            categorizers: categorizers
        };

        for row in rows {
            result.add_statement(row);
        }

        return result;
    }

    fn set_row(&mut self, row : &statement::StatementRow, category : Category) {
        if !self.categories.contains_key(&category) {
            self.categories.insert(category.clone(), vec![]);
            self.all_categories.push(category.clone());
        }

        match self.categories.get_mut(&category) {
            Some(mut value) => value.push(row.clone()),
            None => {}
        }
    }

    fn add_statement(&mut self, row : &statement::StatementRow) {
        let categories : Vec<Option<Category>>;
        { 
            categories = self.categorizers.iter().map(|c| c(&row.description)).collect();
        }

        for category in categories {
            match category {
                Some(value) => self.set_row(row, value),
                None => {}
            }
        }
    }

    pub fn get_statements(&self, category : &Category) -> Option<&Vec<statement::StatementRow>> {
        if !self.categories.contains_key(category) {
            return None;
        }

        return Option::Some(&self.categories[category]);
    }

    pub fn total_spent(&self, category : &Category) -> f32 {
        let sum : f32 = self.get_statements(category).unwrap().iter().map(|r| r.total_amount).sum();
        return sum;
    }

    pub fn all(&self) -> &HashMap<Category, Vec<statement::StatementRow>> {
        return &self.categories;
    }

    pub fn all_categories(&self) -> &Vec<Category> {
        return &self.all_categories;
    }
}

type Categorizer = Box<Fn(&String) -> Option<Category>>;


pub fn default_categorizers() -> Vec<Categorizer> {
    let mut result : Vec<Categorizer> = vec![];

    result.push(regex_mapping(r"(NEW WORLD)|(COUNTDOWN)|(PAKNSAVE)", Category::Supermarket));

    result.push(contains_mapping("KINDLE", Category::Books));
    result.push(contains_mapping("AMAZON", Category::Amazon));

    return result;
}

pub fn contains_mapping(c : &str, category : Category) -> Categorizer {
    let string = c.to_string();

    let func : Categorizer = Box::new(move |desc| {
        if desc.contains(&string[..]) {
            return Some(category.clone());
        }
        return None;
    });
    return func;
}

pub fn regex_mapping(res : &str, category : Category) -> Categorizer {
    let re = regex::Regex::new(res).unwrap();
    let func : Categorizer = Box::new(move |desc| {
        if re.is_match(&desc[..]) {
            return Some(category.clone());
        }
        return None;
    });
    return func;
}