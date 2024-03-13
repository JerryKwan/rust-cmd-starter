use anyhow::{bail, Context};
use clap::{arg, Arg, ArgAction, ArgMatches, Command};

#[derive(Debug, PartialEq)]
pub struct CmdOneArgs {
    pub arg1: String,
    pub arg2: String,
}

#[derive(Debug, PartialEq)]
pub enum CmdOne {
    Command(CmdOneArgs),
}

impl CmdOne {
    pub fn build_cli() -> Command {
        Command::new("cmdone")
            .arg(
                Arg::new("arg1")
                    .default_value("arg1default")
                    .action(ArgAction::Set)
                    .help("Set arg1")
                    .index(1),
            )
            .arg(
                Arg::new("arg2")
                    .default_value("arg2default")
                    .action(ArgAction::Set)
                    .help("Set arg2")
                    .index(2),
            )
            .about("This is the first command")
    }

    pub fn parse_cli_args(matches: ArgMatches) -> anyhow::Result<Self> {
        let arg1 = matches.get_one::<String>("arg1").unwrap().to_string();
        let arg2 = matches.get_one::<String>("arg2").unwrap().to_string();
        println!("arg1 = {:?}, arg2 = {:?}", arg1, arg2);
        Ok(CmdOne::Command(CmdOneArgs { arg1, arg2 }))
    }

    pub fn execute(&self) {
        println!("Executing command one: {:?}", self);
    }
}

#[derive(Debug, PartialEq)]
pub struct CmdTwoArgs {
    pub arg1: String,
    pub arg2: String,
}

#[derive(Debug, PartialEq)]
pub enum CmdTwo {
    Command(CmdTwoArgs),
}

impl CmdTwo {
    pub fn build_cli() -> Command {
        Command::new("cmdtwo")
            .arg(
                Arg::new("arg1")
                    .default_value("arg1default")
                    .action(ArgAction::Set)
                    .help("Set arg1")
                    .index(1),
            )
            .arg(
                Arg::new("arg2")
                    .default_value("arg2default")
                    .action(ArgAction::Set)
                    .help("Set arg2")
                    .index(2),
            )
            .about("This is the first command")
    }

    pub fn parse_cli_args(matches: ArgMatches) -> anyhow::Result<Self> {
        let arg1 = matches.get_one::<String>("arg1").unwrap().to_string();
        let arg2 = matches.get_one::<String>("arg2").unwrap().to_string();
        println!("arg1 = {:?}, arg2 = {:?}", arg1, arg2);
        Ok(CmdTwo::Command(CmdTwoArgs { arg1, arg2 }))
    }

    pub fn execute(&self) {
        println!("Executing command two: {:?}", self);
    }
}

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    CmdOne(CmdOne),
    CmdTwo(CmdTwo),
}

impl CliCommand {
    pub fn build_cli() -> Command {
        Command::new("MyApp")
            .arg(
                Arg::new("level")
                    .short('l')
                    .long("level")
                    .default_value("info")
                    .action(ArgAction::Set)
                    .help("Set the log level"),
            )
            .subcommand(CmdOne::build_cli())
            .subcommand(CmdTwo::build_cli())
            .about("This is a CLI for MyApp")
    }
    pub fn parse_cli_args(mut matches: ArgMatches) -> anyhow::Result<Self> {
        let (subcommand, subcommandmatches) = matches
            .remove_subcommand()
            .context("Failed to parse command")?;
        match subcommand.as_str() {
            "cmdone" => Ok(CliCommand::CmdOne(CmdOne::parse_cli_args(
                subcommandmatches,
            )?)),
            "cmdtwo" => Ok(CliCommand::CmdTwo(CmdTwo::parse_cli_args(
                subcommandmatches,
            )?)),
            _ => bail!("Unknown subcommand: {}", subcommand),
        }
    }

    pub fn execute(&self) {
        match self {
            CliCommand::CmdOne(cmd) => cmd.execute(),
            CliCommand::CmdTwo(cmd) => cmd.execute(),
        }
    }
}
