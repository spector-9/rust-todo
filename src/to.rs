pub struct Todo {
    priority: i8,
    description: String,
}

impl Todo {
    pub fn from(inp_prio: i8, inp_desc: String) -> Self {
        Todo {
            priority: inp_prio,
            description: inp_desc,
        }
    }

    pub fn format_todo(&self, tag_print: bool) -> String {
        let formatted_todo_item: String =
            "[".to_owned() + &self.priority.to_string() + &"] " + &self.description;
        formatted_todo_item
    }
}
