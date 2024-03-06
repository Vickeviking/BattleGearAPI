use clap::value_parser;
use clap::Command;
use clap::Arg;

extern crate battle_gear as api_server;

#[tokio::main]
async fn main() {
    let matches = Command::new("BattleGear API")
        .about("BattleGear API")
        .version("0.1.0")
        .author("VickeViking")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("email").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(Arg::new("full_name").required(true))
                        .arg(Arg::new("country").required(true))
                        .arg(Arg::new("date_of_birth").required(true))
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                )
                .subcommand(
                    Command::new("list")
                        .about("List all users")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a user by ID")
                        .arg_required_else_help(true)
                        .arg(Arg::new("ID").required(true).value_parser(value_parser!(i32)))
                )
                .subcommand(
                    Command::new("delete_by_username")
                        .about("Delete a user by username")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                )
                
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", users_matches)) => {
            match users_matches.subcommand() {
                Some(("create", create_matches)) => {
                    api_server::commands::create_user(
                        create_matches.get_one::<String>("username").unwrap().to_owned(),
                        create_matches.get_one::<String>("email").unwrap().to_owned(),
                        create_matches.get_one::<String>("password").unwrap().to_owned(),
                        create_matches.get_one::<String>("full_name").unwrap().to_owned(),
                        create_matches.get_one::<String>("country").unwrap().to_owned(),
                        create_matches.get_one::<String>("date_of_birth").unwrap().to_owned(),
                        create_matches.get_many::<String>("roles").unwrap().map(|v| v.to_owned()).collect()
                    ).await;
                }
                Some(("list", _)) => {
                    api_server::commands::list_users().await;
                }
                Some(("delete", delete_matches)) => {
                    api_server::commands::delete_user(
                        delete_matches.get_one::<i32>("ID").unwrap().to_owned()
                    ).await;
                }
                Some(("delete_by_username", delete_by_username_matches)) => {
                    api_server::commands::delete_user_by_username(
                        delete_by_username_matches.get_one::<String>("username").unwrap().to_owned()
                    ).await;
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    }
}