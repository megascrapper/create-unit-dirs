use std::process::exit;
use std::fs::create_dir_all;
use std::path::PathBuf;

const ERROR_MSG: &str = "Usage: ./create_unit_dirs [year] [units]";
const EXAMREV_NAME: &str = "Exam revision";

fn create_directories(year: &String, units: Vec<String>) -> std::io::Result<()>{
    let base_path = std::env::current_dir()?;
    for unit in units.iter() {
        let mut unit_path = PathBuf::from(&base_path);
        unit_path.push(year);
        unit_path.push(unit.to_ascii_uppercase());
        for week in 0..=12 {
            let mut week_path = PathBuf::from(&unit_path);
            week_path.push(format!("Week {}", week));
            create_dir_all(week_path)?;
        }
        unit_path.push(EXAMREV_NAME);
        create_dir_all(unit_path)?;
    }
    Ok(())
}

fn main()  {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() >= 3 {
            if argv[1].parse::<u32>().is_ok() {
                if let Err(e) = create_directories(&argv[1], argv[2..].to_vec()) {
                    eprintln!("{}", e);
                    exit(1)
                }
            } else {
                eprintln!("{}", ERROR_MSG);
                exit(1);
            }
    } else {
        eprintln!("{}", ERROR_MSG);
        exit(1);
    }
}
