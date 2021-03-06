use std::result::Result;

use opcua_types::*;
use opcua_types::status_codes::StatusCode;
use opcua_types::status_codes::StatusCode::*;
use opcua_types::service_types::{ServiceFault, SecurityTokenRequestType, OpenSecureChannelResponse, ResponseHeader, ChannelSecurityToken};

use opcua_core::comms::prelude::*;
use opcua_core::crypto::SecurityPolicy;

struct SecureChannelState {
    // Issued flag
    pub issued: bool,
    // Renew count, debugging
    pub renew_count: usize,
    // Last secure channel id
    last_secure_channel_id: UInt32,
    /// Last token id number
    last_token_id: UInt32,
}

impl SecureChannelState {
    pub fn new() -> SecureChannelState {
        SecureChannelState {
            last_secure_channel_id: 0,
            issued: false,
            renew_count: 0,
            last_token_id: 0,
        }
    }

    pub fn create_secure_channel_id(&mut self) -> UInt32 {
        self.last_secure_channel_id += 1;
        self.last_secure_channel_id
    }

    pub fn create_token_id(&mut self) -> UInt32 {
        self.last_token_id += 1;
        self.last_token_id
    }
}

pub struct SecureChannelService {
    // Secure channel info for the session
    secure_channel_state: SecureChannelState,
}

impl SecureChannelService {
    pub fn new() -> SecureChannelService {
        SecureChannelService {
            secure_channel_state: SecureChannelState::new(),
        }
    }

    pub fn open_secure_channel(&mut self, secure_channel: &mut SecureChannel, security_header: &SecurityHeader, client_protocol_version: UInt32, message: &SupportedMessage) -> Result<SupportedMessage, StatusCode> {
        let request = match *message {
            SupportedMessage::OpenSecureChannelRequest(ref request) => {
                trace!("Got secure channel request {:?}", request);
                request
            }
            _ => {
                error!("message is not an open secure channel request, got {:?}", message);
                return Err(BadUnexpectedError);
            }
        };

        let security_header = match *security_header {
            SecurityHeader::Asymmetric(ref security_header) => {
                security_header
            }
            _ => {
                error!("Secure channel request message does not have asymmetric security header");
                return Err(BadUnexpectedError);
            }
        };

        // Must compare protocol version to the one from HELLO
        if request.client_protocol_version != client_protocol_version {
            error!("Client sent a different protocol version than it did in the HELLO - {} vs {}", request.client_protocol_version, client_protocol_version);
            return Ok(ServiceFault::new_supported_message(&request.request_header, BadProtocolVersionUnsupported));
        }

        // Test the request type
        match request.request_type {
            SecurityTokenRequestType::Issue => {
                trace!("Request type == Issue");
                // check to see if renew has been called before or not
                if self.secure_channel_state.renew_count > 0 {
                    error!("Asked to issue token on session that has called renew before");
                }
            }
            SecurityTokenRequestType::Renew => {
                trace!("Request type == Renew");

                // Check for a duplicate nonce. It is invalid for the renew to use the same nonce
                // as was used for last issue/renew
                if request.client_nonce.as_ref() == &secure_channel.remote_nonce()[..] {
                    return Ok(ServiceFault::new_supported_message(&request.request_header, BadNonceInvalid));
                }

                // check to see if the secure channel has been issued before or not
                if !self.secure_channel_state.issued {
                    error!("Asked to renew token on session that has never issued token");
                    return Err(BadUnexpectedError);
                }
                self.secure_channel_state.renew_count += 1;
            }
        }

        // Check the requested security mode
        debug!("Message security mode == {:?}", request.security_mode);
        match request.security_mode {
            MessageSecurityMode::None | MessageSecurityMode::Sign | MessageSecurityMode::SignAndEncrypt => {
                // TODO validate NONCE
            }
            _ => {
                error!("Security mode is invalid");
                return Ok(ServiceFault::new_supported_message(&request.request_header, BadSecurityModeRejected));
            }
        }

        // Process the request
        self.secure_channel_state.issued = true;

        // Create a new secure channel info
        let security_mode = request.security_mode;
        secure_channel.set_security_mode(security_mode);
        secure_channel.set_token_id(self.secure_channel_state.create_token_id());
        secure_channel.set_secure_channel_id(self.secure_channel_state.create_secure_channel_id());
        secure_channel.set_remote_cert_from_byte_string(&security_header.sender_certificate)?;

        let nonce_result = secure_channel.set_remote_nonce_from_byte_string(&request.client_nonce);
        if nonce_result.is_ok() {
            secure_channel.create_random_nonce();
        } else {
            error!("Was unable to set their nonce, check logic");
            return Ok(ServiceFault::new_supported_message(&request.request_header, nonce_result.unwrap_err()));
        }

        let security_policy = secure_channel.security_policy();
        if security_policy != SecurityPolicy::None && (security_mode == MessageSecurityMode::Sign || security_mode == MessageSecurityMode::SignAndEncrypt) {
            secure_channel.derive_keys();
        }

        let response = OpenSecureChannelResponse {
            response_header: ResponseHeader::new_good(&request.request_header),
            server_protocol_version: 0,
            security_token: ChannelSecurityToken {
                channel_id: secure_channel.secure_channel_id(),
                token_id: secure_channel.token_id(),
                created_at: DateTime::now(),
                revised_lifetime: request.requested_lifetime,
            },
            server_nonce: secure_channel.local_nonce_as_byte_string(),
        };
        Ok(SupportedMessage::OpenSecureChannelResponse(response))
    }

    pub fn close_secure_channel(&mut self, _: &SupportedMessage) -> Result<SupportedMessage, StatusCode> {
        info!("CloseSecureChannelRequest received, session closing");
        Err(BadConnectionClosed)
    }
}