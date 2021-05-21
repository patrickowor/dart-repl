//import extra modules
mod bracket;
mod help;

//importing standard modules
use std::io::{Write, stdin, stdout};
use std::fs::File;
use std::process::Command;
use std::{thread, time};


// creating a flush function
#[allow(unused_must_use)]
pub fn flusher(){
    stdout().flush();
}

// creating an input function so as to 
// reduce the code for inputing
fn input(val : &'static str)-> String{
    let mut cd = String::from("");
    print!("{}",val);
    flusher();
    stdin().read_line(&mut cd).unwrap();
    cd = cd.replace("\n", "");
    cd
}


//running main it returns an empty Result so file doesn't throw an error
fn main() -> std::io::Result<()>{
    //checking for dart
   Command::new("dart")
        .args(&["--version"])
         .spawn()
        .expect("sorry repl.dart not found");
    let millis = time::Duration::from_millis(150);
    thread::sleep(millis);
    
    
    
    println!("DART REPL 0.0.2 built on \nRust 1.38.0 frame and dart by owor Patrick \n .exit to cancel \n .clear to clear errors in memory and \n .help for help\n");
    
    // basic variables needed
    let mut old = String::new();
    let mut imp = String::new();
    let mut imp_now = String::from(" ");
    let mut _delete_data = true;
    //event loop
    loop {
        //recieving input
        let mut code = input("::: ");

        //if input is exit
        if code.as_str() == ".exit"{
            if _delete_data == true {
            Command::new("rm")
                    .args(&["repl.dart"])
                    .spawn()
                    .expect("sorry repl.dart not found");
            }
            break;
        }else if &code.split("").collect::<Vec<&str>>()[1] == &"." {
            //if input starts with .
            
            if code.as_str() == ".clear" {
                old = String::new();
                code = String::new();
            } else if code.as_str() == ".help"{
                {
                    loop {
                        let helper = input("[help] > ");
                        if helper.as_str() == ".exit"{
                        break;
                    }
                        help::help(helper);
                    }
                }
                code = String::from("");
            } else if code.as_str() == ".save"{
               _delete_data = false;
               println!("saving file in 'repl.dart'");
               code = String::from("");
            }
        }
        
        // checking to make sure delimeters are complete
        while bracket::check_brackets(&code){
            let new = input("..... ");
            if new == ".exit" {
                break;
            }
            {
            code.push_str(new.as_str());
            }
        }
        // making sure the data is not empty
        if code.as_str() == "" {
            continue;
        }
        // adding ; if it is absent
        if &";" != &code.split("").collect::<Vec<&str>>()[&code.split("").collect::<Vec<&str>>().len() - 2]{
            {
                code.push_str("; \n");
            }
        }
        
        // checking to see if code starts with import
        // and if it does moving it to new variable
        if code.split_whitespace().collect::<Vec<&str>>()[0] == "import"{
            imp_now.push_str(code.as_str());
            code = String::from("");
        }
        
        // creating repl.dart file
        let mut file = File::create("repl.dart")?;
        file.write_all(imp.as_bytes())?;
        file.write_all(imp_now.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(b"void main() { \n")?;
        file.write_all(old.as_bytes())?;
        file.write_all(code.as_bytes())?;
        file.write_all(b"}")?;
        
        // executing repl.dart file
        let output = Command::new("dart")
                        .arg("--enable-asserts")
                        .arg("repl.dart")
                        .output()
                        .expect("failed to execute process");
 
    
        // checking if codes start with assert
        if code.contains("assert(") {
            if code.trim().starts_with("assert"){
                code = String::from("");
            }
        }
        
        // checking if code start with print
        if code.contains("print(") {
            if code.trim().starts_with("print"){
                code = String::from("");
            }
        }
       
        // recieving output and printing it out
       if format!("{}",output.status) == String::from("exit code: 0") || format!("{}",String::from_utf8_lossy(&output.stdout)) != format!("") {
       
                print!("{}", String::from_utf8_lossy(&output.stdout));
                
                old.push_str(code.as_str());
                
                if imp_now.trim().starts_with("import") {
                    imp.push_str(imp_now.as_str());
                    imp_now = String::from(" ");
                }
                
           // returning error     
        } else {
            println!("status: {}", output.status);
            print!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            imp_now = String::from(" ");
        }
   }
   Ok(())
}
