use std::fs;
use std::io::Error;

fn extract_errors(text:&str)->Vec<String>{
    let split_text=text.split("\n");

    let mut results=vec![];

    for line in split_text{
        if line.starts_with("ERROR"){
            results.push(line.to_string())
        }
    }

    results
}


fn main() {
    match fs::read_to_string("logs.txt"){
        Ok(text_that_was_read)=>{
            let error_logs=extract_errors(text_that_was_read.as_str());
            match fs::write("errors.txt",error_logs.join("\n")){
                Ok(())=>println!("Wrote errors"),
                Err(reason_write_failed)=>{
                    println!("Writing of errors.txt failed:{}",reason_write_failed)
                }
            }
        }
        Err(why_this_failed)=>{
            println!("Failed to read:{}",why_this_failed);
        }
    }

    //println!({},error_logs)

    // string_test(String::from("Red"),&String::from("Red"),String::from("Red").as_str());
    // // println!("{:#?}",text);
}


// fn validate_email(email:String)->Result<(),Error>{
//     if email.contains("@"){
//         Ok(())
//     }
//     else{
//         Err(Error::other("Emails must have an @"))
//     }
// }

// fn divide(a:f64,b:f64)->Result<f64,Error>{
//     if b==0.0{
//         Err(Error::other("Divide by 0 error"))
//     }else{
//         Ok(a/b)
//     }
// }
