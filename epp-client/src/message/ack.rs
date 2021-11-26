//! Types for EPP message ack request

use epp_client_macros::*;

use crate::common::{ElementName, NoExtension};
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MessageAck<E> {
    request: MessageAckRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for MessageAck<E> {
    type Input = MessageAckRequest;
    type Output = String;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for registry <poll op="ack"> command
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::message::ack::MessageAck;
/// use epp_client::generate_client_tr_id;
/// use epp_client::common::NoExtension;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an MessageAck instance
///     let message_ack = MessageAck::<NoExtension>::new(12345);
///
///     // send it to the registry and receive a response of type MessageAckResponse
///     let response = client.transact_new(message_ack, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> MessageAck<E> {
    pub fn new(message_id: u32) -> MessageAck<NoExtension> {
        MessageAck {
            request: MessageAckRequest {
                op: "ack".to_string(),
                message_id: message_id.to_string(),
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> MessageAck<F> {
        MessageAck {
            request: self.request,
            extension: Some(extension),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAckRequest {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}
