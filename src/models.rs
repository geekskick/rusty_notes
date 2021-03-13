use super::schema::lists;

#[derive(Insertable, Debug)]
#[table_name="lists"]
pub struct NewItem{
    pub title:String,
    pub detail:String
}

impl NewItem{
    pub fn new<'a>(title:&'a str, detail:&'a str)->NewItem{
        NewItem{
            title: String::from(title),
            detail:String::from(detail)
        }
    }
}


#[derive(Queryable, Debug)]
pub struct TodoItem{
    pub id: i32,
    pub title: String,
    pub detail: String,
    pub done: bool
}

impl From<NewItem> for TodoItem{
    fn from(i: NewItem)->TodoItem{
        TodoItem{
            id: 0, 
            title:i.title, 
            detail:i.detail, 
            done:false
        }
    }
}


