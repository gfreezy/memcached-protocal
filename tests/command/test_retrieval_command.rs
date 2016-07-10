use memcached_protocal::RetrievalCommand;


#[test]
fn test_parse_get() {
    let data = "get key\r\n".as_bytes();

    let cmd1 = RetrievalCommand::parse(&data).unwrap();
    assert_eq!(cmd1,
  	 	RetrievalCommand{
    		command_name: "get".to_owned(),
    		keys: vec!["key".to_owned()],
    	}
  	);
}

#[test]
fn test_parse_gets() {
    let data = "gets key1 key2\r\n".as_bytes();

    let cmd1 = RetrievalCommand::parse(&data).unwrap();
    assert_eq!(cmd1,
  	 	RetrievalCommand{
    		command_name: "gets".to_owned(),
    		keys: vec!["key1".to_owned(), "key2".to_owned()],
    	}
  	);
}
