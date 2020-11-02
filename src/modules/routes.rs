pub trait RoutesModule {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)>;
}

pub struct NullRoutesModule;

impl RoutesModule for NullRoutesModule {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)> {
        vec![]
    }
}
