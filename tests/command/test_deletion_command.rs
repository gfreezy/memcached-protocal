use std::io::BufReader;
use memcached_protocal::DeletionCommand;


#[test]
fn test_parse_with_noreply() {
    let mut data = BufReader::new("delete key 1\r\n".as_bytes());

    let cmd1 = DeletionCommand::parse(&mut data).unwrap();
    assert_eq!(cmd1,
  	 	DeletionCommand{
    		command_name: "delete".to_owned(),
    		key: "key".to_owned(),
    		noreply: Some("1".to_owned()),
    	}
  	);
}

#[test]
fn test_parse_without_noreply() {
    let mut data = BufReader::new("delete key\r\n".as_bytes());

    let cmd1 = DeletionCommand::parse(&mut data).unwrap();
    assert_eq!(cmd1,
  	 	DeletionCommand{
    		command_name: "delete".to_owned(),
    		key: "key".to_owned(),
    		noreply: None,
    	}
  	);
}
