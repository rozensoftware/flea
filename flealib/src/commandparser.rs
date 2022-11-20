extern crate roxmltree;

const COMMAND_TAG_NAME: &'static str = "Command";
const COMMAND_ATTRIBUTE_NAME: &'static str = "name";
const COMMAND_VALUE_NAME: &'static str = "value";

pub struct CommandParser
{

}

impl CommandParser
{
    pub fn get_command(&self, xml: &str) -> Result<(String, String), &'static str>
    {        
        if let Ok(doc) = roxmltree::Document::parse(&xml)
        {
            if let Some(elem) = doc.descendants().find(|n| n.tag_name().name() == COMMAND_TAG_NAME)
            {
                return Ok((elem.attribute(COMMAND_ATTRIBUTE_NAME).unwrap_or("'name' attribute not exists.").to_owned(), 
                    elem.attribute(COMMAND_VALUE_NAME).unwrap_or("'value' attribute not exists.").to_owned()));        
            }
        }
        
        if let Err(e) = roxmltree::Document::parse(xml)
        {
            println!("{}", e);
        }

        Err("Command xml corrupted")
    }    
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]    
    fn get_command_test()
    {
        let c = CommandParser{};

        let res = c.get_command("<Command name='test' value='123'></Command>");
        assert!(res.is_ok());
        assert!(res.unwrap() == ("test".to_string(), "123".to_string()));
        
        let res = c.get_command("<Command2 name='test'></Command>");
        assert!(res.is_err());
    }
}