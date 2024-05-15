use clap::{Arg, ArgAction, Command};
use constants::*;
use std::process;
mod constants;

fn main() {
  let autocomplete: bool = false;
  let bits: bool = false;
  let cols: u8 = COLS_SIZE;
  let ebcdic: bool = false;
  let group_bytes: bool = false;
  let help: bool = false;
  let include: bool = false;
  let len: bool = false;
  let style: bool = false;
  let revert: bool = false;
  let seek: bool = false;
  let offset: u32 = OFFSET_DEFAULT;
  let upper: bool = false;
  let version: bool = false;

  let cmd = Command::new(env!("CARGO_CRATE_NAME"))
    .about("make a hexdump or do the reverse")
    .version(VERSION_STRING)
    .arg_required_else_help(true)
    .multicall(false)
    // Sync subcommand
    //
    // Only a few of its arguments are implemented below.
    .subcommand(
      Command::new("sync")
        .short_flag('S')
        .long_flag("sync")
        .about("Synchronize packages.")
        .arg(
          Arg::new("search")
            .short('s')
            .long("search")
            .conflicts_with("info")
            .action(ArgAction::Set)
            .num_args(1..)
            .help("search remote repositories for matching strings"),
        )
        .arg(
          Arg::new("info")
            .long("info")
            .conflicts_with("search")
            .short('i')
            .action(ArgAction::SetTrue)
            .help("view package information"),
        )
        .arg(
          Arg::new("package")
            .help("packages")
            .required_unless_present("search")
            .action(ArgAction::Set)
            .num_args(1..),
        ),
    );
  match cmd.get_matches().subcommand() {
    Some(("sync", sync_matches)) => {
      if sync_matches.contains_id("search") {
        let packages: Vec<_> = sync_matches
          .get_many::<String>("search")
          .expect("contains_id")
          .map(|s| s.as_str())
          .collect();
        let values = packages.join(", ");
        println!("Searching for {values}...");
        return;
      }

      let packages: Vec<_> = sync_matches
        .get_many::<String>("package")
        .expect("is present")
        .map(|s| s.as_str())
        .collect();
      let values = packages.join(", ");

      if sync_matches.get_flag("info") {
        println!("Retrieving info for {values}...");
      } else {
        println!("Installing {values}...");
      }
    }
    Some(("query", query_matches)) => {
      if let Some(packages) = query_matches.get_many::<String>("info") {
        let comma_sep =
          packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        println!("Retrieving info for {comma_sep}...");
      } else if let Some(queries) = query_matches.get_many::<String>("search") {
        let comma_sep =
          queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        println!("Searching Locally for {comma_sep}...");
      } else {
        println!("Displaying all locally installed packages...");
      }
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
  }

  // Sync subcommand
  // .subcommand(
  //   Command::new("cols")
  //     .short_flag('c')
  //     .long_flag("cols")
  //     .about("Synchronize packages.")
  //     .arg(
  //       Arg::new("search")
  //         .short('s')
  //         .long("search")
  //         .conflicts_with("info")
  //         .action(ArgAction::Set)
  //         .num_args(1..)
  //         .help("search remote repositories for matching strings"),
  //     )
  //     .arg(
  //       Arg::new("info")
  //         .long("info")
  //         .conflicts_with("search")
  //         .short('i')
  //         .action(ArgAction::SetTrue)
  //         .help("view package information"),
  //     )
  //     .arg(
  //       Arg::new("package")
  //         .help("packages")
  //         .required_unless_present("search")
  //         .action(ArgAction::Set)
  //         .num_args(1..),
  //     ),
  // );

  // match cmd.get_matches().subcommand_name() {
  //     Some("hostname") => println!("www"),
  //     Some("dnsdomainname") => println!("example.com"),
  //     _ => unreachable!("parser should ensure only valid subcommand names are used"),
  // }
  process::exit(EXIT_SUCCESS);
}

// fn main() {

//   let cmd_line = std::env::args();
//   for arg in cmd_line {
//       println!("{}", arg);
//   }

//     println!("Hello, world!");
//
// }
