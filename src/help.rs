
pub fn help(code : String){
    let url = "https://api.dart.dev";
    if code == "save" {
        println!("the .save key is used to save already written down code in the directory path");
    } else if code == "clear"{
        println!("type in .clear after an error occurs to clear it off the memory");
    } else if code == "about" {
        println!("this simple repl is created by owor Patrick and I am in no way the owner of the dart programming language or any of it tools");
    } else if code == "dart" {
        println!("dart is a new age object oriented programming language for building multi platform front end pages please refer to dart official website at {}",url);
    } else {
        println!("sorry wrong search keyword");
    }
}