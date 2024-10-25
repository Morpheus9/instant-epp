//! Types for EPP RGP restore request

use instant_xml::{FromXml, ToXml};

use crate::{
    domain::{info::DomainInfo, update::DomainUpdate},
    request::{Extension, Transaction},
};

use super::{RgpStatus, XMLNS};

impl<'a> Transaction<Update<RgpRestoreRequest<'a>>> for DomainUpdate<'a> {}

impl<'a> Transaction<Update<RgpRestoreRequest<'a>>> for DomainInfo<'a> {}

impl Extension for Update<RgpRestoreRequest<'_>> {
    type Response = RgpRequestResponse;
}

// Request

#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "update", ns(XMLNS))]
pub struct Update<T> {
    pub data: T,
}

/// Type corresponding to the `<restore>` tag for an rgp restore request
#[derive(Debug, ToXml)]
#[xml(rename = "restore", ns(XMLNS))]
pub struct RgpRestoreRequest<'a> {
    /// The value of the op attribute in the `<restore>` tag
    #[xml(attribute)]
    pub op: &'a str,
}

impl Default for RgpRestoreRequest<'static> {
    fn default() -> Self {
        Self { op: "request" }
    }
}

// Response

#[derive(Debug, FromXml)]
#[xml(rename = "upData", ns(XMLNS))]
/// Type that represents the `<resData>` tag for domain transfer response
pub struct RgpRequestUpdateResponse {
    /// Data under the `<rgpStatus>` tag
    pub rgp_status: Vec<RgpStatus>,
}

#[derive(Debug, FromXml)]
#[xml(rename = "infData", ns(XMLNS))]
/// Type that represents the `<resData>` tag for domain transfer response
pub struct RgpRequestInfoResponse {
    /// Data under the `<rgpStatus>` tag
    pub rgp_status: Vec<RgpStatus>,
}

/// Type that represents the `<resData>` tag for domain transfer response
#[derive(Debug, FromXml)]
#[xml(forward)]
pub enum RgpRequestResponse {
    Update(RgpRequestUpdateResponse),
    Info(RgpRequestInfoResponse),
}

#[cfg(test)]
mod tests {
    use super::{RgpRestoreRequest, Update};
    use crate::domain::info::DomainInfo;
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::extensions::rgp::request::RgpRequestResponse;
    use crate::extensions::rgp::RgpStatus;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file_with_ext, SUCCESS_MSG, SVTRID};

    #[test]
    fn request_command() {
        let domain_restore_request = Update {
            data: RgpRestoreRequest::default(),
        };

        let mut object = DomainUpdate::new("eppdev.com");

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: None,
        };

        object.info(change_info);

        assert_serialized(
            "request/extensions/rgp_restore_request.xml",
            (&object, &domain_restore_request),
        );
    }

    #[test]
    fn request_response() {
        let object = response_from_file_with_ext::<DomainUpdate, Update<RgpRestoreRequest>>(
            "response/extensions/rgp_restore.xml",
        );
        let ext = object.extension.unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);

        let data = match ext.data {
            RgpRequestResponse::Update(data) => data,
            _ => panic!("Unexpected response type"),
        };

        assert_eq!(data.rgp_status[0], RgpStatus::PendingRestore);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }

    #[test]
    fn domain_info_request_response() {
        let object = response_from_file_with_ext::<DomainInfo, Update<RgpRestoreRequest>>(
            "response/extensions/domain_info_rgp.xml",
        );
        let ext = object.extension.unwrap();

        let data = match ext.data {
            RgpRequestResponse::Info(data) => data,
            _ => panic!("Unexpected response type"),
        };

        assert_eq!(data.rgp_status[0], RgpStatus::AddPeriod);
        assert_eq!(data.rgp_status[1], RgpStatus::RenewPeriod);
    }
}
