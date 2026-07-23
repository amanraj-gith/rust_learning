#[derive(Debug)]
// enum IpAdd { 
//     V4(u8,u8,u8,u8),
//     V6 (String),
// }

enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
fn call(&self){
    // Self::Write(String::from("It has run!"));
    println!("Method called!");
}

    fn test(){
        let x = Message::Write(String::from("Hello, Boss!"));
        x.call();
    //    println!("{:?}",y)
    }
}

fn main(){
    // let ip_addr_kind = IpAdd::V4(0,255,255,0);

    // println!("{:#?}",ip_addr_kind);

    Message::test();
}
