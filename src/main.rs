use std::env;
use std::fs;
use std::process::Command;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match env::var("LOCALAPPDATA") {
        Ok(appdata) => {
            let path = appdata + "\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\RoamingState\\profiles.json";
            match fs::read_to_string(&path) {
                Ok(data) => {
                   let res = serde_json::from_str(data.as_str());
                   if res.is_ok() {
                       let mut data: Value = res.unwrap();
                       //update config
                       data["profiles"][0]["startingDirectory"] = Value::String(args.join(" "));
                       //write new data to file
                       match fs::write(path, serde_json::to_string_pretty(&data).unwrap()) {
                           Ok(data) => {
                               println!("successfull, {:?}", data);
                               Command::new("wt.exe").spawn().expect_err("windows terminal could nit start");
                           },
                           Err(e) => println!("Error: {}", e)
                       };
                   } else {
                       println!("Could not parse JSON");
                   }
                },
                Err(e) => println!("Couldn't read profiles.json. error message: {}", e)
            };
        },
        Err(e) => println!("Couldn't read this variable. error message: {}", e)
    };
}
