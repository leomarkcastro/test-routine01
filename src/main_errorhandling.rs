#![deny(clippy::all)]

fn get_first_name() -> Result<String, String> {
    Ok("John".to_string())
    // Err("I dont know the first name".to_string())
}

fn get_last_name() -> Result<String, String> {
    Ok("Smith".to_string())
    // Err("I dont know the last name".to_string())
}

fn get_name() -> Result<String, String> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn main() {
    let value = get_name();

    // let length = value.map(|n| n.len()).unwrap_or_default();
    // println!("{}", length)

    // match value {
    //     Ok(v) => {
    //         println!("{}", v);
    //     }
    //     Err(_) => {
    //         println!("Eror Happened");
    //     }
    // }

    let errlength = value.map_err(|n| n.len());
    println!("{:?}", errlength);
}
