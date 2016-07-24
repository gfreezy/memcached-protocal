use std::io::BufReader;
use memcached_protocal::parse;
use memcached_protocal::Store;
use memcached_protocal::Retrieval;
use memcached_protocal::RetrievalCommand;
use memcached_protocal::StoreCommand;


#[test]
fn test_parse_get() {
    let data = "get key\r\n".as_bytes();

    let cmd1 = parse(&mut BufReader::new(data)).unwrap();
    assert_eq!(cmd1,
  	 	Retrieval(RetrievalCommand{
    		command_name: "get".to_owned(),
    		keys: vec!["key".to_owned()],
    	})
  	);
}

#[test]
fn test_parse_gets() {
    let data = "gets key1 key2\r\n".as_bytes();

    let cmd1 = parse(&mut BufReader::new(data)).unwrap();
    assert_eq!(cmd1,
  	 	Retrieval(RetrievalCommand{
    		command_name: "gets".to_owned(),
    		keys: vec!["key1".to_owned(), "key2".to_owned()],
    	})
  	);
}


#[test]
fn test_parse_with_noreply() {
    let data = "value".as_bytes();
    let key = "key";
    let cmd_str = format!("set {} 0 0 {} 1\r\n", key, data.len());
    let mut store_cmd = Vec::from(cmd_str.as_bytes());
    store_cmd.extend_from_slice(data);
    store_cmd.extend_from_slice("\r\n".as_bytes());

    let cmd1 = parse(&mut BufReader::new(store_cmd.as_slice())).unwrap();
    assert_eq!(cmd1,
  	 	Store(StoreCommand{
    		command_name: "set".to_owned(),
    		key: key.to_owned(),
    		flags: 0,
    		exptime: 0,
    		bytes: data.len() as u64,
    		cas_unique: None,
    		noreply: Some("1".to_owned()),
    		data_block: Vec::from(data),
    	})
  	);
}

#[test]
fn test_parse_without_noreply() {
    let data = "value".as_bytes();
    let key = "key";
    let cmd_str = format!("set {} 0 0 {}\r\n", key, data.len());
    let mut store_cmd = Vec::from(cmd_str.as_bytes());
    store_cmd.extend_from_slice(data);
    store_cmd.extend_from_slice("\r\n".as_bytes());

    let cmd1 = parse(&mut BufReader::new(store_cmd.as_slice())).unwrap();
    assert_eq!(cmd1,
      Store(StoreCommand{
        command_name: "set".to_owned(),
        key: key.to_owned(),
        flags: 0,
        exptime: 0,
        bytes: data.len() as u64,
        cas_unique: None,
        noreply: None,
        data_block: Vec::from(data),
      })
    );
}

#[test]
fn test_parse_cas_with_noreply() {
    let data = "value".as_bytes();
    let key = "key";
    let cmd_str = format!("cas {} 0 0 {} 1 1\r\n", key, data.len());
    let mut store_cmd = Vec::from(cmd_str.as_bytes());
    store_cmd.extend_from_slice(data);
    store_cmd.extend_from_slice("\r\n".as_bytes());

    let cmd1 = parse(&mut BufReader::new(store_cmd.as_slice())).unwrap();
    assert_eq!(cmd1,
      Store(StoreCommand{
        command_name: "cas".to_owned(),
        key: key.to_owned(),
        flags: 0,
        exptime: 0,
        bytes: data.len() as u64,
        cas_unique: Some(1),
        noreply: Some("1".to_owned()),
        data_block: Vec::from(data),
      })
    );
}


#[test]
fn test_parse_cas_without_noreply() {
    let data = "value".as_bytes();
    let key = "key";
    let cmd_str = format!("cas {} 0 0 {} 1\r\n", key, data.len());
    let mut store_cmd = Vec::from(cmd_str.as_bytes());
    store_cmd.extend_from_slice(data);
    store_cmd.extend_from_slice("\r\n".as_bytes());

    let cmd1 = parse(&mut BufReader::new(store_cmd.as_slice())).unwrap();
    assert_eq!(cmd1,
      Store(StoreCommand{
        command_name: "cas".to_owned(),
        key: key.to_owned(),
        flags: 0,
        exptime: 0,
        bytes: data.len() as u64,
        cas_unique: Some(1),
        noreply: None,
        data_block: Vec::from(data),
      })
    );
}


#[test]
#[should_panic]
fn test_parse_should_panic() {
    let data = "value".as_bytes();
    let key = "key";
    let cmd_str = format!("caas {} 0 0 {} 1\r\n", key, data.len());
    let mut store_cmd = Vec::from(cmd_str.as_bytes());
    store_cmd.extend_from_slice(data);
    store_cmd.extend_from_slice("\r\n".as_bytes());

    parse(&mut BufReader::new(store_cmd.as_slice())).unwrap();
}
