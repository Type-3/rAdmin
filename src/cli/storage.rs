use crate::modules::CliModule;
use crate::ServerError;
use termion::{color, style};
use clap::{App, AppSettings, ArgMatches, SubCommand};

pub struct Storage<'a>(&'a crate::Application);

impl<'a> Storage<'a> {
    pub fn new(modules: &'a crate::Application) -> Storage {
        Storage(modules)
    }
}

impl<'a> CliModule for Storage<'a> {
    fn arg(&self) -> Option<String> {
        Some("storage".into())
    }

    fn app(&self) -> Option<App<'static, 'static>> {
        Some(
            SubCommand::with_name("storage")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("reset"))
                .subcommand(SubCommand::with_name("info")),
        )
    }

    fn handle(&self, matches: Option<&ArgMatches>) -> Result<(), ServerError> {
        match matches {
            None => unreachable!(),
            Some(matches) => match matches.subcommand() {
                ("reset", matches) => self.handle_reset_command(matches)?,
                ("info", matches) => self.handle_info_command(matches)?,
                _ => {}
            },
        }
        Ok(())
    }
}

impl<'a> Storage<'a> {
    fn handle_reset_command(&self, _: Option<&ArgMatches>) -> Result<(), ServerError> {
        let data_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "data".into());
        for module in &self.0.modules.0 {
            for (path, extension) in module.storage() {
                if let Ok(files) = std::fs::read_dir(format!("{}/{}", data_path, path)) {
                    for entry in files {
                        let entry = entry?;
                        let entry_path = entry.path();
                        let ext = entry_path.extension().map(|item| item.to_str().unwrap());
                        if let Some(ext) = ext {
                            if extension.contains(&ext) {
                                std::fs::remove_file(entry.path())?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_info_command(&self, _: Option<&ArgMatches>) -> Result<(), ServerError> {
        let data_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "data".into());
        println!(
            "{}{}Storage Path{}: {}",
            color::Fg(color::Green),
            style::Italic,
            style::Reset,
            data_path
        );
        for module in &self.0.modules.0 {
            let paths = module.storage();
            if paths.len() > 0 {
                println!(
                    "    {}{}({}){}",
                    color::Fg(color::Green),
                    module.identifier(),
                    module.version(),
                    color::Fg(color::Reset)
                );
            }
            for (path, extension) in module.storage() {
                println!(
                    "        {}->{} {}{}Path{}{}: {}{}{}",
                    color::Fg(color::Blue),
                    color::Fg(color::Reset),
                    style::Italic,
                    color::Fg(color::Magenta),
                    style::Reset,
                    color::Fg(color::Reset),
                    color::Fg(color::Cyan),
                    path,
                    color::Fg(color::Reset)
                );
                println!(
                    "        {}->{} {}{}Extensions{}: {}{:?}{}{}",
                    color::Fg(color::Blue),
                    color::Fg(color::Reset),
                    style::Italic,
                    color::Fg(color::Magenta),
                    color::Fg(color::Reset),
                    color::Fg(color::Cyan),
                    extension,
                    color::Fg(color::Reset),
                    style::Reset,
                );
            }
        }
        Ok(())
    }
}
