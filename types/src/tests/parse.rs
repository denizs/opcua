use std::str::FromStr;

use ::*;

#[test]
fn parse_invalid_node_id() {
    let node_id = NodeId::from_str("ns=99 ;i=35");
    assert_eq!(node_id.is_err(), true);
    let node_id = NodeId::from_str("ns=99;i=x");
    assert_eq!(node_id.is_err(), true);
    let node_id = NodeId::from_str("ns=99;s=");
    assert_eq!(node_id.is_err(), true);
    let node_id = NodeId::from_str("ns=;s=valid str");
    assert_eq!(node_id.is_err(), true);
    let node_id = NodeId::from_str("ns=;s=valid str");
    assert_eq!(node_id.is_err(), true);
    let node_id = NodeId::from_str("ns=65537;s=valid str");
    assert_eq!(node_id.is_err(), true);
}

#[test]
fn parse_node_id_integer() {
    // Integer
    let node_id = NodeId::from_str("i=13");
    assert_eq!(node_id.is_ok(), true);
    let node_id = node_id.unwrap();
    assert_eq!(node_id.namespace, 0);
    assert_eq!(node_id.identifier, Identifier::Numeric(13));

    let node_id = NodeId::from_str("ns=99;i=35");
    assert_eq!(node_id.is_ok(), true);
    let node_id = node_id.unwrap();
    assert_eq!(node_id.namespace, 99);
    assert_eq!(node_id.identifier, Identifier::Numeric(35));
}

#[test]
fn parse_node_id_string() {
    // String
    let node_id = NodeId::from_str("ns=1;s=Hello World");
    assert_eq!(node_id.is_ok(), true);
    let node_id = node_id.unwrap();
    assert_eq!(node_id.namespace, 1);
    assert_eq!(node_id.identifier, Identifier::String(UAString::from("Hello World")));

    let node_id = NodeId::from_str("s=No NS this time");
    assert_eq!(node_id.is_ok(), true);
    let node_id = node_id.unwrap();
    assert_eq!(node_id.namespace, 0);
    assert_eq!(node_id.identifier, Identifier::String(UAString::from("No NS this time")));
}

#[test]
fn parse_node_id_guid() {
    // Guid
    let node_id = NodeId::from_str("g=72962B91-FA75-4ae6-8D28-B404DC7DAF63");
    assert_eq!(node_id.is_ok(), true);
    let node_id = node_id.unwrap();
    assert_eq!(node_id.namespace, 0);
    assert_eq!(node_id.identifier, Identifier::Guid(Guid::from_str("72962B91-FA75-4ae6-8D28-B404DC7DAF63").unwrap()));
}

#[test]
fn parse_node_id_byte_string() {
    // ByteString (sample bytes comes from OPC UA spec)
    let node_id = NodeId::from_str("ns=1;b=M/RbKBsRVkePCePcx24oRA==");
    assert_eq!(node_id.is_ok(), true);
    let node_id = node_id.unwrap();
    assert_eq!(node_id.namespace, 1);
    assert_eq!(node_id.identifier, Identifier::ByteString(ByteString::from_base64("M/RbKBsRVkePCePcx24oRA==").unwrap()));
    // Turn byte string back to string, compare to original
    assert_eq!(&node_id.to_string(), "ns=1;b=M/RbKBsRVkePCePcx24oRA==");
}
