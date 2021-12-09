//! Types for EPP namestore request and responses

use serde::{Deserialize, Serialize};

use crate::{common::StringValue, request::EppExtension};

pub const XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::check::DomainCheck;
/// use epp_client::extensions::namestore::NameStore;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     let namestore_ext = NameStore::new("com");
///
///     // Create an DomainCheck instance
///     let domain_check = DomainCheck::<NameStore>::new(
///         vec!["eppdev-100.com", "eppdev-200.com"],
///     ).with_extension(namestore_ext);
///
///     // send it to the registry and receive a response of type EppDomainCheckResponse
///     let response = client.transact(domain_check, "test-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl NameStore {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            data: NameStoreData {
                xmlns: XMLNS.to_string(),
                subproduct: subproduct.into(),
            },
        }
    }
}

impl EppExtension for NameStore {
    type Response = NameStore;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "namestoreExt:namestoreExt")]
pub struct NameStore {
    #[serde(rename = "namestoreExt:namestoreExt", alias = "namestoreExt")]
    pub data: NameStoreData,
}

#[derive(Serialize, Deserialize, Debug)]
/// Type for EPP XML &lt;namestoreExt&gt; extension
pub struct NameStoreData {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:namestoreExt", alias = "xmlns")]
    pub xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "namestoreExt:subProduct", alias = "subProduct")]
    pub subproduct: StringValue,
}