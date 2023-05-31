pub fn handle_action() {
    println!("\n * Welcome to Kettle 🫖    * ");
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
    println!("  cli                                   : Launches the cli version of kettle");
    println!("");
    println!("- created with ❤️  by @saravenpi");
    println!("- https://github.com/saravenpi\n");
}

