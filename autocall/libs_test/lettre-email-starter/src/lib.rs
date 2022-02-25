extern crate lettre;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport, Address};

#[test]
fn test_o() {
  
    // let email = SendableEmail::new()
    //     .to(("450220020@qq.com", "Firstname Lastname"))
    //     .from("freedomour@163.com")
    //     .subject("Hi, Hello world")
    //     .text("Hello world.")
    //     .build()
    //     .unwrap();

        let email = Message::builder()
        .from("freedomour@163.com".parse().unwrap())
        .reply_to("freedomour@163.com".parse().unwrap())
        .to("450220020@qq.com".parse().unwrap())
        .subject("Hi, Hello world")
        .body(String::from("Be happy!")).unwrap();
    
    let creds = Credentials::new("freedomour".to_string(), "XJVQIQNWXBFXJLWP".to_string());
    
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();
    
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}