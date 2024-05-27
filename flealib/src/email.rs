use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::Error;

pub struct EMail
{

}

impl EMail
{
    pub fn new() -> EMail
    {
        EMail
        {
        }
    }

    /// Sends an email
    /// # Arguments
    /// * `to` - Recipient
    /// * `from` - Sender
    /// * `subject` - Subject of the email
    /// * `body` - Body of the email
    /// * `smtp_user_name` - SMTP user name
    /// * `smtp_pass` - SMTP password
    /// * `smtp_host` - SMTP host
    /// # Returns
    /// * `Result<(), Error>` - Result of the operation
    /// # Example
    /// ```
    /// let email = EMail::new();
    /// email.send_email("Person <person@domain.tld>", "NoBody <nb@domain.tld>", "Hello", "Hello!", "smtp_user_name", "smtp_pass", "smtp_host");
    /// ```
    pub fn send_email(&self, to: &str, from: &str, subject: &str, body: &str, 
        smtp_user_name: &str, smtp_pass: &str, smtp_host: &str) -> Result<(), Error>
    {
        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(body))
            .unwrap();
    
        let creds = Credentials::new(smtp_user_name.to_owned(), smtp_pass.to_owned());
    
        // Open a remote connection to an email server
        let mailer = SmtpTransport::relay(smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        mailer.send(&email)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_email_send()
    {
        let email = EMail::new();
        let result = email.send_email("me <your_email>", "test <their_email>", 
            "Hello", "Hello! Does it work?", "user_name", "password", "smtp.hostname.com");

        //print the result
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
