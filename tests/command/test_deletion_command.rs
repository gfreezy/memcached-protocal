use memcached_protocal::DeletionCommand;


#[test]
fn test_parse_with_noreply() {
    let data = "delete key 1\r\n".as_bytes();

    let cmd1 = DeletionCommand::parse(&data).unwrap();
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
    let data = "delete key\r\n".as_bytes();

    let cmd1 = DeletionCommand::parse(&data).unwrap();
    assert_eq!(cmd1,
  	 	DeletionCommand{
    		command_name: "delete".to_owned(),
    		key: "key".to_owned(),
    		noreply: None,
    	}
  	);
}
