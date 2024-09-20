use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs
};


fn main() {
   check_directories();
   check_file_permissions();
  
}

fn set_up_stream(){
    let listener = TcpListener::bind("127.0.0.1:7878")
    .unwrap();

    for request in listener.incoming() {
        let request = request.unwrap();
        handle_request(request);
    }
}

fn handle_request(mut request: TcpStream){
    println!("Received {:?}", request);
}

fn check_directories() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())

}

fn check_file_permissions(){
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    println!("{:?}: {:?}", entry.path(), metadata.permissions());
                } else {
                    println!("Could not get metadata for {:?}", entry)
                }
            }
        }
    }


}