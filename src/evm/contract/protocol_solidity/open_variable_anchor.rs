pub use open_v_anchor_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod open_v_anchor_contract {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
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
    #[doc = "OpenVAnchorContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static OPENVANCHORCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"_hasher\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_levels\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"merkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeAddition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"merkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Insertion\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"subTreeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCommitment\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nullifier\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewNullifier\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PublicKey\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXT_AMOUNT\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_fromTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_extAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_executeWrapping\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_fromTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_minusExtAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_withdrawAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_extAmount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculatePublicAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMaximumDepositLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMinimalWithdrawalLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint48\",\"name\":\"destinationChainId\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"relayingFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"internalType\":\"struct Edge[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLevels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getZeroHash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_left\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_right\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_nullifierHash\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nullifierHashes\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_roots\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxEdges\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maximumDepositAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minimalWithdrawalAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"outerLevels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_resourceId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseChainIdFromResourceId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"keyData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"register\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roots\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"latestLeafindex\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_leafIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_srcResourceID\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updateEdge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"withdrawAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"relayingFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"merkleProof\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"commitmentIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"withdrawAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"relayingFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"merkleProof\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"commitmentIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"withdrawAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint48\",\"name\":\"destinationChainId\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"relayingFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapAndDeposit\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct OpenVAnchorContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OpenVAnchorContract<M> {
        fn clone(&self) -> Self {
            OpenVAnchorContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OpenVAnchorContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for OpenVAnchorContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OpenVAnchorContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OpenVAnchorContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                OPENVANCHORCONTRACT_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `EVM_CHAIN_ID_TYPE` (0x8b7e8782) function"]
        pub fn evm_chain_id_type(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 2]> {
            self.0
                .method_hash([139, 126, 135, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FIELD_SIZE` (0x414a37ba) function"]
        pub fn field_size(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([65, 74, 55, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_EXT_AMOUNT` (0x7fe24ffe) function"]
        pub fn max_ext_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([127, 226, 79, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_FEE` (0xbc063e1a) function"]
        pub fn max_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([188, 6, 62, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT_HISTORY_SIZE` (0xcd87a3b4) function"]
        pub fn root_history_size(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 135, 163, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ZERO_VALUE` (0xec732959) function"]
        pub fn zero_value(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([236, 115, 41, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_executeWrapping` (0x6338bcbc) function"]
        pub fn execute_wrapping(
            &self,
            from_token_address: ethers::core::types::Address,
            to_token_address: ethers::core::types::Address,
            ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash(
                    [99, 56, 188, 188],
                    (from_token_address, to_token_address, ext_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_withdrawAndUnwrap` (0x509cd41e) function"]
        pub fn _withdraw_and_unwrap(
            &self,
            from_token_address: ethers::core::types::Address,
            to_token_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            minus_ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [80, 156, 212, 30],
                    (
                        from_token_address,
                        to_token_address,
                        recipient,
                        minus_ext_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculatePublicAmount` (0x2570b7b4) function"]
        pub fn calculate_public_amount(
            &self,
            ext_amount: I256,
            fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([37, 112, 183, 180], (ext_amount, fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x49ce8997) function"]
        pub fn commitments(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 206, 137, 151], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureMaximumDepositLimit` (0x8c832b13) function"]
        pub fn configure_maximum_deposit_limit(
            &self,
            maximum_deposit_amount: ethers::core::types::U256,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [140, 131, 43, 19],
                    (maximum_deposit_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureMinimalWithdrawalLimit` (0x1f7f99f7) function"]
        pub fn configure_minimal_withdrawal_limit(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [31, 127, 153, 247],
                    (minimal_withdrawal_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentNeighborRootIndex` (0x5d2d766c) function"]
        pub fn current_neighbor_root_index(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([93, 45, 118, 108], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentRootIndex` (0x90eeb02b) function"]
        pub fn current_root_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([144, 238, 176, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xec680c50) function"]
        pub fn deposit(
            &self,
            destination_chain_id: u64,
            deposit_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            relaying_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [236, 104, 12, 80],
                    (
                        destination_chain_id,
                        deposit_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        relaying_fee,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeExistsForChain` (0xfa731687) function"]
        pub fn edge_exists_for_chain(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 115, 22, 135], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeIndex` (0xe70ea87c) function"]
        pub fn edge_index(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([231, 14, 168, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeList` (0xdbc916b8) function"]
        pub fn edge_list(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([219, 201, 22, 184], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledSubtrees` (0xf178e47c) function"]
        pub fn filled_subtrees(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([241, 120, 228, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainIdType` (0x4c830cbd) function"]
        pub fn get_chain_id_type(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([76, 131, 12, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHasher` (0xea495db0) function"]
        pub fn get_hasher(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 73, 93, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRoot` (0xba70f757) function"]
        pub fn get_last_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborEdges` (0x8c0d34d8) function"]
        pub fn get_latest_neighbor_edges(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Edge>>
        {
            self.0
                .method_hash([140, 13, 52, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborRoots` (0x1e627617) function"]
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::U256>,
        > {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLevels` (0x0c394a60) function"]
        pub fn get_levels(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([12, 57, 74, 96], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextIndex` (0x0eb7606f) function"]
        pub fn get_next_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([14, 183, 96, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProposalNonce` (0x0b27fb9a) function"]
        pub fn get_proposal_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getZeroHash` (0x305e9eac) function"]
        pub fn get_zero_hash(
            &self,
            index: u32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([48, 94, 158, 172], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handler` (0xc80916d4) function"]
        pub fn handler(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 9, 22, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasEdge` (0x92156311) function"]
        pub fn has_edge(
            &self,
            chain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 21, 99, 17], chain_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashLeftRight` (0x5bb93995) function"]
        pub fn hash_left_right(
            &self,
            left: ethers::core::types::U256,
            right: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([91, 185, 57, 149], (left, right))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasher` (0xed33639f) function"]
        pub fn hasher(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([237, 51, 99, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xe4a30116) function"]
        pub fn initialize(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
            maximum_deposit_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [228, 163, 1, 22],
                    (minimal_withdrawal_amount, maximum_deposit_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialized` (0x158ef93e) function"]
        pub fn initialized(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownNeighborRoot` (0x3bfa8d7a) function"]
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ethers::core::types::U256,
            root: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 250, 141, 122], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0xa6232a93) function"]
        pub fn is_known_root(
            &self,
            root: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 35, 42, 147], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpent` (0x5a129efe) function"]
        pub fn is_spent(
            &self,
            nullifier_hash: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 18, 158, 254], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentArray` (0xea65ba49) function"]
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>>
        {
            self.0
                .method_hash([234, 101, 186, 73], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidRoots` (0xb75e6798) function"]
        pub fn is_valid_roots(
            &self,
            roots: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 94, 103, 152], roots)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastBalance` (0x8f1c56bd) function"]
        pub fn last_balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([143, 28, 86, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `levels` (0x4ecf518b) function"]
        pub fn levels(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([78, 207, 81, 139], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxEdges` (0x71523c32) function"]
        pub fn max_edges(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([113, 82, 60, 50], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maximumDepositAmount` (0x78abb49b) function"]
        pub fn maximum_deposit_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([120, 171, 180, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minimalWithdrawalAmount` (0x840b2791) function"]
        pub fn minimal_withdrawal_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([132, 11, 39, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `neighborRoots` (0x43e7119f) function"]
        pub fn neighbor_roots(
            &self,
            p0: ethers::core::types::U256,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([67, 231, 17, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nextIndex` (0xfc7e9c6f) function"]
        pub fn next_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([252, 126, 156, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nullifierHashes` (0x1f79a1e9) function"]
        pub fn nullifier_hashes(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 121, 161, 233], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `outerLevels` (0xbfbc0a39) function"]
        pub fn outer_levels(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 188, 10, 57], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseChainIdFromResourceId` (0xc2230d6e) function"]
        pub fn parse_chain_id_from_resource_id(
            &self,
            resource_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([194, 35, 13, 110], resource_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposalNonce` (0xcc3c74a1) function"]
        pub fn proposal_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `register` (0xb2bc6e0f) function"]
        pub fn register(
            &self,
            account: Account,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 188, 110, 15], (account,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0x72c1ad03) function"]
        pub fn set_handler(
            &self,
            handler: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (handler, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateEdge` (0xc1922f9e) function"]
        pub fn update_edge(
            &self,
            root: ethers::core::types::U256,
            leaf_index: u32,
            src_resource_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 146, 47, 158],
                    (root, leaf_index, src_resource_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x68ce8312) function"]
        pub fn withdraw(
            &self,
            withdraw_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            relaying_fee: ethers::core::types::U256,
            merkle_proof: ::std::vec::Vec<ethers::core::types::U256>,
            commitment_index: u32,
            root: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [104, 206, 131, 18],
                    (
                        withdraw_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        relaying_fee,
                        merkle_proof,
                        commitment_index,
                        root,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAndUnwrap` (0x5dc3544e) function"]
        pub fn withdraw_and_unwrap(
            &self,
            withdraw_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            relaying_fee: ethers::core::types::U256,
            merkle_proof: ::std::vec::Vec<ethers::core::types::U256>,
            commitment_index: u32,
            root: ethers::core::types::U256,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [93, 195, 84, 78],
                    (
                        withdraw_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        relaying_fee,
                        merkle_proof,
                        commitment_index,
                        root,
                        token_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapAndDeposit` (0xaf46d4d5) function"]
        pub fn wrap_and_deposit(
            &self,
            destination_chain_id: u64,
            deposit_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            relaying_fee: ethers::core::types::U256,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [175, 70, 212, 213],
                    (
                        destination_chain_id,
                        deposit_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        relaying_fee,
                        token_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `EdgeAddition` event"]
        pub fn edge_addition_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EdgeAdditionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EdgeUpdate` event"]
        pub fn edge_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EdgeUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Insertion` event"]
        pub fn insertion_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InsertionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCommitment` event"]
        pub fn new_commitment_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCommitmentFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewNullifier` event"]
        pub fn new_nullifier_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewNullifierFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PublicKey` event"]
        pub fn public_key_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PublicKeyFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, OpenVAnchorContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for OpenVAnchorContract<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(
        name = "EdgeAddition",
        abi = "EdgeAddition(uint256,uint256,uint256)"
    )]
    pub struct EdgeAdditionFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(
        name = "EdgeUpdate",
        abi = "EdgeUpdate(uint256,uint256,uint256)"
    )]
    pub struct EdgeUpdateFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(name = "Insertion", abi = "Insertion(uint256,uint32,uint256)")]
    pub struct InsertionFilter {
        #[ethevent(indexed)]
        pub commitment: ethers::core::types::U256,
        pub leaf_index: u32,
        pub timestamp: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(
        name = "NewCommitment",
        abi = "NewCommitment(uint256,uint256,uint256,bytes)"
    )]
    pub struct NewCommitmentFilter {
        pub commitment: ethers::core::types::U256,
        pub sub_tree_index: ethers::core::types::U256,
        pub leaf_index: ethers::core::types::U256,
        pub encrypted_output: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(name = "NewNullifier", abi = "NewNullifier(uint256)")]
    pub struct NewNullifierFilter {
        pub nullifier: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(name = "PublicKey", abi = "PublicKey(address,bytes)")]
    pub struct PublicKeyFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub key: ethers::core::types::Bytes,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum OpenVAnchorContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InsertionFilter(InsertionFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
    }
    impl ethers::contract::EthLogDecode for OpenVAnchorContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::EdgeUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::InsertionFilter(decoded));
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::NewCommitmentFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::NewNullifierFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::PublicKeyFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OpenVAnchorContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OpenVAnchorContractEvents::EdgeAdditionFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::EdgeUpdateFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::InsertionFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::NewCommitmentFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::NewNullifierFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::PublicKeyFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `[139, 126, 135, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "EVM_CHAIN_ID_TYPE", abi = "EVM_CHAIN_ID_TYPE()")]
    pub struct EvmChainIdTypeCall;
    #[doc = "Container type for all input parameters for the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "FIELD_SIZE", abi = "FIELD_SIZE()")]
    pub struct FieldSizeCall;
    #[doc = "Container type for all input parameters for the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `[127, 226, 79, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "MAX_EXT_AMOUNT", abi = "MAX_EXT_AMOUNT()")]
    pub struct MaxExtAmountCall;
    #[doc = "Container type for all input parameters for the `MAX_FEE` function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    #[doc = "Container type for all input parameters for the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `[205, 135, 163, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "ROOT_HISTORY_SIZE", abi = "ROOT_HISTORY_SIZE()")]
    pub struct RootHistorySizeCall;
    #[doc = "Container type for all input parameters for the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `[236, 115, 41, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "ZERO_VALUE", abi = "ZERO_VALUE()")]
    pub struct ZeroValueCall;
    #[doc = "Container type for all input parameters for the `_executeWrapping` function with signature `_executeWrapping(address,address,uint256)` and selector `[99, 56, 188, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "_executeWrapping",
        abi = "_executeWrapping(address,address,uint256)"
    )]
    pub struct ExecuteWrappingCall {
        pub from_token_address: ethers::core::types::Address,
        pub to_token_address: ethers::core::types::Address,
        pub ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_withdrawAndUnwrap` function with signature `_withdrawAndUnwrap(address,address,address,uint256)` and selector `[80, 156, 212, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "_withdrawAndUnwrap",
        abi = "_withdrawAndUnwrap(address,address,address,uint256)"
    )]
    pub struct _WithdrawAndUnwrapCall {
        pub from_token_address: ethers::core::types::Address,
        pub to_token_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub minus_ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `[37, 112, 183, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "calculatePublicAmount",
        abi = "calculatePublicAmount(int256,uint256)"
    )]
    pub struct CalculatePublicAmountCall {
        pub ext_amount: I256,
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `commitments` function with signature `commitments(uint256)` and selector `[73, 206, 137, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "commitments", abi = "commitments(uint256)")]
    pub struct CommitmentsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `configureMaximumDepositLimit` function with signature `configureMaximumDepositLimit(uint256,uint32)` and selector `[140, 131, 43, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "configureMaximumDepositLimit",
        abi = "configureMaximumDepositLimit(uint256,uint32)"
    )]
    pub struct ConfigureMaximumDepositLimitCall {
        pub maximum_deposit_amount: ethers::core::types::U256,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `configureMinimalWithdrawalLimit` function with signature `configureMinimalWithdrawalLimit(uint256,uint32)` and selector `[31, 127, 153, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "configureMinimalWithdrawalLimit",
        abi = "configureMinimalWithdrawalLimit(uint256,uint32)"
    )]
    pub struct ConfigureMinimalWithdrawalLimitCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `[93, 45, 118, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "currentNeighborRootIndex",
        abi = "currentNeighborRootIndex(uint256)"
    )]
    pub struct CurrentNeighborRootIndexCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `currentRootIndex` function with signature `currentRootIndex()` and selector `[144, 238, 176, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "currentRootIndex", abi = "currentRootIndex()")]
    pub struct CurrentRootIndexCall;
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit(uint48,uint256,address,bytes,uint256,uint256)` and selector `[236, 104, 12, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit(uint48,uint256,address,bytes,uint256,uint256)"
    )]
    pub struct DepositCall {
        pub destination_chain_id: u64,
        pub deposit_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub relaying_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "edgeExistsForChain", abi = "edgeExistsForChain(uint256)")]
    pub struct EdgeExistsForChainCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `[231, 14, 168, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "edgeIndex", abi = "edgeIndex(uint256)")]
    pub struct EdgeIndexCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `edgeList` function with signature `edgeList(uint256)` and selector `[219, 201, 22, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "edgeList", abi = "edgeList(uint256)")]
    pub struct EdgeListCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `[241, 120, 228, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "filledSubtrees", abi = "filledSubtrees(uint256)")]
    pub struct FilledSubtreesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `getChainIdType` function with signature `getChainIdType()` and selector `[76, 131, 12, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getChainIdType", abi = "getChainIdType()")]
    pub struct GetChainIdTypeCall;
    #[doc = "Container type for all input parameters for the `getHasher` function with signature `getHasher()` and selector `[234, 73, 93, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getHasher", abi = "getHasher()")]
    pub struct GetHasherCall;
    #[doc = "Container type for all input parameters for the `getLastRoot` function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getLastRoot", abi = "getLastRoot()")]
    pub struct GetLastRootCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `[140, 13, 52, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "getLatestNeighborEdges",
        abi = "getLatestNeighborEdges()"
    )]
    pub struct GetLatestNeighborEdgesCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "getLatestNeighborRoots",
        abi = "getLatestNeighborRoots()"
    )]
    pub struct GetLatestNeighborRootsCall;
    #[doc = "Container type for all input parameters for the `getLevels` function with signature `getLevels()` and selector `[12, 57, 74, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getLevels", abi = "getLevels()")]
    pub struct GetLevelsCall;
    #[doc = "Container type for all input parameters for the `getNextIndex` function with signature `getNextIndex()` and selector `[14, 183, 96, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getNextIndex", abi = "getNextIndex()")]
    pub struct GetNextIndexCall;
    #[doc = "Container type for all input parameters for the `getProposalNonce` function with signature `getProposalNonce()` and selector `[11, 39, 251, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    #[doc = "Container type for all input parameters for the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `[48, 94, 158, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getZeroHash", abi = "getZeroHash(uint32)")]
    pub struct GetZeroHashCall {
        pub index: u32,
    }
    #[doc = "Container type for all input parameters for the `handler` function with signature `handler()` and selector `[200, 9, 22, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "handler", abi = "handler()")]
    pub struct HandlerCall;
    #[doc = "Container type for all input parameters for the `hasEdge` function with signature `hasEdge(uint256)` and selector `[146, 21, 99, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "hasEdge", abi = "hasEdge(uint256)")]
    pub struct HasEdgeCall {
        pub chain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `[91, 185, 57, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "hashLeftRight", abi = "hashLeftRight(uint256,uint256)")]
    pub struct HashLeftRightCall {
        pub left: ethers::core::types::U256,
        pub right: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hasher` function with signature `hasher()` and selector `[237, 51, 99, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "hasher", abi = "hasher()")]
    pub struct HasherCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256)` and selector `[228, 163, 1, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256)")]
    pub struct InitializeCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
        pub maximum_deposit_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `[21, 142, 249, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    #[doc = "Container type for all input parameters for the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `[59, 250, 141, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "isKnownNeighborRoot",
        abi = "isKnownNeighborRoot(uint256,uint256)"
    )]
    pub struct IsKnownNeighborRootCall {
        pub neighbor_chain_id: ethers::core::types::U256,
        pub root: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `[166, 35, 42, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isSpent` function with signature `isSpent(uint256)` and selector `[90, 18, 158, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isSpent", abi = "isSpent(uint256)")]
    pub struct IsSpentCall {
        pub nullifier_hash: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `[234, 101, 186, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isSpentArray", abi = "isSpentArray(uint256[])")]
    pub struct IsSpentArrayCall {
        pub nullifier_hashes: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `[183, 94, 103, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isValidRoots", abi = "isValidRoots(uint256[])")]
    pub struct IsValidRootsCall {
        pub roots: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `lastBalance` function with signature `lastBalance()` and selector `[143, 28, 86, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "lastBalance", abi = "lastBalance()")]
    pub struct LastBalanceCall;
    #[doc = "Container type for all input parameters for the `levels` function with signature `levels()` and selector `[78, 207, 81, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "levels", abi = "levels()")]
    pub struct LevelsCall;
    #[doc = "Container type for all input parameters for the `maxEdges` function with signature `maxEdges()` and selector `[113, 82, 60, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "maxEdges", abi = "maxEdges()")]
    pub struct MaxEdgesCall;
    #[doc = "Container type for all input parameters for the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `[120, 171, 180, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "maximumDepositAmount", abi = "maximumDepositAmount()")]
    pub struct MaximumDepositAmountCall;
    #[doc = "Container type for all input parameters for the `minimalWithdrawalAmount` function with signature `minimalWithdrawalAmount()` and selector `[132, 11, 39, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "minimalWithdrawalAmount",
        abi = "minimalWithdrawalAmount()"
    )]
    pub struct MinimalWithdrawalAmountCall;
    #[doc = "Container type for all input parameters for the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `[67, 231, 17, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "neighborRoots", abi = "neighborRoots(uint256,uint32)")]
    pub struct NeighborRootsCall(pub ethers::core::types::U256, pub u32);
    #[doc = "Container type for all input parameters for the `nextIndex` function with signature `nextIndex()` and selector `[252, 126, 156, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "nextIndex", abi = "nextIndex()")]
    pub struct NextIndexCall;
    #[doc = "Container type for all input parameters for the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `[31, 121, 161, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(uint256)")]
    pub struct NullifierHashesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `outerLevels` function with signature `outerLevels()` and selector `[191, 188, 10, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "outerLevels", abi = "outerLevels()")]
    pub struct OuterLevelsCall;
    #[doc = "Container type for all input parameters for the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `[194, 35, 13, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "parseChainIdFromResourceId",
        abi = "parseChainIdFromResourceId(bytes32)"
    )]
    pub struct ParseChainIdFromResourceIdCall {
        pub resource_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    #[doc = "Container type for all input parameters for the `register` function with signature `register((address,bytes))` and selector `[178, 188, 110, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "register", abi = "register((address,bytes))")]
    pub struct RegisterCall {
        pub account: Account,
    }
    #[doc = "Container type for all input parameters for the `roots` function with signature `roots(uint256)` and selector `[194, 180, 10, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "roots", abi = "roots(uint256)")]
    pub struct RootsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `setHandler` function with signature `setHandler(address,uint32)` and selector `[114, 193, 173, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub handler: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `updateEdge` function with signature `updateEdge(uint256,uint32,bytes32)` and selector `[193, 146, 47, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "updateEdge", abi = "updateEdge(uint256,uint32,bytes32)")]
    pub struct UpdateEdgeCall {
        pub root: ethers::core::types::U256,
        pub leaf_index: u32,
        pub src_resource_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,bytes,uint256,uint256,uint256[],uint32,uint256)` and selector `[104, 206, 131, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "withdraw",
        abi = "withdraw(uint256,address,bytes,uint256,uint256,uint256[],uint32,uint256)"
    )]
    pub struct WithdrawCall {
        pub withdraw_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub relaying_fee: ethers::core::types::U256,
        pub merkle_proof: ::std::vec::Vec<ethers::core::types::U256>,
        pub commitment_index: u32,
        pub root: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdrawAndUnwrap` function with signature `withdrawAndUnwrap(uint256,address,bytes,uint256,uint256,uint256[],uint32,uint256,address)` and selector `[93, 195, 84, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "withdrawAndUnwrap",
        abi = "withdrawAndUnwrap(uint256,address,bytes,uint256,uint256,uint256[],uint32,uint256,address)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub withdraw_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub relaying_fee: ethers::core::types::U256,
        pub merkle_proof: ::std::vec::Vec<ethers::core::types::U256>,
        pub commitment_index: u32,
        pub root: ethers::core::types::U256,
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `wrapAndDeposit` function with signature `wrapAndDeposit(uint48,uint256,address,bytes,uint256,uint256,address)` and selector `[175, 70, 212, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "wrapAndDeposit",
        abi = "wrapAndDeposit(uint48,uint256,address,bytes,uint256,uint256,address)"
    )]
    pub struct WrapAndDepositCall {
        pub destination_chain_id: u64,
        pub deposit_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub relaying_fee: ethers::core::types::U256,
        pub token_address: ethers::core::types::Address,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum OpenVAnchorContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        ExecuteWrapping(ExecuteWrappingCall),
        _WithdrawAndUnwrap(_WithdrawAndUnwrapCall),
        CalculatePublicAmount(CalculatePublicAmountCall),
        Commitments(CommitmentsCall),
        ConfigureMaximumDepositLimit(ConfigureMaximumDepositLimitCall),
        ConfigureMinimalWithdrawalLimit(ConfigureMinimalWithdrawalLimitCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        CurrentRootIndex(CurrentRootIndexCall),
        Deposit(DepositCall),
        EdgeExistsForChain(EdgeExistsForChainCall),
        EdgeIndex(EdgeIndexCall),
        EdgeList(EdgeListCall),
        FilledSubtrees(FilledSubtreesCall),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
        GetHasher(GetHasherCall),
        GetLastRoot(GetLastRootCall),
        GetLatestNeighborEdges(GetLatestNeighborEdgesCall),
        GetLatestNeighborRoots(GetLatestNeighborRootsCall),
        GetLevels(GetLevelsCall),
        GetNextIndex(GetNextIndexCall),
        GetProposalNonce(GetProposalNonceCall),
        GetZeroHash(GetZeroHashCall),
        Handler(HandlerCall),
        HasEdge(HasEdgeCall),
        HashLeftRight(HashLeftRightCall),
        Hasher(HasherCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsKnownNeighborRoot(IsKnownNeighborRootCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpent(IsSpentCall),
        IsSpentArray(IsSpentArrayCall),
        IsValidRoots(IsValidRootsCall),
        LastBalance(LastBalanceCall),
        Levels(LevelsCall),
        MaxEdges(MaxEdgesCall),
        MaximumDepositAmount(MaximumDepositAmountCall),
        MinimalWithdrawalAmount(MinimalWithdrawalAmountCall),
        NeighborRoots(NeighborRootsCall),
        NextIndex(NextIndexCall),
        NullifierHashes(NullifierHashesCall),
        OuterLevels(OuterLevelsCall),
        ParseChainIdFromResourceId(ParseChainIdFromResourceIdCall),
        ProposalNonce(ProposalNonceCall),
        Register(RegisterCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        Token(TokenCall),
        UpdateEdge(UpdateEdgeCall),
        Withdraw(WithdrawCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
        WrapAndDeposit(WrapAndDepositCall),
    }
    impl ethers::core::abi::AbiDecode for OpenVAnchorContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EvmChainIdType(decoded));
            }
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::MaxExtAmount(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWrappingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::ExecuteWrapping(decoded));
            }
            if let Ok(decoded) =
                <_WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::_WithdrawAndUnwrap(
                    decoded,
                ));
            }
            if let Ok (decoded) = < CalculatePublicAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: CalculatePublicAmount (decoded)) }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Commitments(decoded));
            }
            if let Ok (decoded) = < ConfigureMaximumDepositLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: ConfigureMaximumDepositLimit (decoded)) }
            if let Ok (decoded) = < ConfigureMinimalWithdrawalLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: ConfigureMinimalWithdrawalLimit (decoded)) }
            if let Ok (decoded) = < CurrentNeighborRootIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: CurrentNeighborRootIndex (decoded)) }
            if let Ok(decoded) =
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::CurrentRootIndex(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EdgeExistsForChain(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetChainIdType(decoded));
            }
            if let Ok(decoded) =
                <GetHasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetHasher(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborEdgesCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: GetLatestNeighborEdges (decoded)) }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetLevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetLevels(decoded));
            }
            if let Ok(decoded) =
                <GetNextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetNextIndex(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <GetZeroHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetZeroHash(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <HasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Hasher(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Initialized(decoded));
            }
            if let Ok (decoded) = < IsKnownNeighborRootCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: IsKnownNeighborRoot (decoded)) }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::LastBalance(decoded));
            }
            if let Ok(decoded) =
                <LevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Levels(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::MaxEdges(decoded));
            }
            if let Ok (decoded) = < MaximumDepositAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: MaximumDepositAmount (decoded)) }
            if let Ok (decoded) = < MinimalWithdrawalAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: MinimalWithdrawalAmount (decoded)) }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <OuterLevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::OuterLevels(decoded));
            }
            if let Ok (decoded) = < ParseChainIdFromResourceIdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: ParseChainIdFromResourceId (decoded)) }
            if let Ok(decoded) =
                <ProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RegisterCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Register(decoded));
            }
            if let Ok(decoded) =
                <RootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::WithdrawAndUnwrap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WrapAndDepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::WrapAndDeposit(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OpenVAnchorContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OpenVAnchorContractCalls::EvmChainIdType(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::FieldSize(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::MaxExtAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::MaxFee(element) => element.encode(),
                OpenVAnchorContractCalls::RootHistorySize(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ZeroValue(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ExecuteWrapping(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::_WithdrawAndUnwrap(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::CalculatePublicAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Commitments(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ConfigureMaximumDepositLimit(
                    element,
                ) => element.encode(),
                OpenVAnchorContractCalls::ConfigureMinimalWithdrawalLimit(
                    element,
                ) => element.encode(),
                OpenVAnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::CurrentRootIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Deposit(element) => element.encode(),
                OpenVAnchorContractCalls::EdgeExistsForChain(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::EdgeIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::EdgeList(element) => element.encode(),
                OpenVAnchorContractCalls::FilledSubtrees(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetChainId(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetChainIdType(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetHasher(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLastRoot(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLevels(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetNextIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetProposalNonce(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetZeroHash(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Handler(element) => element.encode(),
                OpenVAnchorContractCalls::HasEdge(element) => element.encode(),
                OpenVAnchorContractCalls::HashLeftRight(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Hasher(element) => element.encode(),
                OpenVAnchorContractCalls::Initialize(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Initialized(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsKnownRoot(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsSpent(element) => element.encode(),
                OpenVAnchorContractCalls::IsSpentArray(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsValidRoots(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::LastBalance(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Levels(element) => element.encode(),
                OpenVAnchorContractCalls::MaxEdges(element) => element.encode(),
                OpenVAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::NeighborRoots(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::NextIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::NullifierHashes(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::OuterLevels(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ParseChainIdFromResourceId(
                    element,
                ) => element.encode(),
                OpenVAnchorContractCalls::ProposalNonce(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Register(element) => element.encode(),
                OpenVAnchorContractCalls::Roots(element) => element.encode(),
                OpenVAnchorContractCalls::SetHandler(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Token(element) => element.encode(),
                OpenVAnchorContractCalls::UpdateEdge(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Withdraw(element) => element.encode(),
                OpenVAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::WrapAndDeposit(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for OpenVAnchorContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OpenVAnchorContractCalls::EvmChainIdType(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::FieldSize(element) => element.fmt(f),
                OpenVAnchorContractCalls::MaxExtAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::MaxFee(element) => element.fmt(f),
                OpenVAnchorContractCalls::RootHistorySize(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::ZeroValue(element) => element.fmt(f),
                OpenVAnchorContractCalls::ExecuteWrapping(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::_WithdrawAndUnwrap(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::CalculatePublicAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Commitments(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::ConfigureMaximumDepositLimit(
                    element,
                ) => element.fmt(f),
                OpenVAnchorContractCalls::ConfigureMinimalWithdrawalLimit(
                    element,
                ) => element.fmt(f),
                OpenVAnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::CurrentRootIndex(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Deposit(element) => element.fmt(f),
                OpenVAnchorContractCalls::EdgeExistsForChain(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::EdgeIndex(element) => element.fmt(f),
                OpenVAnchorContractCalls::EdgeList(element) => element.fmt(f),
                OpenVAnchorContractCalls::FilledSubtrees(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetChainId(element) => element.fmt(f),
                OpenVAnchorContractCalls::GetChainIdType(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetHasher(element) => element.fmt(f),
                OpenVAnchorContractCalls::GetLastRoot(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetLevels(element) => element.fmt(f),
                OpenVAnchorContractCalls::GetNextIndex(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetProposalNonce(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetZeroHash(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Handler(element) => element.fmt(f),
                OpenVAnchorContractCalls::HasEdge(element) => element.fmt(f),
                OpenVAnchorContractCalls::HashLeftRight(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Hasher(element) => element.fmt(f),
                OpenVAnchorContractCalls::Initialize(element) => element.fmt(f),
                OpenVAnchorContractCalls::Initialized(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsKnownRoot(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsSpent(element) => element.fmt(f),
                OpenVAnchorContractCalls::IsSpentArray(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsValidRoots(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::LastBalance(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Levels(element) => element.fmt(f),
                OpenVAnchorContractCalls::MaxEdges(element) => element.fmt(f),
                OpenVAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::NeighborRoots(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::NextIndex(element) => element.fmt(f),
                OpenVAnchorContractCalls::NullifierHashes(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::OuterLevels(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::ParseChainIdFromResourceId(
                    element,
                ) => element.fmt(f),
                OpenVAnchorContractCalls::ProposalNonce(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Register(element) => element.fmt(f),
                OpenVAnchorContractCalls::Roots(element) => element.fmt(f),
                OpenVAnchorContractCalls::SetHandler(element) => element.fmt(f),
                OpenVAnchorContractCalls::Token(element) => element.fmt(f),
                OpenVAnchorContractCalls::UpdateEdge(element) => element.fmt(f),
                OpenVAnchorContractCalls::Withdraw(element) => element.fmt(f),
                OpenVAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::WrapAndDeposit(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<EvmChainIdTypeCall> for OpenVAnchorContractCalls {
        fn from(var: EvmChainIdTypeCall) -> Self {
            OpenVAnchorContractCalls::EvmChainIdType(var)
        }
    }
    impl ::std::convert::From<FieldSizeCall> for OpenVAnchorContractCalls {
        fn from(var: FieldSizeCall) -> Self {
            OpenVAnchorContractCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<MaxExtAmountCall> for OpenVAnchorContractCalls {
        fn from(var: MaxExtAmountCall) -> Self {
            OpenVAnchorContractCalls::MaxExtAmount(var)
        }
    }
    impl ::std::convert::From<MaxFeeCall> for OpenVAnchorContractCalls {
        fn from(var: MaxFeeCall) -> Self {
            OpenVAnchorContractCalls::MaxFee(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall> for OpenVAnchorContractCalls {
        fn from(var: RootHistorySizeCall) -> Self {
            OpenVAnchorContractCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for OpenVAnchorContractCalls {
        fn from(var: ZeroValueCall) -> Self {
            OpenVAnchorContractCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<ExecuteWrappingCall> for OpenVAnchorContractCalls {
        fn from(var: ExecuteWrappingCall) -> Self {
            OpenVAnchorContractCalls::ExecuteWrapping(var)
        }
    }
    impl ::std::convert::From<_WithdrawAndUnwrapCall> for OpenVAnchorContractCalls {
        fn from(var: _WithdrawAndUnwrapCall) -> Self {
            OpenVAnchorContractCalls::_WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<CalculatePublicAmountCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: CalculatePublicAmountCall) -> Self {
            OpenVAnchorContractCalls::CalculatePublicAmount(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for OpenVAnchorContractCalls {
        fn from(var: CommitmentsCall) -> Self {
            OpenVAnchorContractCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<ConfigureMaximumDepositLimitCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: ConfigureMaximumDepositLimitCall) -> Self {
            OpenVAnchorContractCalls::ConfigureMaximumDepositLimit(var)
        }
    }
    impl ::std::convert::From<ConfigureMinimalWithdrawalLimitCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: ConfigureMinimalWithdrawalLimitCall) -> Self {
            OpenVAnchorContractCalls::ConfigureMinimalWithdrawalLimit(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            OpenVAnchorContractCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<CurrentRootIndexCall> for OpenVAnchorContractCalls {
        fn from(var: CurrentRootIndexCall) -> Self {
            OpenVAnchorContractCalls::CurrentRootIndex(var)
        }
    }
    impl ::std::convert::From<DepositCall> for OpenVAnchorContractCalls {
        fn from(var: DepositCall) -> Self {
            OpenVAnchorContractCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall> for OpenVAnchorContractCalls {
        fn from(var: EdgeExistsForChainCall) -> Self {
            OpenVAnchorContractCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for OpenVAnchorContractCalls {
        fn from(var: EdgeIndexCall) -> Self {
            OpenVAnchorContractCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for OpenVAnchorContractCalls {
        fn from(var: EdgeListCall) -> Self {
            OpenVAnchorContractCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for OpenVAnchorContractCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            OpenVAnchorContractCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for OpenVAnchorContractCalls {
        fn from(var: GetChainIdCall) -> Self {
            OpenVAnchorContractCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetChainIdTypeCall> for OpenVAnchorContractCalls {
        fn from(var: GetChainIdTypeCall) -> Self {
            OpenVAnchorContractCalls::GetChainIdType(var)
        }
    }
    impl ::std::convert::From<GetHasherCall> for OpenVAnchorContractCalls {
        fn from(var: GetHasherCall) -> Self {
            OpenVAnchorContractCalls::GetHasher(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for OpenVAnchorContractCalls {
        fn from(var: GetLastRootCall) -> Self {
            OpenVAnchorContractCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborEdgesCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: GetLatestNeighborEdgesCall) -> Self {
            OpenVAnchorContractCalls::GetLatestNeighborEdges(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            OpenVAnchorContractCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetLevelsCall> for OpenVAnchorContractCalls {
        fn from(var: GetLevelsCall) -> Self {
            OpenVAnchorContractCalls::GetLevels(var)
        }
    }
    impl ::std::convert::From<GetNextIndexCall> for OpenVAnchorContractCalls {
        fn from(var: GetNextIndexCall) -> Self {
            OpenVAnchorContractCalls::GetNextIndex(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall> for OpenVAnchorContractCalls {
        fn from(var: GetProposalNonceCall) -> Self {
            OpenVAnchorContractCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<GetZeroHashCall> for OpenVAnchorContractCalls {
        fn from(var: GetZeroHashCall) -> Self {
            OpenVAnchorContractCalls::GetZeroHash(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for OpenVAnchorContractCalls {
        fn from(var: HandlerCall) -> Self {
            OpenVAnchorContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for OpenVAnchorContractCalls {
        fn from(var: HasEdgeCall) -> Self {
            OpenVAnchorContractCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for OpenVAnchorContractCalls {
        fn from(var: HashLeftRightCall) -> Self {
            OpenVAnchorContractCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<HasherCall> for OpenVAnchorContractCalls {
        fn from(var: HasherCall) -> Self {
            OpenVAnchorContractCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for OpenVAnchorContractCalls {
        fn from(var: InitializeCall) -> Self {
            OpenVAnchorContractCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitializedCall> for OpenVAnchorContractCalls {
        fn from(var: InitializedCall) -> Self {
            OpenVAnchorContractCalls::Initialized(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            OpenVAnchorContractCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for OpenVAnchorContractCalls {
        fn from(var: IsKnownRootCall) -> Self {
            OpenVAnchorContractCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for OpenVAnchorContractCalls {
        fn from(var: IsSpentCall) -> Self {
            OpenVAnchorContractCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for OpenVAnchorContractCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            OpenVAnchorContractCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for OpenVAnchorContractCalls {
        fn from(var: IsValidRootsCall) -> Self {
            OpenVAnchorContractCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LastBalanceCall> for OpenVAnchorContractCalls {
        fn from(var: LastBalanceCall) -> Self {
            OpenVAnchorContractCalls::LastBalance(var)
        }
    }
    impl ::std::convert::From<LevelsCall> for OpenVAnchorContractCalls {
        fn from(var: LevelsCall) -> Self {
            OpenVAnchorContractCalls::Levels(var)
        }
    }
    impl ::std::convert::From<MaxEdgesCall> for OpenVAnchorContractCalls {
        fn from(var: MaxEdgesCall) -> Self {
            OpenVAnchorContractCalls::MaxEdges(var)
        }
    }
    impl ::std::convert::From<MaximumDepositAmountCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: MaximumDepositAmountCall) -> Self {
            OpenVAnchorContractCalls::MaximumDepositAmount(var)
        }
    }
    impl ::std::convert::From<MinimalWithdrawalAmountCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: MinimalWithdrawalAmountCall) -> Self {
            OpenVAnchorContractCalls::MinimalWithdrawalAmount(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for OpenVAnchorContractCalls {
        fn from(var: NeighborRootsCall) -> Self {
            OpenVAnchorContractCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NextIndexCall> for OpenVAnchorContractCalls {
        fn from(var: NextIndexCall) -> Self {
            OpenVAnchorContractCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for OpenVAnchorContractCalls {
        fn from(var: NullifierHashesCall) -> Self {
            OpenVAnchorContractCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<OuterLevelsCall> for OpenVAnchorContractCalls {
        fn from(var: OuterLevelsCall) -> Self {
            OpenVAnchorContractCalls::OuterLevels(var)
        }
    }
    impl ::std::convert::From<ParseChainIdFromResourceIdCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: ParseChainIdFromResourceIdCall) -> Self {
            OpenVAnchorContractCalls::ParseChainIdFromResourceId(var)
        }
    }
    impl ::std::convert::From<ProposalNonceCall> for OpenVAnchorContractCalls {
        fn from(var: ProposalNonceCall) -> Self {
            OpenVAnchorContractCalls::ProposalNonce(var)
        }
    }
    impl ::std::convert::From<RegisterCall> for OpenVAnchorContractCalls {
        fn from(var: RegisterCall) -> Self {
            OpenVAnchorContractCalls::Register(var)
        }
    }
    impl ::std::convert::From<RootsCall> for OpenVAnchorContractCalls {
        fn from(var: RootsCall) -> Self {
            OpenVAnchorContractCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for OpenVAnchorContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            OpenVAnchorContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<TokenCall> for OpenVAnchorContractCalls {
        fn from(var: TokenCall) -> Self {
            OpenVAnchorContractCalls::Token(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for OpenVAnchorContractCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            OpenVAnchorContractCalls::UpdateEdge(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for OpenVAnchorContractCalls {
        fn from(var: WithdrawCall) -> Self {
            OpenVAnchorContractCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for OpenVAnchorContractCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            OpenVAnchorContractCalls::WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WrapAndDepositCall> for OpenVAnchorContractCalls {
        fn from(var: WrapAndDepositCall) -> Self {
            OpenVAnchorContractCalls::WrapAndDeposit(var)
        }
    }
    #[doc = "Container type for all return fields from the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `[139, 126, 135, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EvmChainIdTypeReturn(pub [u8; 2]);
    #[doc = "Container type for all return fields from the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct FieldSizeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `[127, 226, 79, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaxExtAmountReturn(pub I256);
    #[doc = "Container type for all return fields from the `MAX_FEE` function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaxFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `[205, 135, 163, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct RootHistorySizeReturn(pub u32);
    #[doc = "Container type for all return fields from the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `[236, 115, 41, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ZeroValueReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_executeWrapping` function with signature `_executeWrapping(address,address,uint256)` and selector `[99, 56, 188, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ExecuteWrappingReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `[37, 112, 183, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CalculatePublicAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `commitments` function with signature `commitments(uint256)` and selector `[73, 206, 137, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CommitmentsReturn(pub bool);
    #[doc = "Container type for all return fields from the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `[93, 45, 118, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CurrentNeighborRootIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `currentRootIndex` function with signature `currentRootIndex()` and selector `[144, 238, 176, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CurrentRootIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EdgeExistsForChainReturn(pub bool);
    #[doc = "Container type for all return fields from the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `[231, 14, 168, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EdgeIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `edgeList` function with signature `edgeList(uint256)` and selector `[219, 201, 22, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EdgeListReturn {
        pub chain_id: ethers::core::types::U256,
        pub root: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `[241, 120, 228, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct FilledSubtreesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetChainIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getChainIdType` function with signature `getChainIdType()` and selector `[76, 131, 12, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetChainIdTypeReturn(pub u64);
    #[doc = "Container type for all return fields from the `getHasher` function with signature `getHasher()` and selector `[234, 73, 93, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetHasherReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getLastRoot` function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLastRootReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `[140, 13, 52, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLatestNeighborEdgesReturn(
        pub  ::std::vec::Vec<(
            ethers::core::types::U256,
            ethers::core::types::U256,
            ethers::core::types::U256,
            [u8; 32],
        )>,
    );
    #[doc = "Container type for all return fields from the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLatestNeighborRootsReturn(
        pub ::std::vec::Vec<ethers::core::types::U256>,
    );
    #[doc = "Container type for all return fields from the `getLevels` function with signature `getLevels()` and selector `[12, 57, 74, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLevelsReturn(pub u32);
    #[doc = "Container type for all return fields from the `getNextIndex` function with signature `getNextIndex()` and selector `[14, 183, 96, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetNextIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `getProposalNonce` function with signature `getProposalNonce()` and selector `[11, 39, 251, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetProposalNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `[48, 94, 158, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetZeroHashReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `handler` function with signature `handler()` and selector `[200, 9, 22, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HandlerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `hasEdge` function with signature `hasEdge(uint256)` and selector `[146, 21, 99, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HasEdgeReturn(pub bool);
    #[doc = "Container type for all return fields from the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `[91, 185, 57, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HashLeftRightReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hasher` function with signature `hasher()` and selector `[237, 51, 99, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HasherReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `initialized` function with signature `initialized()` and selector `[21, 142, 249, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct InitializedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `[59, 250, 141, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsKnownNeighborRootReturn(pub bool);
    #[doc = "Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `[166, 35, 42, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsKnownRootReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSpent` function with signature `isSpent(uint256)` and selector `[90, 18, 158, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsSpentReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `[234, 101, 186, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsSpentArrayReturn(pub ::std::vec::Vec<bool>);
    #[doc = "Container type for all return fields from the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `[183, 94, 103, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsValidRootsReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastBalance` function with signature `lastBalance()` and selector `[143, 28, 86, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct LastBalanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `levels` function with signature `levels()` and selector `[78, 207, 81, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct LevelsReturn(pub u32);
    #[doc = "Container type for all return fields from the `maxEdges` function with signature `maxEdges()` and selector `[113, 82, 60, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaxEdgesReturn(pub u8);
    #[doc = "Container type for all return fields from the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `[120, 171, 180, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaximumDepositAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `minimalWithdrawalAmount` function with signature `minimalWithdrawalAmount()` and selector `[132, 11, 39, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MinimalWithdrawalAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `[67, 231, 17, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct NeighborRootsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `nextIndex` function with signature `nextIndex()` and selector `[252, 126, 156, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct NextIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `[31, 121, 161, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct NullifierHashesReturn(pub bool);
    #[doc = "Container type for all return fields from the `outerLevels` function with signature `outerLevels()` and selector `[191, 188, 10, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct OuterLevelsReturn(pub u32);
    #[doc = "Container type for all return fields from the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `[194, 35, 13, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ParseChainIdFromResourceIdReturn(pub u64);
    #[doc = "Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ProposalNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `roots` function with signature `roots(uint256)` and selector `[194, 180, 10, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct RootsReturn {
        pub root: ethers::core::types::U256,
        pub latest_leafindex: u32,
    }
    #[doc = "Container type for all return fields from the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct TokenReturn(pub ethers::core::types::Address);
    #[doc = "`Edge(uint256,uint256,uint256,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct Edge {
        pub chain_id: ethers::core::types::U256,
        pub root: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    #[doc = "`Account(address,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct Account {
        pub owner: ethers::core::types::Address,
        pub key_data: ethers::core::types::Bytes,
    }
}