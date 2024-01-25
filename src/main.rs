use std::fs;
use menu_genie::{MenuAction, MenuBuilder, MgErrorKind};
pub mod airport;

fn main() {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    fs::create_dir_all("./data").expect("Failed to create");
    
    println!("FAA NASR Data Processor v{}", VERSION.unwrap_or("Unknown"));
    main_menu();
}

fn main_menu() {
    let mut menu = MenuBuilder::new()
        .with_menu(1)
        .with_menu_item("Reread Data", MenuAction::Nothing)
        .with_menu_item("Export airport list", MenuAction::Nothing)
        .with_menu_item("Generate airport changes list", MenuAction::Nothing)
        .with_menu_item("Generate TEC Route changes list", MenuAction::Nothing)
        .with_quit_button()
        .build();

    loop {
        match menu.prompt() {
            Ok(tuple) => match tuple {
                (1, 1) => println!("Re read data"),
                (1, 2) => println!("Export Airport list"),
                (1, 3) => println!("Generate airport changes list"),
                (1, 4) => println!("Generate TEC Route changes"),
                _ => (),
            },

            Err(e) => {
                println!("{e}");
                match e.kind() {
                    MgErrorKind::MissingMenu(_) | MgErrorKind::MissingMenuItem(..) => break,
                    _ => (),
                }
            }
        } 
    }
}