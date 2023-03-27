mod delete_action;
mod exclude_action;
mod include_action;
mod init_action;
mod list_action;
mod save_action;
mod use_action;

fn check_default(unwraped_data: &str, message: &str) -> bool {
    if unwraped_data == "" {
        println!("{message}");
        return false;
    } else {
        return true;
    }
}

pub fn handle_action(args: &mut std::env::Args, kettle_repo_path: &str) {
    let action = &args.nth(1).unwrap_or_default()[..];
    let action_args = args.into_iter();
    let kettle_name_warning = "⚠️  No kettle name was given";
    let file_name_warning = "⚠️  No file name was  given";
    match action {
        "" => println!("Welcome to Kettle, use -h for all the commands"),
        "save" => save_action::handle_action(kettle_repo_path),
        "delete" => {
            let kettle_name = action_args.nth(0).unwrap_or_default();
            if check_default(&kettle_name, kettle_name_warning) {
                delete_action::handle_action(&kettle_name, kettle_repo_path);
            }
        }
        "init" => {
            let kettle_name = action_args.nth(0).unwrap_or_default();
            if check_default(&kettle_name, kettle_name_warning) {
                init_action::handle_action(&kettle_name, kettle_repo_path);
            }
        }
        "list" => list_action::handle_action(kettle_repo_path),
        "include" => {
            let mut first_file_name = args.nth(0).unwrap_or_default();
            if first_file_name == "-r" {
                first_file_name = args.nth(0).unwrap_or_default();
                if check_default(&first_file_name, file_name_warning) {
                    let other_files = args;
                    include_action::handle_action(&first_file_name, other_files, 1);
                }
            } else {
                if check_default(&first_file_name, file_name_warning) {
                    let other_files = args;
                    include_action::handle_action(&first_file_name, other_files, 0);
                }
            }
        }
        "exclude" => {
            let file_name = args.nth(0).unwrap_or_default();
            if check_default(&file_name, file_name_warning) {
                exclude_action::handle_action(&file_name);
            }
        }
        "use" => {
            let kettle_name = action_args.nth(0).unwrap_or_default();
            if check_default(&kettle_name, kettle_name_warning) {
                let destination_folder = action_args.nth(0).unwrap_or_default();
                if check_default(&destination_folder, "⚠️  No destination folder was given") {
                    use_action::handle_action(&kettle_name, &destination_folder, kettle_repo_path);
                }
            }
        }
        "-h" | "--help" | "-help" | "help" => {
            println!(" * Welcome to Kettle 🫖    * ");
            println!(" * the boilerplate manager *");
            println!("");
            println!("COMMANDS:");
            println!("  help, --h, --help                      : Shows this message");
            println!("  init <kettle_name>                     : Initialises a kettle in your current folder");
            println!("  include <file_name>                    : Includes the given file to the recipe.json file");
            println!("  exclude <file_name>                    : Excludes the given file from the recipe.json file");
            println!(
                "  save                                   : Saves the kettle to the kettle repo"
            );
            println!(
                "  delete <kettle_name>                   : Deletes a kettle from the kettle repo"
            );
            println!("  use <kettle_name> <destination_folder> : Import a kettle to the destination_folder");
            println!("  list                                   : Lists all the kettles in the kettle repo");
            println!("");
            println!("- created with ❤️  by @saravenpi");
            println!("- https://github.com/saravenpi\n");
        }
        _ => println!("This command was not found, use -h for all the commands"),
    };
}
