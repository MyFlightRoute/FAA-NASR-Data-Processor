use std::{fs::self, time::Duration};
use menu_genie::{MenuAction, MenuBuilder, MgErrorKind};

pub mod airport;
pub mod preferential_route;

const ONE_SECOND:Duration = Duration::from_secs(1);
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    fs::create_dir_all("./data").expect("Failed to create");
    
    println!("FAA NASR Data Processor v{}", VERSION.unwrap_or("Unknown"));
    main_menu();
}

fn main_menu() {
    let mut menu = MenuBuilder::new()
        .with_menu(1)
        .with_menu_item("Export airport list", MenuAction::Nothing)
        .with_menu_item("Generate airport changes list", MenuAction::Nothing)
        .with_menu_item("Generate TEC Route changes list", MenuAction::Nothing)
        .with_menu_item("Export TEC Routes", MenuAction::Nothing)
        .with_quit_button()
        .build();

    loop {
        match menu.prompt() {
            Ok(tuple) => match tuple {
                (0, 0) => std::process::exit(0), // Quit option
                (1, 1) => airport::export_airport_list(),
                (1, 2) => airport::generate_airport_changes(),
                (1, 3) => preferential_route::generate_tec_route_changes(),
                (1, 4) => preferential_route::generate_mfr_tec_route_list(),
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