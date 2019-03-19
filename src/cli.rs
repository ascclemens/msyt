use clap::{App, AppSettings, Arg, SubCommand};

pub fn app<'a, 'b: 'a>() -> App<'a, 'b> {
  App::new(clap::crate_name!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
    .about(clap::crate_description!())

    .settings(&[
      AppSettings::SubcommandRequiredElseHelp,
      AppSettings::DeriveDisplayOrder,
      AppSettings::VersionlessSubcommands,
    ])

    .subcommand(SubCommand::with_name("import")
      .about("Import from MSYT files to MSBT files")

      .arg(Arg::with_name("dir_mode")
        .help("Allow specifying directories. msyt will search for all files with the correct extension in the provided directories.")
        .short("d")
        .long("directories")
        .alias("directory"))

      .arg(Arg::with_name("paths")
        .help("MSYT paths to import (MSBT files should be adjacent)")
        .required(true)
        .multiple(true)))
    .subcommand(SubCommand::with_name("export")
      .about("Export from MSBT files to MSYT files")

      .arg(Arg::with_name("dir_mode")
        .help("Allow specifying directories. msyt will search for all files with the correct extension in the provided directories.")
        .short("d")
        .long("directories")
        .alias("directory"))

      .arg(Arg::with_name("no-backup")
        .help("Do not create a backup of any existing output files")
        .short("B")
        .long("no-backup")
        .conflicts_with("backup"))

      .arg(Arg::with_name("extension")
        .help("The extension to use when exporting")
        .short("e")
        .long("extension")
        .alias("ext")
        .takes_value(true)
        .default_value("msbt"))

      .arg(Arg::with_name("paths")
        .help("MSBT paths to export")
        .required(true)
        .multiple(true)))
}
