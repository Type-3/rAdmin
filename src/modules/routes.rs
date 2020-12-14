use rocket::Catcher;

pub trait RoutesModule {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)>;

    fn catch(&self) -> Vec<Catcher> {
        vec![]
    }
}

pub struct NullRoutesModule;

impl RoutesModule for NullRoutesModule {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)> {
        vec![]
    }
}
