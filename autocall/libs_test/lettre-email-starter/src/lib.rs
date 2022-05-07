extern crate lettre;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};


pub fn switch_for_eamil(emial_str:String)->Option<Vec<String>>{
    let from_email_str  =  emial_str;
    match from_email_str.find("@") {
        Some(i)=>{
            let email_tag_last = from_email_str.split_at(i+1).1;
            let email_tag = email_tag_last.split_at(email_tag_last.rfind(".").unwrap_or(0)).0 ;
            match email_tag {
                "163"=>{
                    Some(vec!(email_tag.to_string(),"smtp.163.com".into(),"imap.163.com".into(),"pop.163.com".into()))
                }
                _=>{
                    None
                }
            }
        }
        None=>{
            None
        }
    }
}

pub fn send(){
    let from_email_str  =  "";
    let to_email_str = "";
    let eamil_opt = switch_for_eamil(from_email_str.into());
    

    let email = Message::builder()
    .from("freedomour@163.com".parse().unwrap())
    .reply_to("freedomour@163.com".parse().unwrap())
    .to("450220020@qq.com".parse().unwrap())
    .subject("Hi, Hello world")
    .body(String::from("Be happy!")).unwrap();

    let creds = Credentials::new("freedomour@163.com".to_string(), "XJVQIQNWXBFXJLWP".to_string());
    let mailer = SmtpTransport::relay ("smtp.163.com").unwrap()

    .credentials(creds)
    //.authentication(vec!(Mechanism::Plain))
    .build();
    

// Send the email
match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {:?}", e),
}
}

#[test]
fn test_o() {
  
        let email = Message::builder()
        .from("freedomour@163.com".parse().unwrap())
        .reply_to("freedomour@163.com".parse().unwrap())
        .to("450220020@qq.com".parse().unwrap())
        .subject("Hi, Hello world")
        .body(String::from("Be happy!")).unwrap();
    //XJVQIQNWXBFXJLWP
    //LnExcTEyMzQ1Ng==
    //freedomour@163.com
    let creds = Credentials::new("freedomour@163.com".to_string(), "XJVQIQNWXBFXJLWP".to_string());
    
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay ("smtp.163.com").unwrap()

        .credentials(creds)
        //.authentication(vec!(Mechanism::Plain))
        .build();
        
    
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}


fn test_a() {
  



        let email = Message::builder()
        .from("freedomour@163.com".parse().unwrap())
        .reply_to("freedomour@163.com".parse().unwrap())
        .to("450220020@qq.com".parse().unwrap())
        .subject("Hi, Hello world")
        .body(String::from("Be happy!")).unwrap();
    //XJVQIQNWXBFXJLWP
    //LnExcTEyMzQ1Ng==
    //freedomour@163.com
    let creds = Credentials::new("freedomour@163.com".to_string(), ".q1q123456".to_string());
    
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay ("smtp.163.com").unwrap()

        .credentials(creds)
        //.authentication(vec!(Mechanism::Plain))
        .build();
        
        let sender = SendmailTransport::new();
        let result = sender.send(&email);
    // Send the email
    match result {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}