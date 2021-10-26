pub use anchorproxycontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod anchorproxycontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "AnchorProxyContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ANCHORPROXYCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_anchorTrees\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_governance\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"contract IAnchor\",\"name\":\"addr\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"enum AnchorProxy.InstanceState\",\"name\":\"state\",\"type\":\"uint8\"}],\"internalType\":\"struct AnchorProxy.Instance\",\"name\":\"instance\",\"type\":\"tuple\"}],\"internalType\":\"struct AnchorProxy.AnchorStruct[]\",\"name\":\"_instances\",\"type\":\"tuple[]\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"contract IAnchor\",\"name\":\"anchor\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"name\":\"AnchorProxyDeposit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\"}],\"name\":\"EncryptedNote\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"contract IAnchor\",\"name\":\"instance\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"enum AnchorProxy.InstanceState\",\"name\":\"state\",\"type\":\"uint8\"}],\"name\":\"InstanceStateUpdated\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"anchorTrees\",\"outputs\":[{\"internalType\":\"contract IAnchorTrees\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"_encryptedNotes\",\"type\":\"bytes[]\"}],\"name\":\"backupNotes\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IAnchor\",\"name\":\"_anchor\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_commitment\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_encryptedNote\",\"type\":\"bytes\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"governance\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IAnchor\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"instances\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"enum AnchorProxy.InstanceState\",\"name\":\"state\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IAnchor\",\"name\":\"_anchor\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"},{\"components\":[{\"internalType\":\"bytes\",\"name\":\"_roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_refreshCommitment\",\"type\":\"bytes32\"},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"address payable\",\"name\":\"_relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_refund\",\"type\":\"uint256\"}],\"internalType\":\"struct IAnchor.PublicInputs\",\"name\":\"_publicInputs\",\"type\":\"tuple\"}],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct AnchorProxyContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AnchorProxyContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for AnchorProxyContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AnchorProxyContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AnchorProxyContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                ANCHORPROXYCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `anchorTrees` (0x9b742c01) function"]
        pub fn anchor_trees(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([155, 116, 44, 1], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `backupNotes` (0x6485ba2a) function"]
        pub fn backup_notes(
            &self,
            encrypted_notes: ::std::vec::Vec<Vec<u8>>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 133, 186, 42], encrypted_notes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x13d98d13) function"]
        pub fn deposit(
            &self,
            anchor: ethers::core::types::Address,
            commitment: [u8; 32],
            encrypted_note: Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [19, 217, 141, 19],
                    (anchor, commitment, encrypted_note),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `governance` (0x5aa6e675) function"]
        pub fn governance(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([90, 166, 230, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `instances` (0x032bb443) function"]
        pub fn instances(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, u8),
        > {
            self.0
                .method_hash([3, 43, 180, 67], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x819f3f6e) function"]
        pub fn withdraw(
            &self,
            anchor: ethers::core::types::Address,
            proof: Vec<u8>,
            public_inputs: PublicInputs,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [129, 159, 63, 110],
                    (anchor, proof, public_inputs),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AnchorProxyDeposit` event"]
        pub fn anchor_proxy_deposit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AnchorProxyDepositFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `EncryptedNote` event"]
        pub fn encrypted_note_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EncryptedNoteFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InstanceStateUpdated` event"]
        pub fn instance_state_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InstanceStateUpdatedFilter>
        {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, AnchorProxyContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "AnchorProxyDeposit",
        abi = "AnchorProxyDeposit(address,bytes32,uint256)"
    )]
    pub struct AnchorProxyDepositFilter {
        #[ethevent(indexed)]
        pub anchor: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub timestamp: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "EncryptedNote", abi = "EncryptedNote(address,bytes)")]
    pub struct EncryptedNoteFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub encrypted_note: Vec<u8>,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "InstanceStateUpdated",
        abi = "InstanceStateUpdated(address,uint8)"
    )]
    pub struct InstanceStateUpdatedFilter {
        #[ethevent(indexed)]
        pub instance: ethers::core::types::Address,
        pub state: u8,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AnchorProxyContractEvents {
        AnchorProxyDepositFilter(AnchorProxyDepositFilter),
        EncryptedNoteFilter(EncryptedNoteFilter),
        InstanceStateUpdatedFilter(InstanceStateUpdatedFilter),
    }
    impl ethers::core::abi::Tokenizable for AnchorProxyContractEvents {
        fn from_token(
            token: ethers::core::abi::Token,
        ) -> Result<Self, ethers::core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) =
                AnchorProxyDepositFilter::from_token(token.clone())
            {
                return Ok(
                    AnchorProxyContractEvents::AnchorProxyDepositFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EncryptedNoteFilter::from_token(token.clone())
            {
                return Ok(AnchorProxyContractEvents::EncryptedNoteFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                InstanceStateUpdatedFilter::from_token(token.clone())
            {
                return Ok(
                    AnchorProxyContractEvents::InstanceStateUpdatedFilter(
                        decoded,
                    ),
                );
            }
            Err(ethers::core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers::core::abi::Token {
            match self {
                AnchorProxyContractEvents::AnchorProxyDepositFilter(
                    element,
                ) => element.into_token(),
                AnchorProxyContractEvents::EncryptedNoteFilter(element) => {
                    element.into_token()
                }
                AnchorProxyContractEvents::InstanceStateUpdatedFilter(
                    element,
                ) => element.into_token(),
            }
        }
    }
    impl ethers::core::abi::TokenizableItem for AnchorProxyContractEvents {}
    impl ethers::contract::EthLogDecode for AnchorProxyContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnchorProxyDepositFilter::decode_log(log) {
                return Ok(
                    AnchorProxyContractEvents::AnchorProxyDepositFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EncryptedNoteFilter::decode_log(log) {
                return Ok(AnchorProxyContractEvents::EncryptedNoteFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InstanceStateUpdatedFilter::decode_log(log) {
                return Ok(
                    AnchorProxyContractEvents::InstanceStateUpdatedFilter(
                        decoded,
                    ),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    #[doc = "`PublicInputs(bytes,bytes32,bytes32,address,address,uint256,uint256)`"]
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType,
    )]
    pub struct PublicInputs {
        pub roots: Vec<u8>,
        pub nullifier_hash: [u8; 32],
        pub refresh_commitment: [u8; 32],
        pub recipient: ethers::core::types::Address,
        pub relayer: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
        pub refund: ethers::core::types::U256,
    }
}
