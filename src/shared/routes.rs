pub enum Route {
    Root,
    Todos,
    TodoById,
    GetTodo
}

impl Route {
    pub fn path(&self) -> &'static str {
        match self {
            Route::Root => "/",
            Route::Todos => "/todos",
            Route::TodoById => "/todos/:id",
            Route::GetTodo => "/get-todo"
        }
    }
}