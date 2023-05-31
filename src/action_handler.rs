mod delete_action;
mod exclude_action;
mod include_action;
mod init_action;
mod list_action;
mod save_action;
mod use_action;
mod help_action;
mod cli_action;

fn check_default(unwraped_data: &str, message: &str) -> bool
{
    if unwraped_data == "" {
        println!("{message}");
        return false;
    } else {
        return true;
    }
}

fn init_pre(action_args: &mut std::env::Args, kettle_repo_path: &str,
            kettle_name_warning: &str)
{
    let kettle_name = action_args.nth(0).unwrap_or_default();
    if check_default(&kettle_name, kettle_name_warning) {
        init_action::handle_action(&kettle_name, kettle_repo_path);
    }
}
fn delete_pre(action_args: &mut std::env::Args, kettle_name_warning: &str, kettle_repo_path: &str )
{
    let kettle_name = action_args.nth(0).unwrap_or_default();
    if check_default(&kettle_name, kettle_name_warning) {
        delete_action::handle_action(&kettle_name, kettle_repo_path);
    }
}

fn include_pre(args: &mut std::env::Args, file_name_warning: &str)
{
    let mut first_file_name = args.nth(0).unwrap_or_default();
    if first_file_name == "-r" {
        first_file_name = args.nth(0).unwrap_or_default();
        if check_default(&first_file_name, file_name_warning) {
            let other_files = args;
            include_action::handle_action(&first_file_name,
                                          other_files, 1);
        }
    } else {
        if check_default(&first_file_name, file_name_warning) {
            let other_files = args;
            include_action::handle_action(&first_file_name,
                                          other_files, 0);
        }
    }
}

fn exclude_pre(args: &mut std::env::Args, file_name_warning: &str)
{
    let file_name = args.nth(0).unwrap_or_default();

    if check_default(&file_name, file_name_warning) {
        exclude_action::handle_action(&file_name);
    }
}

fn use_pre(action_args: &mut std::env::Args, kettle_name_warning: &str,
              kettle_repo_path: &str)
{
    let kettle_name = action_args.nth(0).unwrap_or_default();

    if check_default(&kettle_name, kettle_name_warning) {
        let dest_folder = action_args.nth(0).unwrap_or_default();
        if check_default(&dest_folder,
                         "⚠️  No destination folder was given") {
            use_action::handle_action(&kettle_name, &dest_folder,
                                      kettle_repo_path);
        }
    }
}

pub fn handle_action(args: &mut std::env::Args, kettle_repo_path: &str)
{
    let action = &args.nth(1).unwrap_or_default()[..];
    let action_args = args.into_iter();
    let kettle_name_warning = "⚠️  No kettle name was given";
    let file_name_warning = "⚠️  No file name was  given";

    match action {
        "" => println!("Welcome to Kettle, use -h for all the commands"),
        "save" => save_action::handle_action(kettle_repo_path),
        "delete" => delete_pre(action_args, kettle_name_warning, kettle_repo_path),
        "init" => init_pre(action_args, kettle_repo_path, kettle_name_warning),
        "list" => list_action::handle_action(kettle_repo_path),
        "include" => include_pre(args, file_name_warning),
        "exclude" => exclude_pre(args, file_name_warning),
        "use" => use_pre(action_args, kettle_name_warning, kettle_repo_path),
        "cli" => cli_action::handle_action(),
        "-h" | "--help" | "-help" | "help" => help_action::handle_action(),
        _ => println!("This command was not found, use -h for all the commands"),
    };
}
