use crate::contract::{DIDContract, DIDContractClient};
use crate::service::{Service, ServiceType};
use crate::verification_method::{
    VerificationMethod, VerificationMethodType, VerificationRelationship,
};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String, Vec};

pub struct DIDContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub did_method: String,
    pub context: Vec<String>,
    pub verification_methods: Vec<VerificationMethod>,
    pub services: Vec<Service>,
    pub contract: DIDContractClient<'a>,
}

impl<'a> DIDContractTest<'a> {
    pub fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::random(&env);
        let did_method = String::from_slice(&env, "chaincerts");
        let context = vec![
            &env,
            String::from_slice(&env, "https://www.w3.org/ns/did/v1"),
            String::from_slice(&env, "https://w3id.org/security/suites/ed25519-2020/v1"),
            String::from_slice(&env, "https://w3id.org/security/suites/x25519-2020/v1"),
        ];
        let verification_methods = vec![
            &env,
            VerificationMethod {
                id: String::from_slice(&env, "keys-1"),
                type_: VerificationMethodType::Ed25519VerificationKey2020,
                controller: String::from_slice(&env, ""),
                public_key_multibase: String::from_slice(
                    &env,
                    "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
                ),
                verification_relationships: vec![
                    &env,
                    VerificationRelationship::Authentication,
                    VerificationRelationship::AssertionMethod,
                    VerificationRelationship::CapabilityInvocation,
                ],
            },
            VerificationMethod {
                id: String::from_slice(&env, "keys-2"),
                type_: VerificationMethodType::X25519KeyAgreementKey2020,
                controller: String::from_slice(&env, ""),
                public_key_multibase: String::from_slice(
                    &env,
                    "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN",
                ),
                verification_relationships: vec![&env, VerificationRelationship::KeyAgreement],
            },
            VerificationMethod {
                id: String::from_slice(&env, "keys-3"),
                type_: VerificationMethodType::Ed25519VerificationKey2020,
                controller: String::from_slice(&env, "did:chaincerts:ujonoldr6vfinvl3a32su5lw"),
                public_key_multibase: String::from_slice(
                    &env,
                    "z6MkkD6nsbeFUQ28G5D7gPaAJgMk2o7SNQeepaZvf5Tbpjy6",
                ),
                verification_relationships: vec![
                    &env,
                    VerificationRelationship::CapabilityDelegation,
                ],
            },
        ];

        let services = vec![
            &env,
            Service {
                id: String::from_slice(&env, "chaincerts"),
                type_: ServiceType::LinkedDomains,
                service_endpoint: String::from_slice(&env, "https://chaincerts.co"),
            },
        ];

        let contract = DIDContractClient::new(&env, &env.register_contract(None, DIDContract));

        DIDContractTest {
            env,
            admin,
            did_method,
            context,
            verification_methods,
            services,
            contract,
        }
    }
}
