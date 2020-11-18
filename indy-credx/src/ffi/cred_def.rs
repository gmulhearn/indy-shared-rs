use std::os::raw::c_char;

use ffi_support::{rust_string_to_c, FfiStr};
use indy_utils::Qualifiable;

use super::error::ErrorCode;
use super::object::ObjectHandle;
use crate::services::{
    issuer::new_credential_definition,
    types::{
        CredentialDefinition, CredentialDefinitionConfig, CredentialDefinitionPrivate,
        CredentialKeyCorrectnessProof as KeyCorrectnessProof, DidValue, SignatureType,
    },
};

#[no_mangle]
pub extern "C" fn credx_create_credential_definition(
    origin_did: FfiStr,
    schema: ObjectHandle,
    tag: FfiStr,
    signature_type: FfiStr,
    support_revocation: i8,
    cred_def_p: *mut ObjectHandle,
    cred_def_pvt_p: *mut ObjectHandle,
    key_proof_p: *mut ObjectHandle,
) -> ErrorCode {
    catch_err! {
        check_useful_c_ptr!(cred_def_p);
        check_useful_c_ptr!(cred_def_pvt_p);
        check_useful_c_ptr!(key_proof_p);
        let origin_did = DidValue::from_str(origin_did.as_str())?;
        let (cred_def, cred_def_pvt, key_proof) = new_credential_definition(
            &origin_did,
            schema.load()?.cast_ref()?,
            tag.as_opt_str().unwrap_or("default"),
            SignatureType::from_str(signature_type.as_str()).map_err(err_map!(Input))?,
            CredentialDefinitionConfig {
                support_revocation: support_revocation != 0
            }
        )?;
        let cred_def = ObjectHandle::create(cred_def)?;
        let cred_def_pvt = ObjectHandle::create(cred_def_pvt)?;
        let key_proof = ObjectHandle::create(key_proof)?;
        unsafe {
            *cred_def_p = cred_def;
            *cred_def_pvt_p = cred_def_pvt;
            *key_proof_p = key_proof;
        }
        Ok(ErrorCode::Success)
    }
}

#[no_mangle]
pub extern "C" fn credx_credential_definition_get_id(
    handle: ObjectHandle,
    result_p: *mut *const c_char,
) -> ErrorCode {
    catch_err! {
        check_useful_c_ptr!(result_p);
        let schema = handle.load()?;
        let id = match schema.cast_ref::<CredentialDefinition>()? {
            CredentialDefinition::CredentialDefinitionV1(c) => c.id.to_string(),
        };
        unsafe { *result_p = rust_string_to_c(id) };
        Ok(ErrorCode::Success)
    }
}

impl_indy_object!(CredentialDefinition, "CredentialDefinition");
impl_indy_object_from_json!(CredentialDefinition, credx_credential_definition_from_json);

impl_indy_object!(CredentialDefinitionPrivate, "CredentialDefinitionPrivate");
impl_indy_object_from_json!(
    CredentialDefinitionPrivate,
    credx_credential_definition_private_from_json
);

impl_indy_object!(KeyCorrectnessProof, "KeyCorrectnessProof");
impl_indy_object_from_json!(KeyCorrectnessProof, credx_key_correctness_proof_from_json);
