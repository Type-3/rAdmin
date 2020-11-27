use radmin::{
    crate_authors, crate_description, crate_name, crate_version, Application, ServerError,
};
use radmin::modules::Modules;
use radmin::acl::{AclModule, AclModuleConfig};

fn main() -> Result<(), ServerError> {
    let app = Application::default()
        .name(crate_name!())
        .version(crate_version!())
        .description(crate_description!())
        .author(crate_authors!());
    radmin::run(app)
}
