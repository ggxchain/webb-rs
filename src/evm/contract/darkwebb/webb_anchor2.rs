pub use webbanchor2contract_mod::*;
#[allow(clippy::too_many_arguments)]
mod webbanchor2contract_mod {
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
    #[doc = "WEBBAnchor2Contract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WEBBANCHOR2CONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IVerifier\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_denomination\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"_merkleTreeHeight\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"_chainID\",\"type\":\"uint32\"},{\"internalType\":\"contract IMintableCompToken\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_bridge\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"name\":\"Deposit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeAddition\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32[1]\",\"name\":\"roots\",\"type\":\"bytes32[1]\"}],\"name\":\"RootHistoryRecorded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32[1]\",\"name\":\"roots\",\"type\":\"bytes32[1]\"}],\"name\":\"RootHistoryUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"nullifierHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"}],\"name\":\"Withdrawal\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"}],\"name\":\"addEdge\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"bridge\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"chainID\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"denomination\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_commitment\",\"type\":\"bytes32\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"bytes32[1]\",\"name\":\"roots\",\"type\":\"bytes32[1]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\"}],\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\"}],\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"neighborChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"}],\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\"}],\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"spent\",\"type\":\"bool[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_bridge\",\"type\":\"address\"}],\"name\":\"setBridge\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"}],\"name\":\"setHandler\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[8]\",\"name\":\"_proof\",\"type\":\"uint256[8]\"}],\"name\":\"unpackProof\",\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2][2]\",\"name\":\"\",\"type\":\"uint256[2][2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"}],\"name\":\"updateEdge\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"verifier\",\"outputs\":[{\"internalType\":\"contract IVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"_roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"address payable\",\"name\":\"_relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_refund\",\"type\":\"uint256\"}],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"zeros\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct WEBBAnchor2Contract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for WEBBAnchor2Contract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for WEBBAnchor2Contract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(WEBBAnchor2Contract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> WEBBAnchor2Contract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                WEBBANCHOR2CONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
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
        #[doc = "Calls the contract's `addEdge` (0x3b9e44de) function"]
        pub fn add_edge(
            &self,
            source_chain_id: ethers::core::types::U256,
            root: [u8; 32],
            height: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [59, 158, 68, 222],
                    (source_chain_id, root, height),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridge` (0xe78cea92) function"]
        pub fn bridge(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 140, 234, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chainID` (0xadc879e9) function"]
        pub fn chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([173, 200, 121, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x839df945) function"]
        pub fn commitments(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 157, 249, 69], p0)
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
        #[doc = "Calls the contract's `denomination` (0x8bca6d16) function"]
        pub fn denomination(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([139, 202, 109, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb214faa5) function"]
        pub fn deposit(
            &self,
            commitment: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 20, 250, 165], commitment)
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
                [u8; 32],
                ethers::core::types::U256,
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
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([241, 120, 228, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRoot` (0xba70f757) function"]
        pub fn get_last_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborRoots` (0x1e627617) function"]
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [[u8; 32]; 1]>
        {
            self.0
                .method_hash([30, 98, 118, 23], ())
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
        #[doc = "Calls the contract's `hashLeftRight` (0x8ea3099e) function"]
        pub fn hash_left_right(
            &self,
            hasher: ethers::core::types::Address,
            left: [u8; 32],
            right: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 163, 9, 158], (hasher, left, right))
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
        #[doc = "Calls the contract's `isKnownNeighborRoot` (0x11e4dcb9) function"]
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ethers::core::types::U256,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([17, 228, 220, 185], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0x6d9833e3) function"]
        pub fn is_known_root(
            &self,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 152, 51, 227], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpent` (0xe5285dcc) function"]
        pub fn is_spent(
            &self,
            nullifier_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 40, 93, 204], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentArray` (0x9fa12d0b) function"]
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, Vec<bool>> {
            self.0
                .method_hash([159, 161, 45, 11], nullifier_hashes)
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
        #[doc = "Calls the contract's `neighborRoots` (0x43e7119f) function"]
        pub fn neighbor_roots(
            &self,
            p0: ethers::core::types::U256,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
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
        #[doc = "Calls the contract's `nullifierHashes` (0x17cc915c) function"]
        pub fn nullifier_hashes(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 204, 145, 92], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBridge` (0x8dd14802) function"]
        pub fn set_bridge(
            &self,
            bridge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 209, 72, 2], bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0xbac426d0) function"]
        pub fn set_handler(
            &self,
            handler: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 196, 38, 208], handler)
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
        #[doc = "Calls the contract's `unpackProof` (0xf5ab0dd6) function"]
        pub fn unpack_proof(
            &self,
            proof: [ethers::core::types::U256; 8usize],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [ethers::core::types::U256; 2],
                [[ethers::core::types::U256; 2]; 2],
                [ethers::core::types::U256; 2],
            ),
        > {
            self.0
                .method_hash([245, 171, 13, 214], proof)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateEdge` (0x44347ba9) function"]
        pub fn update_edge(
            &self,
            source_chain_id: ethers::core::types::U256,
            root: [u8; 32],
            height: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [68, 52, 123, 169],
                    (source_chain_id, root, height),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifier` (0x2b7ac3f3) function"]
        pub fn verifier(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xd6bb4d8c) function"]
        pub fn withdraw(
            &self,
            proof: Vec<u8>,
            roots: Vec<u8>,
            nullifier_hash: [u8; 32],
            recipient: ethers::core::types::Address,
            relayer: ethers::core::types::Address,
            fee: ethers::core::types::U256,
            refund: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [214, 187, 77, 140],
                    (
                        proof,
                        roots,
                        nullifier_hash,
                        recipient,
                        relayer,
                        fee,
                        refund,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `zeros` (0xe8295588) function"]
        pub fn zeros(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 41, 85, 136], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
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
        #[doc = "Gets the contract's `RootHistoryRecorded` event"]
        pub fn root_history_recorded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RootHistoryRecordedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `RootHistoryUpdate` event"]
        pub fn root_history_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RootHistoryUpdateFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdrawal` event"]
        pub fn withdrawal_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawalFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, WEBBAnchor2ContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(bytes32,uint32,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub leaf_index: u32,
        pub timestamp: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "EdgeAddition",
        abi = "EdgeAddition(uint256,uint256,bytes32)"
    )]
    pub struct EdgeAdditionFilter {
        pub chain_id: ethers::core::types::U256,
        pub height: ethers::core::types::U256,
        pub merkle_root: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "EdgeUpdate",
        abi = "EdgeUpdate(uint256,uint256,bytes32)"
    )]
    pub struct EdgeUpdateFilter {
        pub chain_id: ethers::core::types::U256,
        pub height: ethers::core::types::U256,
        pub merkle_root: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "RootHistoryRecorded",
        abi = "RootHistoryRecorded(uint256,bytes32[1])"
    )]
    pub struct RootHistoryRecordedFilter {
        pub timestamp: ethers::core::types::U256,
        pub roots: [[u8; 32]; 1],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "RootHistoryUpdate",
        abi = "RootHistoryUpdate(uint256,bytes32[1])"
    )]
    pub struct RootHistoryUpdateFilter {
        pub timestamp: ethers::core::types::U256,
        pub roots: [[u8; 32]; 1],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "Withdrawal",
        abi = "Withdrawal(address,bytes32,address,uint256)"
    )]
    pub struct WithdrawalFilter {
        pub to: ethers::core::types::Address,
        pub nullifier_hash: [u8; 32],
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum WEBBAnchor2ContractEvents {
        DepositFilter(DepositFilter),
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        RootHistoryRecordedFilter(RootHistoryRecordedFilter),
        RootHistoryUpdateFilter(RootHistoryUpdateFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ethers::core::abi::Tokenizable for WEBBAnchor2ContractEvents {
        fn from_token(
            token: ethers::core::abi::Token,
        ) -> Result<Self, ethers::core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::from_token(token.clone()) {
                return Ok(WEBBAnchor2ContractEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = EdgeAdditionFilter::from_token(token.clone()) {
                return Ok(WEBBAnchor2ContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::from_token(token.clone()) {
                return Ok(WEBBAnchor2ContractEvents::EdgeUpdateFilter(decoded));
            }
            if let Ok(decoded) =
                RootHistoryRecordedFilter::from_token(token.clone())
            {
                return Ok(
                    WEBBAnchor2ContractEvents::RootHistoryRecordedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                RootHistoryUpdateFilter::from_token(token.clone())
            {
                return Ok(WEBBAnchor2ContractEvents::RootHistoryUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = WithdrawalFilter::from_token(token.clone()) {
                return Ok(WEBBAnchor2ContractEvents::WithdrawalFilter(decoded));
            }
            Err(ethers::core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers::core::abi::Token {
            match self {
                WEBBAnchor2ContractEvents::DepositFilter(element) => {
                    element.into_token()
                }
                WEBBAnchor2ContractEvents::EdgeAdditionFilter(element) => {
                    element.into_token()
                }
                WEBBAnchor2ContractEvents::EdgeUpdateFilter(element) => {
                    element.into_token()
                }
                WEBBAnchor2ContractEvents::RootHistoryRecordedFilter(
                    element,
                ) => element.into_token(),
                WEBBAnchor2ContractEvents::RootHistoryUpdateFilter(element) => {
                    element.into_token()
                }
                WEBBAnchor2ContractEvents::WithdrawalFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers::core::abi::TokenizableItem for WEBBAnchor2ContractEvents {}
    impl ethers::contract::EthLogDecode for WEBBAnchor2ContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(WEBBAnchor2ContractEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(WEBBAnchor2ContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(WEBBAnchor2ContractEvents::EdgeUpdateFilter(decoded));
            }
            if let Ok(decoded) = RootHistoryRecordedFilter::decode_log(log) {
                return Ok(
                    WEBBAnchor2ContractEvents::RootHistoryRecordedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RootHistoryUpdateFilter::decode_log(log) {
                return Ok(WEBBAnchor2ContractEvents::RootHistoryUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(WEBBAnchor2ContractEvents::WithdrawalFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
}
