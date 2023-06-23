pub use fungible_token_wrapper_contract::*;
#[doc = r" This module was auto-generated with ethers-rs Abigen."]
#[doc = r" More information at: <https://github.com/gakonst/ethers-rs>"]
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod fungible_token_wrapper_contract {
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"HandlerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MINTER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PAUSER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnFrom\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feePercentage\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_deposit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountToWrap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFee\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountToWrap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeFromAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"historicalTokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"historicallyValid\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_feePercentage\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_feeRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isNativeAllowed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isNativeAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_feePercentage\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_feeRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeRecipient\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_isNativeAllowed\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNativeAllowed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapAndSendTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapFor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"valid\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapFor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapForAndSendTo\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wrappingLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = "The parsed JSON ABI of the contract."]
    pub static FUNGIBLETOKENWRAPPERCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    # [rustfmt :: skip] const __BYTECODE : & [u8] = & [96 , 128 , 96 , 64 , 82 , 52 , 128 , 21 , 98 , 0 , 0 , 17 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 64 , 81 , 98 , 0 , 67 , 92 , 56 , 3 , 128 , 98 , 0 , 67 , 92 , 131 , 57 , 129 , 1 , 96 , 64 , 129 , 144 , 82 , 98 , 0 , 0 , 52 , 145 , 98 , 0 , 2 , 237 , 86 , 91 , 129 , 129 , 129 , 129 , 129 , 129 , 96 , 6 , 98 , 0 , 0 , 72 , 131 , 130 , 98 , 0 , 3 , 229 , 86 , 91 , 80 , 96 , 7 , 98 , 0 , 0 , 87 , 130 , 130 , 98 , 0 , 3 , 229 , 86 , 91 , 80 , 80 , 96 , 8 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 80 , 98 , 0 , 0 , 113 , 96 , 0 , 51 , 98 , 0 , 0 , 219 , 86 , 91 , 98 , 0 , 0 , 157 , 127 , 159 , 45 , 240 , 254 , 210 , 199 , 118 , 72 , 222 , 88 , 96 , 164 , 204 , 80 , 140 , 208 , 129 , 140 , 133 , 184 , 184 , 161 , 171 , 76 , 238 , 239 , 141 , 152 , 28 , 137 , 86 , 166 , 51 , 98 , 0 , 0 , 219 , 86 , 91 , 98 , 0 , 0 , 201 , 127 , 101 , 215 , 162 , 142 , 50 , 101 , 179 , 122 , 100 , 116 , 146 , 159 , 51 , 101 , 33 , 179 , 50 , 193 , 104 , 27 , 147 , 63 , 108 , 185 , 243 , 55 , 102 , 115 , 68 , 13 , 134 , 42 , 51 , 98 , 0 , 0 , 219 , 86 , 91 , 80 , 80 , 96 , 1 , 96 , 9 , 85 , 80 , 98 , 0 , 4 , 177 , 146 , 80 , 80 , 80 , 86 , 91 , 98 , 0 , 0 , 231 , 130 , 130 , 98 , 0 , 0 , 235 , 86 , 91 , 80 , 80 , 86 , 91 , 98 , 0 , 1 , 2 , 130 , 130 , 98 , 0 , 1 , 46 , 96 , 32 , 27 , 98 , 0 , 33 , 154 , 23 , 96 , 32 , 28 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 144 , 145 , 32 , 98 , 0 , 1 , 41 , 145 , 131 , 144 , 98 , 0 , 34 , 5 , 98 , 0 , 1 , 182 , 130 , 27 , 23 , 144 , 28 , 86 , 91 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 132 , 82 , 144 , 145 , 82 , 144 , 32 , 84 , 96 , 255 , 22 , 98 , 0 , 0 , 231 , 87 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 128 , 134 , 82 , 146 , 82 , 128 , 132 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 147 , 23 , 144 , 146 , 85 , 144 , 81 , 51 , 146 , 133 , 145 , 127 , 47 , 135 , 136 , 17 , 126 , 126 , 255 , 29 , 130 , 233 , 38 , 236 , 121 , 73 , 1 , 209 , 124 , 120 , 2 , 74 , 80 , 39 , 9 , 64 , 48 , 69 , 64 , 167 , 51 , 101 , 111 , 13 , 145 , 144 , 164 , 80 , 80 , 86 , 91 , 96 , 0 , 98 , 0 , 1 , 205 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 98 , 0 , 1 , 214 , 86 , 91 , 144 , 80 , 91 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 1 , 131 , 1 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 84 , 98 , 0 , 2 , 31 , 87 , 80 , 129 , 84 , 96 , 1 , 129 , 129 , 1 , 132 , 85 , 96 , 0 , 132 , 129 , 82 , 96 , 32 , 128 , 130 , 32 , 144 , 147 , 1 , 132 , 144 , 85 , 132 , 84 , 132 , 130 , 82 , 130 , 134 , 1 , 144 , 147 , 82 , 96 , 64 , 144 , 32 , 145 , 144 , 145 , 85 , 98 , 0 , 1 , 208 , 86 , 91 , 80 , 96 , 0 , 98 , 0 , 1 , 208 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 65 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 96 , 0 , 130 , 96 , 31 , 131 , 1 , 18 , 98 , 0 , 2 , 80 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 96 , 1 , 96 , 1 , 96 , 64 , 27 , 3 , 128 , 130 , 17 , 21 , 98 , 0 , 2 , 109 , 87 , 98 , 0 , 2 , 109 , 98 , 0 , 2 , 40 , 86 , 91 , 96 , 64 , 81 , 96 , 31 , 131 , 1 , 96 , 31 , 25 , 144 , 129 , 22 , 96 , 63 , 1 , 22 , 129 , 1 , 144 , 130 , 130 , 17 , 129 , 131 , 16 , 23 , 21 , 98 , 0 , 2 , 152 , 87 , 98 , 0 , 2 , 152 , 98 , 0 , 2 , 40 , 86 , 91 , 129 , 96 , 64 , 82 , 131 , 129 , 82 , 96 , 32 , 146 , 80 , 134 , 131 , 133 , 136 , 1 , 1 , 17 , 21 , 98 , 0 , 2 , 181 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 145 , 80 , 91 , 131 , 130 , 16 , 21 , 98 , 0 , 2 , 217 , 87 , 133 , 130 , 1 , 131 , 1 , 81 , 129 , 131 , 1 , 132 , 1 , 82 , 144 , 130 , 1 , 144 , 98 , 0 , 2 , 186 , 86 , 91 , 96 , 0 , 147 , 129 , 1 , 144 , 146 , 1 , 146 , 144 , 146 , 82 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 98 , 0 , 3 , 1 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 81 , 96 , 1 , 96 , 1 , 96 , 64 , 27 , 3 , 128 , 130 , 17 , 21 , 98 , 0 , 3 , 25 , 87 , 96 , 0 , 128 , 253 , 91 , 98 , 0 , 3 , 39 , 134 , 131 , 135 , 1 , 98 , 0 , 2 , 62 , 86 , 91 , 147 , 80 , 96 , 32 , 133 , 1 , 81 , 145 , 80 , 128 , 130 , 17 , 21 , 98 , 0 , 3 , 62 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 98 , 0 , 3 , 77 , 133 , 130 , 134 , 1 , 98 , 0 , 2 , 62 , 86 , 91 , 145 , 80 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 1 , 129 , 129 , 28 , 144 , 130 , 22 , 128 , 98 , 0 , 3 , 108 , 87 , 96 , 127 , 130 , 22 , 145 , 80 , 91 , 96 , 32 , 130 , 16 , 129 , 3 , 98 , 0 , 3 , 141 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 34 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 145 , 144 , 80 , 86 , 91 , 96 , 31 , 130 , 17 , 21 , 98 , 0 , 1 , 41 , 87 , 96 , 0 , 129 , 129 , 82 , 96 , 32 , 129 , 32 , 96 , 31 , 133 , 1 , 96 , 5 , 28 , 129 , 1 , 96 , 32 , 134 , 16 , 21 , 98 , 0 , 3 , 188 , 87 , 80 , 128 , 91 , 96 , 31 , 133 , 1 , 96 , 5 , 28 , 130 , 1 , 145 , 80 , 91 , 129 , 129 , 16 , 21 , 98 , 0 , 3 , 221 , 87 , 130 , 129 , 85 , 96 , 1 , 1 , 98 , 0 , 3 , 200 , 86 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 129 , 81 , 96 , 1 , 96 , 1 , 96 , 64 , 27 , 3 , 129 , 17 , 21 , 98 , 0 , 4 , 1 , 87 , 98 , 0 , 4 , 1 , 98 , 0 , 2 , 40 , 86 , 91 , 98 , 0 , 4 , 25 , 129 , 98 , 0 , 4 , 18 , 132 , 84 , 98 , 0 , 3 , 87 , 86 , 91 , 132 , 98 , 0 , 3 , 147 , 86 , 91 , 96 , 32 , 128 , 96 , 31 , 131 , 17 , 96 , 1 , 129 , 20 , 98 , 0 , 4 , 81 , 87 , 96 , 0 , 132 , 21 , 98 , 0 , 4 , 56 , 87 , 80 , 133 , 131 , 1 , 81 , 91 , 96 , 0 , 25 , 96 , 3 , 134 , 144 , 27 , 28 , 25 , 22 , 96 , 1 , 133 , 144 , 27 , 23 , 133 , 85 , 98 , 0 , 3 , 221 , 86 , 91 , 96 , 0 , 133 , 129 , 82 , 96 , 32 , 129 , 32 , 96 , 31 , 25 , 134 , 22 , 145 , 91 , 130 , 129 , 16 , 21 , 98 , 0 , 4 , 130 , 87 , 136 , 134 , 1 , 81 , 130 , 85 , 148 , 132 , 1 , 148 , 96 , 1 , 144 , 145 , 1 , 144 , 132 , 1 , 98 , 0 , 4 , 97 , 86 , 91 , 80 , 133 , 130 , 16 , 21 , 98 , 0 , 4 , 161 , 87 , 135 , 133 , 1 , 81 , 96 , 0 , 25 , 96 , 3 , 136 , 144 , 27 , 96 , 248 , 22 , 28 , 25 , 22 , 129 , 85 , 91 , 80 , 80 , 80 , 80 , 80 , 96 , 1 , 144 , 129 , 27 , 1 , 144 , 85 , 80 , 86 , 91 , 97 , 62 , 155 , 128 , 98 , 0 , 4 , 193 , 96 , 0 , 57 , 96 , 0 , 243 , 254 , 96 , 128 , 96 , 64 , 82 , 96 , 4 , 54 , 16 , 97 , 3 , 140 , 87 , 96 , 0 , 53 , 96 , 224 , 28 , 128 , 99 , 133 , 192 , 10 , 232 , 17 , 97 , 1 , 220 , 87 , 128 , 99 , 186 , 196 , 38 , 208 , 17 , 97 , 1 , 2 , 87 , 128 , 99 , 206 , 215 , 47 , 135 , 17 , 97 , 0 , 160 , 87 , 128 , 99 , 230 , 58 , 177 , 233 , 17 , 97 , 0 , 111 , 87 , 128 , 99 , 230 , 58 , 177 , 233 , 20 , 97 , 10 , 111 , 87 , 128 , 99 , 246 , 62 , 187 , 69 , 20 , 97 , 10 , 145 , 87 , 128 , 99 , 250 , 224 , 149 , 154 , 20 , 97 , 10 , 177 , 87 , 128 , 99 , 252 , 151 , 166 , 82 , 20 , 97 , 10 , 209 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 206 , 215 , 47 , 135 , 20 , 97 , 9 , 244 , 87 , 128 , 99 , 213 , 57 , 19 , 147 , 20 , 97 , 10 , 13 , 87 , 128 , 99 , 213 , 71 , 116 , 31 , 20 , 97 , 10 , 47 , 87 , 128 , 99 , 221 , 98 , 237 , 62 , 20 , 97 , 10 , 79 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 194 , 174 , 71 , 32 , 17 , 97 , 0 , 220 , 87 , 128 , 99 , 194 , 174 , 71 , 32 , 20 , 97 , 9 , 126 , 87 , 128 , 99 , 200 , 9 , 22 , 212 , 20 , 97 , 9 , 158 , 87 , 128 , 99 , 202 , 21 , 200 , 115 , 20 , 97 , 9 , 190 , 87 , 128 , 99 , 204 , 60 , 116 , 161 , 20 , 97 , 9 , 222 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 186 , 196 , 38 , 208 , 20 , 97 , 9 , 43 , 87 , 128 , 99 , 191 , 55 , 108 , 122 , 20 , 97 , 9 , 75 , 87 , 128 , 99 , 193 , 135 , 100 , 83 , 20 , 97 , 9 , 94 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 160 , 1 , 236 , 221 , 17 , 97 , 1 , 122 , 87 , 128 , 99 , 170 , 108 , 168 , 8 , 17 , 97 , 1 , 73 , 87 , 128 , 99 , 170 , 108 , 168 , 8 , 20 , 97 , 8 , 143 , 87 , 128 , 99 , 172 , 138 , 38 , 12 , 20 , 97 , 8 , 177 , 87 , 128 , 99 , 177 , 203 , 162 , 88 , 20 , 97 , 8 , 225 , 87 , 128 , 99 , 179 , 228 , 8 , 63 , 20 , 97 , 9 , 17 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 160 , 1 , 236 , 221 , 20 , 97 , 8 , 12 , 87 , 128 , 99 , 162 , 23 , 253 , 223 , 20 , 97 , 8 , 58 , 87 , 128 , 99 , 164 , 87 , 194 , 215 , 20 , 97 , 8 , 79 , 87 , 128 , 99 , 169 , 5 , 156 , 187 , 20 , 97 , 8 , 111 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 144 , 16 , 208 , 124 , 17 , 97 , 1 , 182 , 87 , 128 , 99 , 144 , 16 , 208 , 124 , 20 , 97 , 7 , 151 , 87 , 128 , 99 , 145 , 209 , 72 , 84 , 20 , 97 , 7 , 183 , 87 , 128 , 99 , 149 , 216 , 155 , 65 , 20 , 97 , 7 , 215 , 87 , 128 , 99 , 150 , 205 , 77 , 254 , 20 , 97 , 7 , 236 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 133 , 192 , 10 , 232 , 20 , 97 , 7 , 55 , 87 , 128 , 99 , 133 , 209 , 72 , 52 , 20 , 97 , 7 , 87 , 87 , 128 , 99 , 139 , 84 , 120 , 185 , 20 , 97 , 7 , 119 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 49 , 60 , 229 , 103 , 17 , 97 , 2 , 193 , 87 , 128 , 99 , 70 , 144 , 72 , 64 , 17 , 97 , 2 , 95 , 87 , 128 , 99 , 112 , 160 , 130 , 49 , 17 , 97 , 2 , 46 , 87 , 128 , 99 , 112 , 160 , 130 , 49 , 20 , 97 , 6 , 185 , 87 , 128 , 99 , 121 , 204 , 103 , 144 , 20 , 97 , 6 , 239 , 87 , 128 , 99 , 123 , 46 , 48 , 214 , 20 , 97 , 7 , 15 , 87 , 128 , 99 , 132 , 86 , 203 , 89 , 20 , 97 , 7 , 34 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 70 , 144 , 72 , 64 , 20 , 97 , 6 , 35 , 87 , 128 , 99 , 72 , 8 , 40 , 94 , 20 , 97 , 6 , 97 , 87 , 128 , 99 , 79 , 100 , 178 , 190 , 20 , 97 , 6 , 129 , 87 , 128 , 99 , 92 , 151 , 90 , 187 , 20 , 97 , 6 , 161 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 57 , 244 , 118 , 147 , 17 , 97 , 2 , 155 , 87 , 128 , 99 , 57 , 244 , 118 , 147 , 20 , 97 , 5 , 174 , 87 , 128 , 99 , 63 , 75 , 168 , 58 , 20 , 97 , 5 , 206 , 87 , 128 , 99 , 64 , 193 , 15 , 25 , 20 , 97 , 5 , 227 , 87 , 128 , 99 , 66 , 150 , 108 , 104 , 20 , 97 , 6 , 3 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 49 , 60 , 229 , 103 , 20 , 97 , 5 , 82 , 87 , 128 , 99 , 54 , 86 , 138 , 190 , 20 , 97 , 5 , 110 , 87 , 128 , 99 , 57 , 80 , 147 , 81 , 20 , 97 , 5 , 142 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 28 , 74 , 20 , 54 , 17 , 97 , 3 , 46 , 87 , 128 , 99 , 36 , 138 , 156 , 163 , 17 , 97 , 3 , 8 , 87 , 128 , 99 , 36 , 138 , 156 , 163 , 20 , 97 , 4 , 206 , 87 , 128 , 99 , 38 , 28 , 128 , 182 , 20 , 97 , 4 , 255 , 87 , 128 , 99 , 44 , 166 , 147 , 136 , 20 , 97 , 5 , 31 , 87 , 128 , 99 , 47 , 47 , 241 , 93 , 20 , 97 , 5 , 50 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 28 , 74 , 20 , 54 , 20 , 97 , 4 , 120 , 87 , 128 , 99 , 31 , 145 , 67 , 130 , 20 , 97 , 4 , 152 , 87 , 128 , 99 , 35 , 184 , 114 , 221 , 20 , 97 , 4 , 174 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 9 , 94 , 167 , 179 , 17 , 97 , 3 , 106 , 87 , 128 , 99 , 9 , 94 , 167 , 179 , 20 , 97 , 4 , 10 , 87 , 128 , 99 , 11 , 39 , 251 , 154 , 20 , 97 , 4 , 42 , 87 , 128 , 99 , 21 , 142 , 249 , 62 , 20 , 97 , 4 , 73 , 87 , 128 , 99 , 24 , 22 , 13 , 221 , 20 , 97 , 4 , 99 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 1 , 255 , 201 , 167 , 20 , 97 , 3 , 145 , 87 , 128 , 99 , 6 , 253 , 222 , 3 , 20 , 97 , 3 , 198 , 87 , 128 , 99 , 7 , 24 , 79 , 28 , 20 , 97 , 3 , 232 , 87 , 91 , 96 , 0 , 128 , 253 , 91 , 52 , 128 , 21 , 97 , 3 , 157 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 3 , 172 , 54 , 96 , 4 , 97 , 51 , 170 , 86 , 91 , 97 , 10 , 241 , 86 , 91 , 96 , 64 , 81 , 144 , 21 , 21 , 129 , 82 , 96 , 32 , 1 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 243 , 91 , 52 , 128 , 21 , 97 , 3 , 210 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 219 , 97 , 11 , 28 , 86 , 91 , 96 , 64 , 81 , 97 , 3 , 189 , 145 , 144 , 97 , 51 , 248 , 86 , 91 , 52 , 128 , 21 , 97 , 3 , 244 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 4 , 3 , 54 , 96 , 4 , 97 , 52 , 84 , 86 , 91 , 97 , 11 , 174 , 86 , 91 , 0 , 91 , 52 , 128 , 21 , 97 , 4 , 22 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 4 , 37 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 12 , 227 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 54 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 11 , 84 , 91 , 96 , 64 , 81 , 144 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 85 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 0 , 84 , 97 , 3 , 177 , 144 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 111 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 5 , 84 , 97 , 4 , 59 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 132 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 4 , 147 , 54 , 96 , 4 , 97 , 52 , 84 , 86 , 91 , 97 , 12 , 251 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 164 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 18 , 84 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 186 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 4 , 201 , 54 , 96 , 4 , 97 , 52 , 181 , 86 , 91 , 97 , 14 , 233 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 218 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 4 , 233 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 96 , 0 , 144 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 144 , 82 , 96 , 64 , 144 , 145 , 32 , 1 , 84 , 144 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 11 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 26 , 54 , 96 , 4 , 97 , 52 , 181 , 86 , 91 , 97 , 15 , 13 , 86 , 91 , 97 , 4 , 8 , 97 , 5 , 45 , 54 , 96 , 4 , 97 , 52 , 181 , 86 , 91 , 97 , 16 , 125 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 62 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 77 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 17 , 215 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 94 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 64 , 81 , 96 , 18 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 122 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 137 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 17 , 253 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 154 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 5 , 169 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 18 , 123 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 186 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 201 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 18 , 157 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 218 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 19 , 212 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 239 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 254 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 20 , 104 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 15 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 6 , 30 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 20 , 245 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 47 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 10 , 84 , 97 , 6 , 73 , 144 , 98 , 1 , 0 , 0 , 144 , 4 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 109 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 6 , 124 , 54 , 96 , 4 , 97 , 53 , 63 , 86 , 91 , 97 , 21 , 2 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 141 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 6 , 73 , 97 , 6 , 156 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 22 , 45 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 173 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 8 , 84 , 96 , 255 , 22 , 97 , 3 , 177 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 197 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 6 , 212 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 144 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 251 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 7 , 10 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 22 , 87 , 86 , 91 , 97 , 4 , 8 , 97 , 7 , 29 , 54 , 96 , 4 , 97 , 53 , 158 , 86 , 91 , 97 , 22 , 108 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 46 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 23 , 198 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 67 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 7 , 82 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 24 , 88 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 99 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 6 , 73 , 97 , 7 , 114 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 24 , 122 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 131 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 7 , 146 , 54 , 96 , 4 , 97 , 53 , 255 , 86 , 91 , 97 , 24 , 138 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 163 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 6 , 73 , 97 , 7 , 178 , 54 , 96 , 4 , 97 , 54 , 28 , 86 , 91 , 97 , 24 , 199 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 195 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 7 , 210 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 24 , 230 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 227 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 219 , 97 , 25 , 17 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 248 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 8 , 7 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 25 , 32 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 24 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 10 , 84 , 97 , 8 , 39 , 144 , 97 , 255 , 255 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 97 , 255 , 255 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 70 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 0 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 91 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 106 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 25 , 70 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 123 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 138 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 25 , 193 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 155 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 8 , 164 , 97 , 25 , 207 , 86 , 91 , 96 , 64 , 81 , 97 , 3 , 189 , 145 , 144 , 97 , 54 , 62 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 189 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 204 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 96 , 15 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 237 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 252 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 96 , 16 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 29 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 17 , 84 , 97 , 3 , 177 , 144 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 55 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 9 , 70 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 97 , 26 , 48 , 86 , 91 , 97 , 4 , 8 , 97 , 9 , 89 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 26 , 212 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 106 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 9 , 121 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 97 , 27 , 250 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 138 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 9 , 153 , 54 , 96 , 4 , 97 , 54 , 157 , 86 , 91 , 97 , 28 , 59 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 170 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 12 , 84 , 97 , 6 , 73 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 202 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 9 , 217 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 29 , 67 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 234 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 11 , 84 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 0 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 10 , 84 , 97 , 255 , 255 , 22 , 97 , 8 , 39 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 25 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 59 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 74 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 29 , 90 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 91 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 10 , 106 , 54 , 96 , 4 , 97 , 54 , 185 , 86 , 91 , 97 , 29 , 128 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 123 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 157 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 172 , 54 , 96 , 4 , 97 , 54 , 231 , 86 , 91 , 97 , 29 , 171 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 189 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 204 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 31 , 125 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 221 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 236 , 54 , 96 , 4 , 97 , 52 , 84 , 86 , 91 , 97 , 31 , 172 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 130 , 22 , 99 , 90 , 5 , 24 , 15 , 96 , 224 , 27 , 20 , 128 , 97 , 11 , 22 , 87 , 80 , 97 , 11 , 22 , 130 , 97 , 34 , 26 , 86 , 91 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 96 , 96 , 6 , 128 , 84 , 97 , 11 , 43 , 144 , 97 , 55 , 94 , 86 , 91 , 128 , 96 , 31 , 1 , 96 , 32 , 128 , 145 , 4 , 2 , 96 , 32 , 1 , 96 , 64 , 81 , 144 , 129 , 1 , 96 , 64 , 82 , 128 , 146 , 145 , 144 , 129 , 129 , 82 , 96 , 32 , 1 , 130 , 128 , 84 , 97 , 11 , 87 , 144 , 97 , 55 , 94 , 86 , 91 , 128 , 21 , 97 , 11 , 164 , 87 , 128 , 96 , 31 , 16 , 97 , 11 , 121 , 87 , 97 , 1 , 0 , 128 , 131 , 84 , 4 , 2 , 131 , 82 , 145 , 96 , 32 , 1 , 145 , 97 , 11 , 164 , 86 , 91 , 130 , 1 , 145 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 144 , 91 , 129 , 84 , 129 , 82 , 144 , 96 , 1 , 1 , 144 , 96 , 32 , 1 , 128 , 131 , 17 , 97 , 11 , 135 , 87 , 130 , 144 , 3 , 96 , 31 , 22 , 130 , 1 , 145 , 91 , 80 , 80 , 80 , 80 , 80 , 144 , 80 , 144 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 11 , 225 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 12 , 9 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 12 , 23 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 12 , 54 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 12 , 183 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 58 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 70 , 101 , 101 , 32 , 82 , 101 , 99 , 105 , 112 , 105 , 96 , 68 , 130 , 1 , 82 , 127 , 101 , 110 , 116 , 32 , 99 , 97 , 110 , 110 , 111 , 116 , 32 , 98 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 80 , 80 , 96 , 10 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 146 , 22 , 98 , 1 , 0 , 0 , 2 , 98 , 1 , 0 , 0 , 96 , 1 , 96 , 176 , 27 , 3 , 25 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 96 , 0 , 51 , 97 , 12 , 241 , 129 , 133 , 133 , 97 , 34 , 79 , 86 , 91 , 80 , 96 , 1 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 13 , 37 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 13 , 77 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 13 , 91 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 13 , 122 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 13 , 251 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 43 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 84 , 111 , 107 , 101 , 110 , 32 , 115 , 104 , 111 , 117 , 96 , 68 , 130 , 1 , 82 , 106 , 27 , 25 , 8 , 24 , 153 , 72 , 29 , 152 , 91 , 26 , 89 , 96 , 170 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 128 , 91 , 96 , 13 , 84 , 129 , 16 , 21 , 97 , 14 , 90 , 87 , 132 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 96 , 13 , 130 , 129 , 84 , 129 , 16 , 97 , 14 , 38 , 87 , 97 , 14 , 38 , 97 , 56 , 190 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 144 , 145 , 32 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 3 , 97 , 14 , 72 , 87 , 128 , 145 , 80 , 97 , 14 , 90 , 86 , 91 , 128 , 97 , 14 , 82 , 129 , 97 , 56 , 212 , 86 , 91 , 145 , 80 , 80 , 97 , 13 , 255 , 86 , 91 , 80 , 96 , 13 , 84 , 129 , 16 , 97 , 14 , 186 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 84 , 111 , 107 , 101 , 110 , 32 , 110 , 111 , 116 , 32 , 96 , 68 , 130 , 1 , 82 , 100 , 25 , 155 , 221 , 91 , 153 , 96 , 218 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 97 , 14 , 227 , 129 , 97 , 35 , 115 , 86 , 91 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 51 , 97 , 14 , 247 , 133 , 130 , 133 , 97 , 36 , 35 , 86 , 91 , 97 , 15 , 2 , 133 , 133 , 133 , 97 , 36 , 151 , 86 , 91 , 80 , 96 , 1 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 97 , 15 , 21 , 97 , 38 , 77 , 86 , 91 , 97 , 15 , 45 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 15 , 73 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 237 , 86 , 91 , 129 , 129 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 15 , 160 , 87 , 128 , 71 , 16 , 21 , 97 , 15 , 121 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 48 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 15 , 155 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 121 , 86 , 91 , 97 , 16 , 96 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 129 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 15 , 230 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 16 , 10 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 16 , 21 , 97 , 16 , 40 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 253 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 16 , 96 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 69 , 86 , 91 , 97 , 16 , 108 , 133 , 133 , 133 , 136 , 97 , 38 , 166 , 86 , 91 , 80 , 80 , 97 , 16 , 120 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 80 , 80 , 80 , 86 , 91 , 97 , 16 , 133 , 97 , 38 , 77 , 86 , 91 , 97 , 16 , 157 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 16 , 185 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 237 , 86 , 91 , 96 , 10 , 84 , 130 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 129 , 22 , 144 , 131 , 144 , 131 , 22 , 97 , 17 , 29 , 87 , 128 , 21 , 97 , 16 , 246 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 147 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 17 , 24 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 240 , 86 , 91 , 97 , 17 , 115 , 86 , 91 , 52 , 21 , 97 , 17 , 59 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 89 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 17 , 115 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 166 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 17 , 153 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 233 , 86 , 91 , 97 , 17 , 162 , 129 , 97 , 39 , 14 , 86 , 91 , 97 , 17 , 190 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 59 , 86 , 91 , 97 , 17 , 202 , 134 , 134 , 134 , 137 , 97 , 39 , 135 , 86 , 91 , 80 , 80 , 80 , 97 , 16 , 120 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 144 , 82 , 96 , 64 , 144 , 145 , 32 , 1 , 84 , 97 , 17 , 243 , 129 , 97 , 40 , 114 , 86 , 91 , 97 , 16 , 120 , 131 , 131 , 97 , 40 , 124 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 51 , 20 , 97 , 18 , 109 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 47 , 96 , 36 , 130 , 1 , 82 , 127 , 65 , 99 , 99 , 101 , 115 , 115 , 67 , 111 , 110 , 116 , 114 , 111 , 108 , 58 , 32 , 99 , 97 , 110 , 32 , 111 , 110 , 108 , 121 , 32 , 114 , 101 , 110 , 111 , 117 , 110 , 99 , 101 , 96 , 68 , 130 , 1 , 82 , 110 , 16 , 57 , 55 , 182 , 50 , 185 , 144 , 51 , 55 , 185 , 16 , 57 , 178 , 182 , 51 , 96 , 137 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 40 , 158 , 86 , 91 , 80 , 80 , 86 , 91 , 96 , 0 , 51 , 97 , 12 , 241 , 129 , 133 , 133 , 97 , 18 , 142 , 131 , 131 , 97 , 29 , 128 , 86 , 91 , 97 , 18 , 152 , 145 , 144 , 97 , 56 , 78 , 86 , 91 , 97 , 34 , 79 , 86 , 91 , 97 , 18 , 165 , 97 , 38 , 77 , 86 , 91 , 129 , 129 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 18 , 252 , 87 , 128 , 71 , 16 , 21 , 97 , 18 , 213 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 48 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 18 , 247 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 121 , 86 , 91 , 97 , 19 , 188 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 129 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 19 , 66 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 19 , 102 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 16 , 21 , 97 , 19 , 132 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 253 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 19 , 188 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 69 , 86 , 91 , 97 , 19 , 200 , 51 , 133 , 133 , 51 , 97 , 38 , 166 , 86 , 91 , 80 , 80 , 97 , 18 , 119 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 97 , 19 , 236 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 20 , 94 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 57 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 114 , 101 , 115 , 101 , 116 , 77 , 105 , 110 , 116 , 101 , 114 , 80 , 97 , 117 , 115 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 96 , 68 , 130 , 1 , 82 , 127 , 118 , 101 , 32 , 112 , 97 , 117 , 115 , 101 , 114 , 32 , 114 , 111 , 108 , 101 , 32 , 116 , 111 , 32 , 117 , 110 , 112 , 97 , 117 , 115 , 101 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 20 , 102 , 97 , 40 , 192 , 86 , 91 , 86 , 91 , 97 , 20 , 128 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 20 , 235 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 54 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 114 , 101 , 115 , 101 , 116 , 77 , 105 , 110 , 116 , 101 , 114 , 80 , 97 , 117 , 115 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 96 , 68 , 130 , 1 , 82 , 117 , 29 , 153 , 72 , 27 , 90 , 91 , 157 , 25 , 92 , 136 , 28 , 155 , 219 , 25 , 72 , 29 , 27 , 200 , 27 , 90 , 91 , 157 , 96 , 82 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 41 , 18 , 86 , 91 , 97 , 20 , 255 , 51 , 130 , 97 , 41 , 223 , 86 , 91 , 80 , 86 , 91 , 97 , 21 , 10 , 97 , 38 , 77 , 86 , 91 , 130 , 130 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 21 , 97 , 87 , 128 , 71 , 16 , 21 , 97 , 21 , 58 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 48 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 21 , 92 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 121 , 86 , 91 , 97 , 22 , 33 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 129 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 21 , 167 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 21 , 203 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 16 , 21 , 97 , 21 , 233 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 253 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 22 , 33 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 69 , 86 , 91 , 97 , 16 , 108 , 51 , 134 , 134 , 134 , 97 , 38 , 166 , 86 , 91 , 96 , 13 , 129 , 129 , 84 , 129 , 16 , 97 , 22 , 61 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 144 , 145 , 32 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 144 , 80 , 129 , 86 , 91 , 97 , 22 , 98 , 130 , 51 , 131 , 97 , 36 , 35 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 41 , 223 , 86 , 91 , 97 , 22 , 116 , 97 , 38 , 77 , 86 , 91 , 97 , 22 , 140 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 22 , 168 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 237 , 86 , 91 , 96 , 10 , 84 , 131 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 129 , 22 , 144 , 132 , 144 , 131 , 22 , 97 , 23 , 12 , 87 , 128 , 21 , 97 , 22 , 229 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 147 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 23 , 7 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 240 , 86 , 91 , 97 , 23 , 98 , 86 , 91 , 52 , 21 , 97 , 23 , 42 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 89 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 23 , 98 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 166 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 23 , 136 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 233 , 86 , 91 , 97 , 23 , 145 , 129 , 97 , 39 , 14 , 86 , 91 , 97 , 23 , 173 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 59 , 86 , 91 , 97 , 23 , 185 , 135 , 135 , 135 , 135 , 97 , 39 , 135 , 86 , 91 , 80 , 80 , 80 , 97 , 14 , 227 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 97 , 23 , 222 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 24 , 80 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 55 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 114 , 101 , 115 , 101 , 116 , 77 , 105 , 110 , 116 , 101 , 114 , 80 , 97 , 117 , 115 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 96 , 68 , 130 , 1 , 82 , 127 , 118 , 101 , 32 , 112 , 97 , 117 , 115 , 101 , 114 , 32 , 114 , 111 , 108 , 101 , 32 , 116 , 111 , 32 , 112 , 97 , 117 , 115 , 101 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 20 , 102 , 97 , 43 , 31 , 86 , 91 , 96 , 10 , 84 , 96 , 0 , 144 , 97 , 39 , 16 , 144 , 97 , 24 , 112 , 144 , 97 , 255 , 255 , 22 , 132 , 97 , 60 , 125 , 86 , 91 , 97 , 11 , 22 , 145 , 144 , 97 , 60 , 148 , 86 , 91 , 96 , 14 , 129 , 129 , 84 , 129 , 16 , 97 , 22 , 61 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 24 , 180 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 17 , 128 , 84 , 96 , 255 , 25 , 22 , 145 , 21 , 21 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 97 , 24 , 223 , 144 , 131 , 97 , 43 , 92 , 86 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 1 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 147 , 144 , 147 , 22 , 132 , 82 , 145 , 144 , 82 , 144 , 32 , 84 , 96 , 255 , 22 , 144 , 86 , 91 , 96 , 96 , 96 , 7 , 128 , 84 , 97 , 11 , 43 , 144 , 97 , 55 , 94 , 86 , 91 , 96 , 10 , 84 , 96 , 0 , 144 , 97 , 25 , 54 , 144 , 97 , 255 , 255 , 22 , 97 , 39 , 16 , 97 , 60 , 182 , 86 , 91 , 97 , 255 , 255 , 22 , 97 , 24 , 112 , 131 , 97 , 39 , 16 , 97 , 60 , 125 , 86 , 91 , 96 , 0 , 51 , 129 , 97 , 25 , 84 , 130 , 134 , 97 , 29 , 128 , 86 , 91 , 144 , 80 , 131 , 129 , 16 , 21 , 97 , 25 , 180 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 100 , 101 , 99 , 114 , 101 , 97 , 115 , 101 , 100 , 32 , 97 , 108 , 108 , 111 , 119 , 97 , 110 , 99 , 101 , 32 , 98 , 101 , 108 , 111 , 119 , 96 , 68 , 130 , 1 , 82 , 100 , 32 , 122 , 101 , 114 , 111 , 96 , 216 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 15 , 2 , 130 , 134 , 134 , 132 , 3 , 97 , 34 , 79 , 86 , 91 , 96 , 0 , 51 , 97 , 12 , 241 , 129 , 133 , 133 , 97 , 36 , 151 , 86 , 91 , 96 , 96 , 96 , 13 , 128 , 84 , 128 , 96 , 32 , 2 , 96 , 32 , 1 , 96 , 64 , 81 , 144 , 129 , 1 , 96 , 64 , 82 , 128 , 146 , 145 , 144 , 129 , 129 , 82 , 96 , 32 , 1 , 130 , 128 , 84 , 128 , 21 , 97 , 11 , 164 , 87 , 96 , 32 , 2 , 130 , 1 , 145 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 144 , 91 , 129 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 82 , 96 , 1 , 144 , 145 , 1 , 144 , 96 , 32 , 1 , 128 , 131 , 17 , 97 , 26 , 9 , 87 , 80 , 80 , 80 , 80 , 80 , 144 , 80 , 144 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 26 , 90 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 97 , 26 , 128 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 209 , 86 , 91 , 96 , 12 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 96 , 64 , 81 , 144 , 129 , 82 , 127 , 254 , 20 , 152 , 67 , 164 , 64 , 75 , 67 , 105 , 157 , 68 , 108 , 153 , 201 , 190 , 45 , 122 , 91 , 252 , 139 , 214 , 110 , 21 , 202 , 76 , 250 , 213 , 202 , 40 , 17 , 221 , 155 , 144 , 96 , 32 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 161 , 80 , 86 , 91 , 97 , 26 , 220 , 97 , 38 , 77 , 86 , 91 , 96 , 10 , 84 , 130 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 129 , 22 , 144 , 131 , 144 , 131 , 22 , 97 , 27 , 64 , 87 , 128 , 21 , 97 , 27 , 25 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 147 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 27 , 59 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 240 , 86 , 91 , 97 , 27 , 150 , 86 , 91 , 52 , 21 , 97 , 27 , 94 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 89 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 27 , 150 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 166 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 27 , 188 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 233 , 86 , 91 , 97 , 27 , 197 , 129 , 97 , 39 , 14 , 86 , 91 , 97 , 27 , 225 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 59 , 86 , 91 , 97 , 27 , 237 , 51 , 134 , 134 , 51 , 97 , 39 , 135 , 86 , 91 , 80 , 80 , 80 , 97 , 18 , 119 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 28 , 21 , 87 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 11 , 22 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 11 , 22 , 86 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 28 , 101 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 28 , 141 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 28 , 155 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 28 , 186 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 97 , 39 , 16 , 97 , 255 , 255 , 132 , 22 , 16 , 97 , 29 , 41 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 44 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 102 , 101 , 96 , 68 , 130 , 1 , 82 , 107 , 101 , 32 , 112 , 101 , 114 , 99 , 101 , 110 , 116 , 97 , 103 , 101 , 96 , 160 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 80 , 80 , 96 , 10 , 128 , 84 , 97 , 255 , 255 , 25 , 22 , 97 , 255 , 255 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 97 , 11 , 22 , 144 , 97 , 43 , 104 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 144 , 82 , 96 , 64 , 144 , 145 , 32 , 1 , 84 , 97 , 29 , 118 , 129 , 97 , 40 , 114 , 86 , 91 , 97 , 16 , 120 , 131 , 131 , 97 , 40 , 158 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 145 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 4 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 147 , 144 , 148 , 22 , 130 , 82 , 145 , 144 , 145 , 82 , 32 , 84 , 144 , 86 , 91 , 96 , 0 , 84 , 96 , 255 , 22 , 21 , 97 , 29 , 254 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 129 , 144 , 82 , 96 , 36 , 130 , 1 , 82 , 127 , 73 , 110 , 105 , 116 , 105 , 97 , 108 , 105 , 122 , 101 , 100 , 58 , 32 , 65 , 108 , 114 , 101 , 97 , 100 , 121 , 32 , 105 , 110 , 105 , 116 , 105 , 97 , 108 , 105 , 122 , 101 , 100 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 97 , 30 , 115 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 54 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 70 , 101 , 101 , 32 , 82 , 101 , 99 , 105 , 112 , 105 , 96 , 68 , 130 , 1 , 82 , 117 , 6 , 86 , 231 , 66 , 4 , 22 , 70 , 71 , 38 , 87 , 55 , 50 , 6 , 54 , 22 , 226 , 119 , 66 , 6 , 38 , 82 , 3 , 96 , 84 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 30 , 153 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 209 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 97 , 31 , 6 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 46 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 65 , 100 , 109 , 105 , 110 , 32 , 65 , 100 , 100 , 114 , 96 , 68 , 130 , 1 , 82 , 109 , 6 , 87 , 55 , 50 , 6 , 54 , 22 , 226 , 119 , 66 , 6 , 38 , 82 , 3 , 96 , 148 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 31 , 15 , 129 , 97 , 43 , 114 , 86 , 91 , 80 , 96 , 0 , 128 , 84 , 96 , 1 , 96 , 255 , 25 , 145 , 130 , 22 , 23 , 144 , 145 , 85 , 96 , 10 , 128 , 84 , 97 , 255 , 255 , 151 , 144 , 151 , 22 , 96 , 1 , 96 , 1 , 96 , 176 , 27 , 3 , 25 , 144 , 151 , 22 , 150 , 144 , 150 , 23 , 98 , 1 , 0 , 0 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 150 , 135 , 22 , 2 , 23 , 144 , 149 , 85 , 96 , 12 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 147 , 144 , 148 , 22 , 146 , 144 , 146 , 23 , 144 , 146 , 85 , 96 , 18 , 145 , 144 , 145 , 85 , 96 , 17 , 128 , 84 , 144 , 146 , 22 , 144 , 21 , 21 , 23 , 144 , 85 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 31 , 167 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 18 , 85 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 31 , 214 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 31 , 254 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 32 , 12 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 32 , 43 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 21 , 97 , 32 , 177 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 47 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 84 , 111 , 107 , 101 , 110 , 32 , 115 , 104 , 111 , 117 , 96 , 68 , 130 , 1 , 82 , 110 , 27 , 25 , 8 , 27 , 155 , 221 , 8 , 24 , 153 , 72 , 29 , 152 , 91 , 26 , 89 , 96 , 138 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 13 , 128 , 84 , 96 , 1 , 129 , 1 , 144 , 145 , 85 , 127 , 215 , 182 , 153 , 1 , 5 , 113 , 145 , 1 , 218 , 190 , 183 , 113 , 68 , 242 , 163 , 56 , 92 , 128 , 51 , 172 , 211 , 175 , 151 , 233 , 66 , 58 , 105 , 94 , 129 , 173 , 30 , 181 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 33 , 116 , 87 , 96 , 14 , 128 , 84 , 96 , 1 , 128 , 130 , 1 , 144 , 146 , 85 , 127 , 187 , 123 , 74 , 69 , 77 , 195 , 73 , 57 , 35 , 72 , 47 , 7 , 130 , 35 , 41 , 237 , 25 , 232 , 36 , 78 , 255 , 88 , 44 , 194 , 4 , 248 , 85 , 76 , 54 , 32 , 195 , 253 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 145 , 23 , 144 , 85 , 91 , 80 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 96 , 1 , 23 , 144 , 85 , 86 , 91 , 97 , 33 , 164 , 130 , 130 , 97 , 24 , 230 , 86 , 91 , 97 , 18 , 119 , 87 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 128 , 134 , 82 , 146 , 82 , 128 , 132 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 147 , 23 , 144 , 146 , 85 , 144 , 81 , 51 , 146 , 133 , 145 , 127 , 47 , 135 , 136 , 17 , 126 , 126 , 255 , 29 , 130 , 233 , 38 , 236 , 121 , 73 , 1 , 209 , 124 , 120 , 2 , 74 , 80 , 39 , 9 , 64 , 48 , 69 , 64 , 167 , 51 , 101 , 111 , 13 , 145 , 144 , 164 , 80 , 80 , 86 , 91 , 96 , 0 , 97 , 24 , 223 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 43 , 173 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 130 , 22 , 99 , 121 , 101 , 219 , 11 , 96 , 224 , 27 , 20 , 128 , 97 , 11 , 22 , 87 , 80 , 99 , 1 , 255 , 201 , 167 , 96 , 224 , 27 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 131 , 22 , 20 , 97 , 11 , 22 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 34 , 177 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 36 , 128 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 97 , 112 , 112 , 114 , 111 , 118 , 101 , 32 , 102 , 114 , 111 , 109 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 96 , 68 , 130 , 1 , 82 , 99 , 114 , 101 , 115 , 115 , 96 , 224 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 35 , 18 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 34 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 97 , 112 , 112 , 114 , 111 , 118 , 101 , 32 , 116 , 111 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 96 , 68 , 130 , 1 , 82 , 97 , 115 , 115 , 96 , 240 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 129 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 4 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 148 , 135 , 22 , 128 , 132 , 82 , 148 , 130 , 82 , 145 , 130 , 144 , 32 , 133 , 144 , 85 , 144 , 81 , 132 , 129 , 82 , 127 , 140 , 91 , 225 , 229 , 235 , 236 , 125 , 91 , 209 , 79 , 113 , 66 , 125 , 30 , 132 , 243 , 221 , 3 , 20 , 192 , 247 , 178 , 41 , 30 , 91 , 32 , 10 , 200 , 199 , 195 , 185 , 37 , 145 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 80 , 80 , 80 , 86 , 91 , 96 , 13 , 128 , 84 , 97 , 35 , 131 , 144 , 96 , 1 , 144 , 97 , 61 , 33 , 86 , 91 , 129 , 84 , 129 , 16 , 97 , 35 , 147 , 87 , 97 , 35 , 147 , 97 , 56 , 190 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 144 , 145 , 32 , 1 , 84 , 96 , 13 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 146 , 22 , 145 , 131 , 144 , 129 , 16 , 97 , 35 , 191 , 87 , 97 , 35 , 191 , 97 , 56 , 190 , 86 , 91 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 96 , 0 , 97 , 1 , 0 , 10 , 129 , 84 , 129 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 2 , 25 , 22 , 144 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 2 , 23 , 144 , 85 , 80 , 96 , 13 , 128 , 84 , 128 , 97 , 35 , 254 , 87 , 97 , 35 , 254 , 97 , 61 , 52 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 32 , 144 , 32 , 129 , 1 , 96 , 0 , 25 , 144 , 129 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 144 , 85 , 1 , 144 , 85 , 80 , 86 , 91 , 96 , 0 , 97 , 36 , 47 , 132 , 132 , 97 , 29 , 128 , 86 , 91 , 144 , 80 , 96 , 0 , 25 , 129 , 20 , 97 , 14 , 227 , 87 , 129 , 129 , 16 , 21 , 97 , 36 , 138 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 29 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 105 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 97 , 108 , 108 , 111 , 119 , 97 , 110 , 99 , 101 , 0 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 14 , 227 , 132 , 132 , 132 , 132 , 3 , 97 , 34 , 79 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 36 , 251 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 102 , 114 , 111 , 109 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 96 , 68 , 130 , 1 , 82 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 216 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 37 , 93 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 35 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 116 , 111 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 96 , 68 , 130 , 1 , 82 , 98 , 101 , 115 , 115 , 96 , 232 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 37 , 104 , 131 , 131 , 131 , 97 , 43 , 252 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 129 , 129 , 16 , 21 , 97 , 37 , 224 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 38 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 97 , 109 , 111 , 117 , 110 , 116 , 32 , 101 , 120 , 99 , 101 , 101 , 100 , 115 , 32 , 98 , 96 , 68 , 130 , 1 , 82 , 101 , 97 , 108 , 97 , 110 , 99 , 101 , 96 , 208 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 128 , 133 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 128 , 130 , 32 , 134 , 134 , 3 , 144 , 85 , 146 , 134 , 22 , 128 , 130 , 82 , 144 , 131 , 144 , 32 , 128 , 84 , 134 , 1 , 144 , 85 , 145 , 81 , 127 , 221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239 , 144 , 97 , 38 , 64 , 144 , 134 , 129 , 82 , 96 , 32 , 1 , 144 , 86 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 97 , 14 , 227 , 86 , 91 , 96 , 2 , 96 , 9 , 84 , 3 , 97 , 38 , 159 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 31 , 96 , 36 , 130 , 1 , 82 , 127 , 82 , 101 , 101 , 110 , 116 , 114 , 97 , 110 , 99 , 121 , 71 , 117 , 97 , 114 , 100 , 58 , 32 , 114 , 101 , 101 , 110 , 116 , 114 , 97 , 110 , 116 , 32 , 99 , 97 , 108 , 108 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 2 , 96 , 9 , 85 , 86 , 91 , 97 , 38 , 176 , 132 , 131 , 97 , 41 , 223 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 38 , 250 , 87 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 144 , 131 , 21 , 97 , 8 , 252 , 2 , 144 , 132 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 38 , 244 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 97 , 14 , 227 , 86 , 91 , 97 , 14 , 227 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 130 , 132 , 97 , 44 , 7 , 86 , 91 , 96 , 0 , 96 , 18 , 84 , 48 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 99 , 24 , 22 , 13 , 221 , 96 , 64 , 81 , 129 , 99 , 255 , 255 , 255 , 255 , 22 , 96 , 224 , 27 , 129 , 82 , 96 , 4 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 39 , 81 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 39 , 117 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 97 , 39 , 127 , 144 , 132 , 97 , 56 , 78 , 86 , 91 , 17 , 21 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 0 , 97 , 39 , 167 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 21 , 97 , 39 , 161 , 87 , 131 , 97 , 24 , 88 , 86 , 91 , 52 , 97 , 24 , 88 , 86 , 91 , 144 , 80 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 21 , 97 , 39 , 201 , 87 , 97 , 39 , 196 , 130 , 133 , 97 , 61 , 33 , 86 , 91 , 97 , 39 , 211 , 86 , 91 , 97 , 39 , 211 , 130 , 52 , 97 , 61 , 33 , 86 , 91 , 144 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 97 , 40 , 40 , 87 , 96 , 10 , 84 , 96 , 64 , 81 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 144 , 97 , 8 , 252 , 132 , 21 , 2 , 144 , 132 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 40 , 34 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 97 , 40 , 96 , 86 , 91 , 97 , 40 , 61 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 135 , 48 , 132 , 97 , 44 , 106 , 86 , 91 , 96 , 10 , 84 , 97 , 40 , 96 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 135 , 129 , 22 , 145 , 137 , 145 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 22 , 133 , 97 , 44 , 106 , 86 , 91 , 97 , 40 , 106 , 131 , 130 , 97 , 41 , 18 , 86 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 97 , 20 , 255 , 129 , 51 , 97 , 44 , 162 , 86 , 91 , 97 , 40 , 134 , 130 , 130 , 97 , 33 , 154 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 97 , 16 , 120 , 144 , 130 , 97 , 34 , 5 , 86 , 91 , 97 , 40 , 168 , 130 , 130 , 97 , 44 , 251 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 97 , 16 , 120 , 144 , 130 , 97 , 45 , 98 , 86 , 91 , 97 , 40 , 200 , 97 , 45 , 119 , 86 , 91 , 96 , 8 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 127 , 93 , 185 , 238 , 10 , 73 , 91 , 242 , 230 , 255 , 156 , 145 , 167 , 131 , 76 , 27 , 164 , 253 , 210 , 68 , 165 , 232 , 170 , 78 , 83 , 123 , 211 , 138 , 234 , 228 , 176 , 115 , 170 , 51 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 161 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 41 , 104 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 31 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 109 , 105 , 110 , 116 , 32 , 116 , 111 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 41 , 116 , 96 , 0 , 131 , 131 , 97 , 43 , 252 , 86 , 91 , 128 , 96 , 5 , 96 , 0 , 130 , 130 , 84 , 97 , 41 , 134 , 145 , 144 , 97 , 56 , 78 , 86 , 91 , 144 , 145 , 85 , 80 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 3 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 128 , 84 , 134 , 1 , 144 , 85 , 81 , 132 , 129 , 82 , 127 , 221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239 , 145 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 80 , 80 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 42 , 63 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 33 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 98 , 117 , 114 , 110 , 32 , 102 , 114 , 111 , 109 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 96 , 68 , 130 , 1 , 82 , 96 , 115 , 96 , 248 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 42 , 75 , 130 , 96 , 0 , 131 , 97 , 43 , 252 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 129 , 129 , 16 , 21 , 97 , 42 , 191 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 34 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 98 , 117 , 114 , 110 , 32 , 97 , 109 , 111 , 117 , 110 , 116 , 32 , 101 , 120 , 99 , 101 , 101 , 100 , 115 , 32 , 98 , 97 , 108 , 97 , 110 , 96 , 68 , 130 , 1 , 82 , 97 , 99 , 101 , 96 , 240 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 3 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 134 , 134 , 3 , 144 , 85 , 96 , 5 , 128 , 84 , 135 , 144 , 3 , 144 , 85 , 81 , 133 , 129 , 82 , 145 , 146 , 145 , 127 , 221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239 , 145 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 80 , 80 , 80 , 86 , 91 , 97 , 43 , 39 , 97 , 45 , 192 , 86 , 91 , 96 , 8 , 128 , 84 , 96 , 255 , 25 , 22 , 96 , 1 , 23 , 144 , 85 , 127 , 98 , 231 , 140 , 234 , 1 , 190 , 227 , 32 , 205 , 78 , 66 , 2 , 112 , 181 , 234 , 116 , 0 , 13 , 17 , 176 , 201 , 247 , 71 , 84 , 235 , 219 , 252 , 84 , 75 , 5 , 162 , 88 , 97 , 40 , 245 , 51 , 144 , 86 , 91 , 96 , 0 , 97 , 24 , 223 , 131 , 131 , 97 , 46 , 6 , 86 , 91 , 96 , 0 , 97 , 11 , 22 , 130 , 84 , 144 , 86 , 91 , 97 , 43 , 138 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 130 , 97 , 46 , 48 , 86 , 91 , 97 , 43 , 149 , 96 , 0 , 130 , 97 , 46 , 48 , 86 , 91 , 97 , 20 , 255 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 130 , 97 , 46 , 48 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 1 , 131 , 1 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 84 , 97 , 43 , 244 , 87 , 80 , 129 , 84 , 96 , 1 , 129 , 129 , 1 , 132 , 85 , 96 , 0 , 132 , 129 , 82 , 96 , 32 , 128 , 130 , 32 , 144 , 147 , 1 , 132 , 144 , 85 , 132 , 84 , 132 , 130 , 82 , 130 , 134 , 1 , 144 , 147 , 82 , 96 , 64 , 144 , 32 , 145 , 144 , 145 , 85 , 97 , 11 , 22 , 86 , 91 , 80 , 96 , 0 , 97 , 11 , 22 , 86 , 91 , 97 , 16 , 120 , 131 , 131 , 131 , 97 , 46 , 58 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 36 , 130 , 1 , 82 , 96 , 68 , 129 , 1 , 130 , 144 , 82 , 97 , 16 , 120 , 144 , 132 , 144 , 99 , 169 , 5 , 156 , 187 , 96 , 224 , 27 , 144 , 96 , 100 , 1 , 91 , 96 , 64 , 128 , 81 , 96 , 31 , 25 , 129 , 132 , 3 , 1 , 129 , 82 , 145 , 144 , 82 , 96 , 32 , 129 , 1 , 128 , 81 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 22 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 144 , 147 , 22 , 146 , 144 , 146 , 23 , 144 , 145 , 82 , 97 , 46 , 160 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 128 , 133 , 22 , 96 , 36 , 131 , 1 , 82 , 131 , 22 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 129 , 1 , 130 , 144 , 82 , 97 , 14 , 227 , 144 , 133 , 144 , 99 , 35 , 184 , 114 , 221 , 96 , 224 , 27 , 144 , 96 , 132 , 1 , 97 , 44 , 51 , 86 , 91 , 97 , 44 , 172 , 130 , 130 , 97 , 24 , 230 , 86 , 91 , 97 , 18 , 119 , 87 , 97 , 44 , 185 , 129 , 97 , 47 , 114 , 86 , 91 , 97 , 44 , 196 , 131 , 96 , 32 , 97 , 47 , 132 , 86 , 91 , 96 , 64 , 81 , 96 , 32 , 1 , 97 , 44 , 213 , 146 , 145 , 144 , 97 , 61 , 74 , 86 , 91 , 96 , 64 , 128 , 81 , 96 , 31 , 25 , 129 , 132 , 3 , 1 , 129 , 82 , 144 , 130 , 144 , 82 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 130 , 82 , 97 , 11 , 216 , 145 , 96 , 4 , 1 , 97 , 51 , 248 , 86 , 91 , 97 , 45 , 5 , 130 , 130 , 97 , 24 , 230 , 86 , 91 , 21 , 97 , 18 , 119 , 87 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 128 , 133 , 82 , 146 , 82 , 128 , 131 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 81 , 51 , 146 , 133 , 145 , 127 , 246 , 57 , 31 , 92 , 50 , 217 , 198 , 157 , 42 , 71 , 234 , 103 , 11 , 68 , 41 , 116 , 181 , 57 , 53 , 209 , 237 , 199 , 253 , 100 , 235 , 33 , 224 , 71 , 168 , 57 , 23 , 27 , 145 , 144 , 164 , 80 , 80 , 86 , 91 , 96 , 0 , 97 , 24 , 223 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 49 , 32 , 86 , 91 , 96 , 8 , 84 , 96 , 255 , 22 , 97 , 20 , 102 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 20 , 96 , 36 , 130 , 1 , 82 , 115 , 20 , 24 , 93 , 92 , 216 , 88 , 155 , 25 , 78 , 136 , 27 , 155 , 221 , 8 , 28 , 24 , 93 , 92 , 217 , 89 , 96 , 98 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 8 , 84 , 96 , 255 , 22 , 21 , 97 , 20 , 102 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 16 , 96 , 36 , 130 , 1 , 82 , 111 , 20 , 24 , 93 , 92 , 216 , 88 , 155 , 25 , 78 , 136 , 28 , 24 , 93 , 92 , 217 , 89 , 96 , 130 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 130 , 96 , 0 , 1 , 130 , 129 , 84 , 129 , 16 , 97 , 46 , 29 , 87 , 97 , 46 , 29 , 97 , 56 , 190 , 86 , 91 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 84 , 144 , 80 , 146 , 145 , 80 , 80 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 40 , 124 , 86 , 91 , 96 , 8 , 84 , 96 , 255 , 22 , 21 , 97 , 16 , 120 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 42 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 97 , 117 , 115 , 97 , 98 , 108 , 101 , 58 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 119 , 104 , 96 , 68 , 130 , 1 , 82 , 105 , 26 , 91 , 25 , 72 , 28 , 24 , 93 , 92 , 217 , 89 , 96 , 178 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 97 , 46 , 245 , 130 , 96 , 64 , 81 , 128 , 96 , 64 , 1 , 96 , 64 , 82 , 128 , 96 , 32 , 129 , 82 , 96 , 32 , 1 , 127 , 83 , 97 , 102 , 101 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 108 , 111 , 119 , 45 , 108 , 101 , 118 , 101 , 108 , 32 , 99 , 97 , 108 , 108 , 32 , 102 , 97 , 105 , 108 , 101 , 100 , 129 , 82 , 80 , 133 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 97 , 50 , 26 , 144 , 146 , 145 , 144 , 99 , 255 , 255 , 255 , 255 , 22 , 86 , 91 , 128 , 81 , 144 , 145 , 80 , 21 , 97 , 16 , 120 , 87 , 128 , 128 , 96 , 32 , 1 , 144 , 81 , 129 , 1 , 144 , 97 , 47 , 19 , 145 , 144 , 97 , 61 , 191 , 86 , 91 , 97 , 16 , 120 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 42 , 96 , 36 , 130 , 1 , 82 , 127 , 83 , 97 , 102 , 101 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 69 , 82 , 67 , 50 , 48 , 32 , 111 , 112 , 101 , 114 , 97 , 116 , 105 , 111 , 110 , 32 , 100 , 105 , 100 , 32 , 110 , 96 , 68 , 130 , 1 , 82 , 105 , 27 , 221 , 8 , 28 , 221 , 88 , 216 , 217 , 89 , 89 , 96 , 178 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 96 , 97 , 11 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 20 , 91 , 96 , 96 , 96 , 0 , 97 , 47 , 147 , 131 , 96 , 2 , 97 , 60 , 125 , 86 , 91 , 97 , 47 , 158 , 144 , 96 , 2 , 97 , 56 , 78 , 86 , 91 , 103 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 129 , 17 , 21 , 97 , 47 , 182 , 87 , 97 , 47 , 182 , 97 , 61 , 220 , 86 , 91 , 96 , 64 , 81 , 144 , 128 , 130 , 82 , 128 , 96 , 31 , 1 , 96 , 31 , 25 , 22 , 96 , 32 , 1 , 130 , 1 , 96 , 64 , 82 , 128 , 21 , 97 , 47 , 224 , 87 , 96 , 32 , 130 , 1 , 129 , 128 , 54 , 131 , 55 , 1 , 144 , 80 , 91 , 80 , 144 , 80 , 96 , 3 , 96 , 252 , 27 , 129 , 96 , 0 , 129 , 81 , 129 , 16 , 97 , 47 , 251 , 87 , 97 , 47 , 251 , 97 , 56 , 190 , 86 , 91 , 96 , 32 , 1 , 1 , 144 , 96 , 1 , 96 , 1 , 96 , 248 , 27 , 3 , 25 , 22 , 144 , 129 , 96 , 0 , 26 , 144 , 83 , 80 , 96 , 15 , 96 , 251 , 27 , 129 , 96 , 1 , 129 , 81 , 129 , 16 , 97 , 48 , 42 , 87 , 97 , 48 , 42 , 97 , 56 , 190 , 86 , 91 , 96 , 32 , 1 , 1 , 144 , 96 , 1 , 96 , 1 , 96 , 248 , 27 , 3 , 25 , 22 , 144 , 129 , 96 , 0 , 26 , 144 , 83 , 80 , 96 , 0 , 97 , 48 , 78 , 132 , 96 , 2 , 97 , 60 , 125 , 86 , 91 , 97 , 48 , 89 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 144 , 80 , 91 , 96 , 1 , 129 , 17 , 21 , 97 , 48 , 209 , 87 , 111 , 24 , 24 , 153 , 25 , 154 , 26 , 155 , 27 , 156 , 28 , 176 , 177 , 49 , 178 , 50 , 179 , 96 , 129 , 27 , 133 , 96 , 15 , 22 , 96 , 16 , 129 , 16 , 97 , 48 , 141 , 87 , 97 , 48 , 141 , 97 , 56 , 190 , 86 , 91 , 26 , 96 , 248 , 27 , 130 , 130 , 129 , 81 , 129 , 16 , 97 , 48 , 163 , 87 , 97 , 48 , 163 , 97 , 56 , 190 , 86 , 91 , 96 , 32 , 1 , 1 , 144 , 96 , 1 , 96 , 1 , 96 , 248 , 27 , 3 , 25 , 22 , 144 , 129 , 96 , 0 , 26 , 144 , 83 , 80 , 96 , 4 , 148 , 144 , 148 , 28 , 147 , 97 , 48 , 202 , 129 , 97 , 61 , 242 , 86 , 91 , 144 , 80 , 97 , 48 , 92 , 86 , 91 , 80 , 131 , 21 , 97 , 24 , 223 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 129 , 144 , 82 , 96 , 36 , 130 , 1 , 82 , 127 , 83 , 116 , 114 , 105 , 110 , 103 , 115 , 58 , 32 , 104 , 101 , 120 , 32 , 108 , 101 , 110 , 103 , 116 , 104 , 32 , 105 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 1 , 131 , 1 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 84 , 128 , 21 , 97 , 50 , 9 , 87 , 96 , 0 , 97 , 49 , 68 , 96 , 1 , 131 , 97 , 61 , 33 , 86 , 91 , 133 , 84 , 144 , 145 , 80 , 96 , 0 , 144 , 97 , 49 , 88 , 144 , 96 , 1 , 144 , 97 , 61 , 33 , 86 , 91 , 144 , 80 , 129 , 129 , 20 , 97 , 49 , 189 , 87 , 96 , 0 , 134 , 96 , 0 , 1 , 130 , 129 , 84 , 129 , 16 , 97 , 49 , 120 , 87 , 97 , 49 , 120 , 97 , 56 , 190 , 86 , 91 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 84 , 144 , 80 , 128 , 135 , 96 , 0 , 1 , 132 , 129 , 84 , 129 , 16 , 97 , 49 , 155 , 87 , 97 , 49 , 155 , 97 , 56 , 190 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 128 , 131 , 32 , 144 , 145 , 1 , 146 , 144 , 146 , 85 , 145 , 130 , 82 , 96 , 1 , 136 , 1 , 144 , 82 , 96 , 64 , 144 , 32 , 131 , 144 , 85 , 91 , 133 , 84 , 134 , 144 , 128 , 97 , 49 , 206 , 87 , 97 , 49 , 206 , 97 , 61 , 52 , 86 , 91 , 96 , 1 , 144 , 3 , 129 , 129 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 96 , 0 , 144 , 85 , 144 , 85 , 133 , 96 , 1 , 1 , 96 , 0 , 134 , 129 , 82 , 96 , 32 , 1 , 144 , 129 , 82 , 96 , 32 , 1 , 96 , 0 , 32 , 96 , 0 , 144 , 85 , 96 , 1 , 147 , 80 , 80 , 80 , 80 , 97 , 11 , 22 , 86 , 91 , 96 , 0 , 145 , 80 , 80 , 97 , 11 , 22 , 86 , 91 , 80 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 96 , 97 , 50 , 41 , 132 , 132 , 96 , 0 , 133 , 97 , 50 , 49 , 86 , 91 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 96 , 130 , 71 , 16 , 21 , 97 , 50 , 146 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 38 , 96 , 36 , 130 , 1 , 82 , 127 , 65 , 100 , 100 , 114 , 101 , 115 , 115 , 58 , 32 , 105 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 98 , 97 , 108 , 97 , 110 , 99 , 101 , 32 , 102 , 111 , 96 , 68 , 130 , 1 , 82 , 101 , 28 , 136 , 24 , 216 , 91 , 27 , 96 , 210 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 128 , 134 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 133 , 135 , 96 , 64 , 81 , 97 , 50 , 174 , 145 , 144 , 97 , 62 , 9 , 86 , 91 , 96 , 0 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 133 , 135 , 90 , 241 , 146 , 80 , 80 , 80 , 61 , 128 , 96 , 0 , 129 , 20 , 97 , 50 , 235 , 87 , 96 , 64 , 81 , 145 , 80 , 96 , 31 , 25 , 96 , 63 , 61 , 1 , 22 , 130 , 1 , 96 , 64 , 82 , 61 , 130 , 82 , 61 , 96 , 0 , 96 , 32 , 132 , 1 , 62 , 97 , 50 , 240 , 86 , 91 , 96 , 96 , 145 , 80 , 91 , 80 , 145 , 80 , 145 , 80 , 97 , 51 , 1 , 135 , 131 , 131 , 135 , 97 , 51 , 12 , 86 , 91 , 151 , 150 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 96 , 131 , 21 , 97 , 51 , 123 , 87 , 130 , 81 , 96 , 0 , 3 , 97 , 51 , 116 , 87 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 59 , 97 , 51 , 116 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 29 , 96 , 36 , 130 , 1 , 82 , 127 , 65 , 100 , 100 , 114 , 101 , 115 , 115 , 58 , 32 , 99 , 97 , 108 , 108 , 32 , 116 , 111 , 32 , 110 , 111 , 110 , 45 , 99 , 111 , 110 , 116 , 114 , 97 , 99 , 116 , 0 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 80 , 129 , 97 , 50 , 41 , 86 , 91 , 97 , 50 , 41 , 131 , 131 , 129 , 81 , 21 , 97 , 51 , 144 , 87 , 129 , 81 , 128 , 131 , 96 , 32 , 1 , 253 , 91 , 128 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 145 , 144 , 97 , 51 , 248 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 51 , 188 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 129 , 22 , 129 , 20 , 97 , 24 , 223 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 91 , 131 , 129 , 16 , 21 , 97 , 51 , 239 , 87 , 129 , 129 , 1 , 81 , 131 , 130 , 1 , 82 , 96 , 32 , 1 , 97 , 51 , 215 , 86 , 91 , 80 , 80 , 96 , 0 , 145 , 1 , 82 , 86 , 91 , 96 , 32 , 129 , 82 , 96 , 0 , 130 , 81 , 128 , 96 , 32 , 132 , 1 , 82 , 97 , 52 , 23 , 129 , 96 , 64 , 133 , 1 , 96 , 32 , 135 , 1 , 97 , 51 , 212 , 86 , 91 , 96 , 31 , 1 , 96 , 31 , 25 , 22 , 145 , 144 , 145 , 1 , 96 , 64 , 1 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 20 , 255 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 53 , 99 , 255 , 255 , 255 , 255 , 129 , 22 , 129 , 20 , 97 , 28 , 54 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 52 , 103 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 52 , 114 , 129 , 97 , 52 , 43 , 86 , 91 , 145 , 80 , 97 , 52 , 128 , 96 , 32 , 132 , 1 , 97 , 52 , 64 , 86 , 91 , 144 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 52 , 156 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 52 , 167 , 129 , 97 , 52 , 43 , 86 , 91 , 148 , 96 , 32 , 147 , 144 , 147 , 1 , 53 , 147 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 96 , 132 , 134 , 3 , 18 , 21 , 97 , 52 , 202 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 53 , 97 , 52 , 213 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 80 , 96 , 32 , 132 , 1 , 53 , 97 , 52 , 229 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 149 , 146 , 148 , 80 , 80 , 80 , 96 , 64 , 145 , 144 , 145 , 1 , 53 , 144 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 53 , 8 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 53 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 53 , 34 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 145 , 80 , 96 , 32 , 131 , 1 , 53 , 97 , 53 , 52 , 129 , 97 , 52 , 43 , 86 , 91 , 128 , 145 , 80 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 96 , 132 , 134 , 3 , 18 , 21 , 97 , 53 , 84 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 53 , 97 , 53 , 95 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 80 , 96 , 32 , 132 , 1 , 53 , 145 , 80 , 96 , 64 , 132 , 1 , 53 , 97 , 53 , 118 , 129 , 97 , 52 , 43 , 86 , 91 , 128 , 145 , 80 , 80 , 146 , 80 , 146 , 80 , 146 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 53 , 147 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 97 , 24 , 223 , 129 , 97 , 52 , 43 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 128 , 133 , 135 , 3 , 18 , 21 , 97 , 53 , 180 , 87 , 96 , 0 , 128 , 253 , 91 , 132 , 53 , 97 , 53 , 191 , 129 , 97 , 52 , 43 , 86 , 91 , 147 , 80 , 96 , 32 , 133 , 1 , 53 , 97 , 53 , 207 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 80 , 96 , 64 , 133 , 1 , 53 , 145 , 80 , 96 , 96 , 133 , 1 , 53 , 97 , 53 , 230 , 129 , 97 , 52 , 43 , 86 , 91 , 147 , 150 , 146 , 149 , 80 , 144 , 147 , 80 , 80 , 86 , 91 , 128 , 21 , 21 , 129 , 20 , 97 , 20 , 255 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 54 , 17 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 97 , 24 , 223 , 129 , 97 , 53 , 241 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 54 , 47 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 80 , 128 , 53 , 146 , 96 , 32 , 144 , 145 , 1 , 53 , 145 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 130 , 81 , 130 , 130 , 1 , 129 , 144 , 82 , 96 , 0 , 145 , 144 , 132 , 130 , 1 , 144 , 96 , 64 , 133 , 1 , 144 , 132 , 91 , 129 , 129 , 16 , 21 , 97 , 54 , 127 , 87 , 131 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 131 , 82 , 146 , 132 , 1 , 146 , 145 , 132 , 1 , 145 , 96 , 1 , 1 , 97 , 54 , 90 , 86 , 91 , 80 , 144 , 150 , 149 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 128 , 53 , 97 , 255 , 255 , 129 , 22 , 129 , 20 , 97 , 28 , 54 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 54 , 176 , 87 , 96 , 0 , 128 , 253 , 91 , 97 , 52 , 114 , 131 , 97 , 54 , 139 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 54 , 204 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 54 , 215 , 129 , 97 , 52 , 43 , 86 , 91 , 145 , 80 , 96 , 32 , 131 , 1 , 53 , 97 , 53 , 52 , 129 , 97 , 52 , 43 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 192 , 135 , 137 , 3 , 18 , 21 , 97 , 55 , 0 , 87 , 96 , 0 , 128 , 253 , 91 , 97 , 55 , 9 , 135 , 97 , 54 , 139 , 86 , 91 , 149 , 80 , 96 , 32 , 135 , 1 , 53 , 97 , 55 , 25 , 129 , 97 , 52 , 43 , 86 , 91 , 148 , 80 , 96 , 64 , 135 , 1 , 53 , 97 , 55 , 41 , 129 , 97 , 52 , 43 , 86 , 91 , 147 , 80 , 96 , 96 , 135 , 1 , 53 , 146 , 80 , 96 , 128 , 135 , 1 , 53 , 97 , 55 , 64 , 129 , 97 , 53 , 241 , 86 , 91 , 145 , 80 , 96 , 160 , 135 , 1 , 53 , 97 , 55 , 80 , 129 , 97 , 52 , 43 , 86 , 91 , 128 , 145 , 80 , 80 , 146 , 149 , 80 , 146 , 149 , 80 , 146 , 149 , 86 , 91 , 96 , 1 , 129 , 129 , 28 , 144 , 130 , 22 , 128 , 97 , 55 , 114 , 87 , 96 , 127 , 130 , 22 , 145 , 80 , 91 , 96 , 32 , 130 , 16 , 129 , 3 , 97 , 55 , 146 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 34 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 145 , 144 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 57 , 144 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 79 , 110 , 108 , 121 , 32 , 104 , 97 , 110 , 100 , 108 , 96 , 64 , 130 , 1 , 82 , 127 , 101 , 114 , 32 , 99 , 97 , 110 , 32 , 99 , 97 , 108 , 108 , 32 , 116 , 104 , 105 , 115 , 32 , 102 , 117 , 110 , 99 , 116 , 105 , 111 , 110 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 110 , 111 , 96 , 64 , 130 , 1 , 82 , 98 , 110 , 99 , 101 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 17 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 128 , 130 , 1 , 128 , 130 , 17 , 21 , 97 , 11 , 22 , 87 , 97 , 11 , 22 , 97 , 56 , 56 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 58 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 78 , 111 , 110 , 99 , 101 , 32 , 109 , 117 , 115 , 116 , 96 , 64 , 130 , 1 , 82 , 127 , 32 , 110 , 111 , 116 , 32 , 105 , 110 , 99 , 114 , 101 , 109 , 101 , 110 , 116 , 32 , 109 , 111 , 114 , 101 , 32 , 116 , 104 , 97 , 110 , 32 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 50 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 96 , 0 , 96 , 1 , 130 , 1 , 97 , 56 , 230 , 87 , 97 , 56 , 230 , 97 , 56 , 56 , 86 , 91 , 80 , 96 , 1 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 118 , 101 , 32 , 109 , 105 , 110 , 116 , 101 , 114 , 32 , 114 , 96 , 64 , 130 , 1 , 82 , 98 , 111 , 108 , 101 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 41 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 110 , 97 , 116 , 105 , 118 , 96 , 64 , 130 , 1 , 82 , 104 , 101 , 32 , 98 , 97 , 108 , 97 , 110 , 99 , 101 , 96 , 184 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 69 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 78 , 97 , 116 , 105 , 118 , 101 , 32 , 117 , 110 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 32 , 96 , 64 , 130 , 1 , 82 , 127 , 105 , 115 , 32 , 110 , 111 , 116 , 32 , 97 , 108 , 108 , 111 , 119 , 101 , 100 , 32 , 102 , 111 , 114 , 32 , 116 , 104 , 105 , 115 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 119 , 114 , 96 , 96 , 130 , 1 , 82 , 100 , 48 , 184 , 56 , 50 , 185 , 96 , 217 , 27 , 96 , 128 , 130 , 1 , 82 , 96 , 160 , 1 , 144 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 57 , 246 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 81 , 145 , 144 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 40 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 69 , 82 , 67 , 50 , 48 , 96 , 64 , 130 , 1 , 82 , 103 , 32 , 98 , 97 , 108 , 97 , 110 , 99 , 101 , 96 , 192 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 46 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 104 , 105 , 115 , 116 , 111 , 114 , 105 , 99 , 97 , 108 , 96 , 64 , 130 , 1 , 82 , 109 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 144 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 57 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 97 , 109 , 111 , 117 , 110 , 116 , 32 , 112 , 114 , 111 , 96 , 64 , 130 , 1 , 82 , 127 , 118 , 105 , 100 , 101 , 100 , 32 , 102 , 111 , 114 , 32 , 110 , 97 , 116 , 105 , 118 , 101 , 32 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 67 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 78 , 97 , 116 , 105 , 118 , 101 , 32 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 32 , 105 , 115 , 96 , 64 , 130 , 1 , 82 , 127 , 32 , 110 , 111 , 116 , 32 , 97 , 108 , 108 , 111 , 119 , 101 , 100 , 32 , 102 , 111 , 114 , 32 , 116 , 104 , 105 , 115 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 119 , 114 , 97 , 112 , 96 , 96 , 130 , 1 , 82 , 98 , 56 , 50 , 185 , 96 , 233 , 27 , 96 , 128 , 130 , 1 , 82 , 96 , 160 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 45 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 118 , 97 , 108 , 117 , 101 , 32 , 115 , 101 , 110 , 116 , 96 , 64 , 130 , 1 , 82 , 108 , 32 , 102 , 111 , 114 , 32 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 96 , 152 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 97 , 100 , 100 , 114 , 96 , 64 , 130 , 1 , 82 , 98 , 101 , 115 , 115 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 50 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 70 , 101 , 101 , 32 , 82 , 101 , 99 , 105 , 112 , 105 , 101 , 110 , 116 , 32 , 99 , 97 , 110 , 110 , 96 , 64 , 130 , 1 , 82 , 113 , 111 , 116 , 32 , 98 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 112 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 34 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 97 , 109 , 111 , 117 , 96 , 64 , 130 , 1 , 82 , 97 , 27 , 157 , 96 , 242 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 128 , 130 , 2 , 129 , 21 , 130 , 130 , 4 , 132 , 20 , 23 , 97 , 11 , 22 , 87 , 97 , 11 , 22 , 97 , 56 , 56 , 86 , 91 , 96 , 0 , 130 , 97 , 60 , 177 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 18 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 4 , 144 , 86 , 91 , 97 , 255 , 255 , 130 , 129 , 22 , 130 , 130 , 22 , 3 , 144 , 128 , 130 , 17 , 21 , 97 , 50 , 19 , 87 , 97 , 50 , 19 , 97 , 56 , 56 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 48 , 144 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 72 , 97 , 110 , 100 , 108 , 101 , 114 , 32 , 65 , 100 , 96 , 64 , 130 , 1 , 82 , 111 , 6 , 71 , 38 , 87 , 55 , 50 , 6 , 54 , 22 , 226 , 119 , 66 , 6 , 38 , 82 , 3 , 96 , 132 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 129 , 129 , 3 , 129 , 129 , 17 , 21 , 97 , 11 , 22 , 87 , 97 , 11 , 22 , 97 , 56 , 56 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 49 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 127 , 65 , 99 , 99 , 101 , 115 , 115 , 67 , 111 , 110 , 116 , 114 , 111 , 108 , 58 , 32 , 97 , 99 , 99 , 111 , 117 , 110 , 116 , 32 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 129 , 82 , 96 , 0 , 131 , 81 , 97 , 61 , 130 , 129 , 96 , 23 , 133 , 1 , 96 , 32 , 136 , 1 , 97 , 51 , 212 , 86 , 91 , 112 , 1 , 3 , 75 , 153 , 3 , 107 , 75 , 155 , 155 , 75 , 115 , 57 , 3 , 147 , 123 , 99 , 41 , 96 , 125 , 27 , 96 , 23 , 145 , 132 , 1 , 145 , 130 , 1 , 82 , 131 , 81 , 97 , 61 , 179 , 129 , 96 , 40 , 132 , 1 , 96 , 32 , 136 , 1 , 97 , 51 , 212 , 86 , 91 , 1 , 96 , 40 , 1 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 61 , 209 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 97 , 24 , 223 , 129 , 97 , 53 , 241 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 65 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 96 , 0 , 129 , 97 , 62 , 1 , 87 , 97 , 62 , 1 , 97 , 56 , 56 , 86 , 91 , 80 , 96 , 0 , 25 , 1 , 144 , 86 , 91 , 96 , 0 , 130 , 81 , 97 , 62 , 27 , 129 , 132 , 96 , 32 , 135 , 1 , 97 , 51 , 212 , 86 , 91 , 145 , 144 , 145 , 1 , 146 , 145 , 80 , 80 , 86 , 254 , 101 , 215 , 162 , 142 , 50 , 101 , 179 , 122 , 100 , 116 , 146 , 159 , 51 , 101 , 33 , 179 , 50 , 193 , 104 , 27 , 147 , 63 , 108 , 185 , 243 , 55 , 102 , 115 , 68 , 13 , 134 , 42 , 159 , 45 , 240 , 254 , 210 , 199 , 118 , 72 , 222 , 88 , 96 , 164 , 204 , 80 , 140 , 208 , 129 , 140 , 133 , 184 , 184 , 161 , 171 , 76 , 238 , 239 , 141 , 152 , 28 , 137 , 86 , 166 , 162 , 100 , 105 , 112 , 102 , 115 , 88 , 34 , 18 , 32 , 178 , 146 , 213 , 143 , 104 , 179 , 51 , 146 , 89 , 134 , 143 , 195 , 133 , 98 , 181 , 91 , 159 , 88 , 25 , 143 , 97 , 73 , 62 , 116 , 56 , 5 , 135 , 103 , 39 , 145 , 124 , 199 , 100 , 115 , 111 , 108 , 99 , 67 , 0 , 8 , 18 , 0 , 51] ;
    #[doc = "The bytecode of the contract."]
    pub static FUNGIBLETOKENWRAPPERCONTRACT_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    # [rustfmt :: skip] const __DEPLOYED_BYTECODE : & [u8] = & [96 , 128 , 96 , 64 , 82 , 96 , 4 , 54 , 16 , 97 , 3 , 140 , 87 , 96 , 0 , 53 , 96 , 224 , 28 , 128 , 99 , 133 , 192 , 10 , 232 , 17 , 97 , 1 , 220 , 87 , 128 , 99 , 186 , 196 , 38 , 208 , 17 , 97 , 1 , 2 , 87 , 128 , 99 , 206 , 215 , 47 , 135 , 17 , 97 , 0 , 160 , 87 , 128 , 99 , 230 , 58 , 177 , 233 , 17 , 97 , 0 , 111 , 87 , 128 , 99 , 230 , 58 , 177 , 233 , 20 , 97 , 10 , 111 , 87 , 128 , 99 , 246 , 62 , 187 , 69 , 20 , 97 , 10 , 145 , 87 , 128 , 99 , 250 , 224 , 149 , 154 , 20 , 97 , 10 , 177 , 87 , 128 , 99 , 252 , 151 , 166 , 82 , 20 , 97 , 10 , 209 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 206 , 215 , 47 , 135 , 20 , 97 , 9 , 244 , 87 , 128 , 99 , 213 , 57 , 19 , 147 , 20 , 97 , 10 , 13 , 87 , 128 , 99 , 213 , 71 , 116 , 31 , 20 , 97 , 10 , 47 , 87 , 128 , 99 , 221 , 98 , 237 , 62 , 20 , 97 , 10 , 79 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 194 , 174 , 71 , 32 , 17 , 97 , 0 , 220 , 87 , 128 , 99 , 194 , 174 , 71 , 32 , 20 , 97 , 9 , 126 , 87 , 128 , 99 , 200 , 9 , 22 , 212 , 20 , 97 , 9 , 158 , 87 , 128 , 99 , 202 , 21 , 200 , 115 , 20 , 97 , 9 , 190 , 87 , 128 , 99 , 204 , 60 , 116 , 161 , 20 , 97 , 9 , 222 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 186 , 196 , 38 , 208 , 20 , 97 , 9 , 43 , 87 , 128 , 99 , 191 , 55 , 108 , 122 , 20 , 97 , 9 , 75 , 87 , 128 , 99 , 193 , 135 , 100 , 83 , 20 , 97 , 9 , 94 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 160 , 1 , 236 , 221 , 17 , 97 , 1 , 122 , 87 , 128 , 99 , 170 , 108 , 168 , 8 , 17 , 97 , 1 , 73 , 87 , 128 , 99 , 170 , 108 , 168 , 8 , 20 , 97 , 8 , 143 , 87 , 128 , 99 , 172 , 138 , 38 , 12 , 20 , 97 , 8 , 177 , 87 , 128 , 99 , 177 , 203 , 162 , 88 , 20 , 97 , 8 , 225 , 87 , 128 , 99 , 179 , 228 , 8 , 63 , 20 , 97 , 9 , 17 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 160 , 1 , 236 , 221 , 20 , 97 , 8 , 12 , 87 , 128 , 99 , 162 , 23 , 253 , 223 , 20 , 97 , 8 , 58 , 87 , 128 , 99 , 164 , 87 , 194 , 215 , 20 , 97 , 8 , 79 , 87 , 128 , 99 , 169 , 5 , 156 , 187 , 20 , 97 , 8 , 111 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 144 , 16 , 208 , 124 , 17 , 97 , 1 , 182 , 87 , 128 , 99 , 144 , 16 , 208 , 124 , 20 , 97 , 7 , 151 , 87 , 128 , 99 , 145 , 209 , 72 , 84 , 20 , 97 , 7 , 183 , 87 , 128 , 99 , 149 , 216 , 155 , 65 , 20 , 97 , 7 , 215 , 87 , 128 , 99 , 150 , 205 , 77 , 254 , 20 , 97 , 7 , 236 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 133 , 192 , 10 , 232 , 20 , 97 , 7 , 55 , 87 , 128 , 99 , 133 , 209 , 72 , 52 , 20 , 97 , 7 , 87 , 87 , 128 , 99 , 139 , 84 , 120 , 185 , 20 , 97 , 7 , 119 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 49 , 60 , 229 , 103 , 17 , 97 , 2 , 193 , 87 , 128 , 99 , 70 , 144 , 72 , 64 , 17 , 97 , 2 , 95 , 87 , 128 , 99 , 112 , 160 , 130 , 49 , 17 , 97 , 2 , 46 , 87 , 128 , 99 , 112 , 160 , 130 , 49 , 20 , 97 , 6 , 185 , 87 , 128 , 99 , 121 , 204 , 103 , 144 , 20 , 97 , 6 , 239 , 87 , 128 , 99 , 123 , 46 , 48 , 214 , 20 , 97 , 7 , 15 , 87 , 128 , 99 , 132 , 86 , 203 , 89 , 20 , 97 , 7 , 34 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 70 , 144 , 72 , 64 , 20 , 97 , 6 , 35 , 87 , 128 , 99 , 72 , 8 , 40 , 94 , 20 , 97 , 6 , 97 , 87 , 128 , 99 , 79 , 100 , 178 , 190 , 20 , 97 , 6 , 129 , 87 , 128 , 99 , 92 , 151 , 90 , 187 , 20 , 97 , 6 , 161 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 57 , 244 , 118 , 147 , 17 , 97 , 2 , 155 , 87 , 128 , 99 , 57 , 244 , 118 , 147 , 20 , 97 , 5 , 174 , 87 , 128 , 99 , 63 , 75 , 168 , 58 , 20 , 97 , 5 , 206 , 87 , 128 , 99 , 64 , 193 , 15 , 25 , 20 , 97 , 5 , 227 , 87 , 128 , 99 , 66 , 150 , 108 , 104 , 20 , 97 , 6 , 3 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 49 , 60 , 229 , 103 , 20 , 97 , 5 , 82 , 87 , 128 , 99 , 54 , 86 , 138 , 190 , 20 , 97 , 5 , 110 , 87 , 128 , 99 , 57 , 80 , 147 , 81 , 20 , 97 , 5 , 142 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 28 , 74 , 20 , 54 , 17 , 97 , 3 , 46 , 87 , 128 , 99 , 36 , 138 , 156 , 163 , 17 , 97 , 3 , 8 , 87 , 128 , 99 , 36 , 138 , 156 , 163 , 20 , 97 , 4 , 206 , 87 , 128 , 99 , 38 , 28 , 128 , 182 , 20 , 97 , 4 , 255 , 87 , 128 , 99 , 44 , 166 , 147 , 136 , 20 , 97 , 5 , 31 , 87 , 128 , 99 , 47 , 47 , 241 , 93 , 20 , 97 , 5 , 50 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 28 , 74 , 20 , 54 , 20 , 97 , 4 , 120 , 87 , 128 , 99 , 31 , 145 , 67 , 130 , 20 , 97 , 4 , 152 , 87 , 128 , 99 , 35 , 184 , 114 , 221 , 20 , 97 , 4 , 174 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 9 , 94 , 167 , 179 , 17 , 97 , 3 , 106 , 87 , 128 , 99 , 9 , 94 , 167 , 179 , 20 , 97 , 4 , 10 , 87 , 128 , 99 , 11 , 39 , 251 , 154 , 20 , 97 , 4 , 42 , 87 , 128 , 99 , 21 , 142 , 249 , 62 , 20 , 97 , 4 , 73 , 87 , 128 , 99 , 24 , 22 , 13 , 221 , 20 , 97 , 4 , 99 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 1 , 255 , 201 , 167 , 20 , 97 , 3 , 145 , 87 , 128 , 99 , 6 , 253 , 222 , 3 , 20 , 97 , 3 , 198 , 87 , 128 , 99 , 7 , 24 , 79 , 28 , 20 , 97 , 3 , 232 , 87 , 91 , 96 , 0 , 128 , 253 , 91 , 52 , 128 , 21 , 97 , 3 , 157 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 3 , 172 , 54 , 96 , 4 , 97 , 51 , 170 , 86 , 91 , 97 , 10 , 241 , 86 , 91 , 96 , 64 , 81 , 144 , 21 , 21 , 129 , 82 , 96 , 32 , 1 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 243 , 91 , 52 , 128 , 21 , 97 , 3 , 210 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 219 , 97 , 11 , 28 , 86 , 91 , 96 , 64 , 81 , 97 , 3 , 189 , 145 , 144 , 97 , 51 , 248 , 86 , 91 , 52 , 128 , 21 , 97 , 3 , 244 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 4 , 3 , 54 , 96 , 4 , 97 , 52 , 84 , 86 , 91 , 97 , 11 , 174 , 86 , 91 , 0 , 91 , 52 , 128 , 21 , 97 , 4 , 22 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 4 , 37 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 12 , 227 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 54 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 11 , 84 , 91 , 96 , 64 , 81 , 144 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 85 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 0 , 84 , 97 , 3 , 177 , 144 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 111 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 5 , 84 , 97 , 4 , 59 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 132 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 4 , 147 , 54 , 96 , 4 , 97 , 52 , 84 , 86 , 91 , 97 , 12 , 251 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 164 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 18 , 84 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 186 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 4 , 201 , 54 , 96 , 4 , 97 , 52 , 181 , 86 , 91 , 97 , 14 , 233 , 86 , 91 , 52 , 128 , 21 , 97 , 4 , 218 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 4 , 233 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 96 , 0 , 144 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 144 , 82 , 96 , 64 , 144 , 145 , 32 , 1 , 84 , 144 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 11 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 26 , 54 , 96 , 4 , 97 , 52 , 181 , 86 , 91 , 97 , 15 , 13 , 86 , 91 , 97 , 4 , 8 , 97 , 5 , 45 , 54 , 96 , 4 , 97 , 52 , 181 , 86 , 91 , 97 , 16 , 125 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 62 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 77 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 17 , 215 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 94 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 64 , 81 , 96 , 18 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 122 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 137 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 17 , 253 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 154 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 5 , 169 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 18 , 123 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 186 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 201 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 18 , 157 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 218 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 19 , 212 , 86 , 91 , 52 , 128 , 21 , 97 , 5 , 239 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 5 , 254 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 20 , 104 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 15 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 6 , 30 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 20 , 245 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 47 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 10 , 84 , 97 , 6 , 73 , 144 , 98 , 1 , 0 , 0 , 144 , 4 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 109 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 6 , 124 , 54 , 96 , 4 , 97 , 53 , 63 , 86 , 91 , 97 , 21 , 2 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 141 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 6 , 73 , 97 , 6 , 156 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 22 , 45 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 173 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 8 , 84 , 96 , 255 , 22 , 97 , 3 , 177 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 197 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 6 , 212 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 144 , 86 , 91 , 52 , 128 , 21 , 97 , 6 , 251 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 7 , 10 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 22 , 87 , 86 , 91 , 97 , 4 , 8 , 97 , 7 , 29 , 54 , 96 , 4 , 97 , 53 , 158 , 86 , 91 , 97 , 22 , 108 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 46 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 23 , 198 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 67 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 7 , 82 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 24 , 88 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 99 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 6 , 73 , 97 , 7 , 114 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 24 , 122 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 131 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 7 , 146 , 54 , 96 , 4 , 97 , 53 , 255 , 86 , 91 , 97 , 24 , 138 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 163 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 6 , 73 , 97 , 7 , 178 , 54 , 96 , 4 , 97 , 54 , 28 , 86 , 91 , 97 , 24 , 199 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 195 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 7 , 210 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 24 , 230 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 227 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 219 , 97 , 25 , 17 , 86 , 91 , 52 , 128 , 21 , 97 , 7 , 248 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 8 , 7 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 25 , 32 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 24 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 10 , 84 , 97 , 8 , 39 , 144 , 97 , 255 , 255 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 97 , 255 , 255 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 97 , 3 , 189 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 70 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 0 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 91 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 106 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 25 , 70 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 123 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 138 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 25 , 193 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 155 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 8 , 164 , 97 , 25 , 207 , 86 , 91 , 96 , 64 , 81 , 97 , 3 , 189 , 145 , 144 , 97 , 54 , 62 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 189 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 204 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 96 , 15 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 8 , 237 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 8 , 252 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 96 , 16 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 29 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 17 , 84 , 97 , 3 , 177 , 144 , 96 , 255 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 55 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 9 , 70 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 97 , 26 , 48 , 86 , 91 , 97 , 4 , 8 , 97 , 9 , 89 , 54 , 96 , 4 , 97 , 52 , 137 , 86 , 91 , 97 , 26 , 212 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 106 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 3 , 177 , 97 , 9 , 121 , 54 , 96 , 4 , 97 , 53 , 129 , 86 , 91 , 97 , 27 , 250 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 138 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 9 , 153 , 54 , 96 , 4 , 97 , 54 , 157 , 86 , 91 , 97 , 28 , 59 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 170 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 12 , 84 , 97 , 6 , 73 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 202 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 9 , 217 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 29 , 67 , 86 , 91 , 52 , 128 , 21 , 97 , 9 , 234 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 11 , 84 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 0 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 10 , 84 , 97 , 255 , 255 , 22 , 97 , 8 , 39 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 25 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 59 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 74 , 54 , 96 , 4 , 97 , 53 , 15 , 86 , 91 , 97 , 29 , 90 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 91 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 97 , 10 , 106 , 54 , 96 , 4 , 97 , 54 , 185 , 86 , 91 , 97 , 29 , 128 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 123 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 59 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 129 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 157 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 172 , 54 , 96 , 4 , 97 , 54 , 231 , 86 , 91 , 97 , 29 , 171 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 189 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 204 , 54 , 96 , 4 , 97 , 52 , 246 , 86 , 91 , 97 , 31 , 125 , 86 , 91 , 52 , 128 , 21 , 97 , 10 , 221 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 4 , 8 , 97 , 10 , 236 , 54 , 96 , 4 , 97 , 52 , 84 , 86 , 91 , 97 , 31 , 172 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 130 , 22 , 99 , 90 , 5 , 24 , 15 , 96 , 224 , 27 , 20 , 128 , 97 , 11 , 22 , 87 , 80 , 97 , 11 , 22 , 130 , 97 , 34 , 26 , 86 , 91 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 96 , 96 , 6 , 128 , 84 , 97 , 11 , 43 , 144 , 97 , 55 , 94 , 86 , 91 , 128 , 96 , 31 , 1 , 96 , 32 , 128 , 145 , 4 , 2 , 96 , 32 , 1 , 96 , 64 , 81 , 144 , 129 , 1 , 96 , 64 , 82 , 128 , 146 , 145 , 144 , 129 , 129 , 82 , 96 , 32 , 1 , 130 , 128 , 84 , 97 , 11 , 87 , 144 , 97 , 55 , 94 , 86 , 91 , 128 , 21 , 97 , 11 , 164 , 87 , 128 , 96 , 31 , 16 , 97 , 11 , 121 , 87 , 97 , 1 , 0 , 128 , 131 , 84 , 4 , 2 , 131 , 82 , 145 , 96 , 32 , 1 , 145 , 97 , 11 , 164 , 86 , 91 , 130 , 1 , 145 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 144 , 91 , 129 , 84 , 129 , 82 , 144 , 96 , 1 , 1 , 144 , 96 , 32 , 1 , 128 , 131 , 17 , 97 , 11 , 135 , 87 , 130 , 144 , 3 , 96 , 31 , 22 , 130 , 1 , 145 , 91 , 80 , 80 , 80 , 80 , 80 , 144 , 80 , 144 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 11 , 225 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 12 , 9 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 12 , 23 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 12 , 54 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 12 , 183 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 58 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 70 , 101 , 101 , 32 , 82 , 101 , 99 , 105 , 112 , 105 , 96 , 68 , 130 , 1 , 82 , 127 , 101 , 110 , 116 , 32 , 99 , 97 , 110 , 110 , 111 , 116 , 32 , 98 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 80 , 80 , 96 , 10 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 146 , 22 , 98 , 1 , 0 , 0 , 2 , 98 , 1 , 0 , 0 , 96 , 1 , 96 , 176 , 27 , 3 , 25 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 96 , 0 , 51 , 97 , 12 , 241 , 129 , 133 , 133 , 97 , 34 , 79 , 86 , 91 , 80 , 96 , 1 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 13 , 37 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 13 , 77 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 13 , 91 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 13 , 122 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 13 , 251 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 43 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 84 , 111 , 107 , 101 , 110 , 32 , 115 , 104 , 111 , 117 , 96 , 68 , 130 , 1 , 82 , 106 , 27 , 25 , 8 , 24 , 153 , 72 , 29 , 152 , 91 , 26 , 89 , 96 , 170 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 128 , 91 , 96 , 13 , 84 , 129 , 16 , 21 , 97 , 14 , 90 , 87 , 132 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 96 , 13 , 130 , 129 , 84 , 129 , 16 , 97 , 14 , 38 , 87 , 97 , 14 , 38 , 97 , 56 , 190 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 144 , 145 , 32 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 3 , 97 , 14 , 72 , 87 , 128 , 145 , 80 , 97 , 14 , 90 , 86 , 91 , 128 , 97 , 14 , 82 , 129 , 97 , 56 , 212 , 86 , 91 , 145 , 80 , 80 , 97 , 13 , 255 , 86 , 91 , 80 , 96 , 13 , 84 , 129 , 16 , 97 , 14 , 186 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 84 , 111 , 107 , 101 , 110 , 32 , 110 , 111 , 116 , 32 , 96 , 68 , 130 , 1 , 82 , 100 , 25 , 155 , 221 , 91 , 153 , 96 , 218 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 97 , 14 , 227 , 129 , 97 , 35 , 115 , 86 , 91 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 51 , 97 , 14 , 247 , 133 , 130 , 133 , 97 , 36 , 35 , 86 , 91 , 97 , 15 , 2 , 133 , 133 , 133 , 97 , 36 , 151 , 86 , 91 , 80 , 96 , 1 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 97 , 15 , 21 , 97 , 38 , 77 , 86 , 91 , 97 , 15 , 45 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 15 , 73 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 237 , 86 , 91 , 129 , 129 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 15 , 160 , 87 , 128 , 71 , 16 , 21 , 97 , 15 , 121 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 48 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 15 , 155 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 121 , 86 , 91 , 97 , 16 , 96 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 129 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 15 , 230 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 16 , 10 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 16 , 21 , 97 , 16 , 40 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 253 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 16 , 96 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 69 , 86 , 91 , 97 , 16 , 108 , 133 , 133 , 133 , 136 , 97 , 38 , 166 , 86 , 91 , 80 , 80 , 97 , 16 , 120 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 80 , 80 , 80 , 86 , 91 , 97 , 16 , 133 , 97 , 38 , 77 , 86 , 91 , 97 , 16 , 157 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 16 , 185 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 237 , 86 , 91 , 96 , 10 , 84 , 130 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 129 , 22 , 144 , 131 , 144 , 131 , 22 , 97 , 17 , 29 , 87 , 128 , 21 , 97 , 16 , 246 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 147 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 17 , 24 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 240 , 86 , 91 , 97 , 17 , 115 , 86 , 91 , 52 , 21 , 97 , 17 , 59 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 89 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 17 , 115 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 166 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 17 , 153 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 233 , 86 , 91 , 97 , 17 , 162 , 129 , 97 , 39 , 14 , 86 , 91 , 97 , 17 , 190 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 59 , 86 , 91 , 97 , 17 , 202 , 134 , 134 , 134 , 137 , 97 , 39 , 135 , 86 , 91 , 80 , 80 , 80 , 97 , 16 , 120 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 144 , 82 , 96 , 64 , 144 , 145 , 32 , 1 , 84 , 97 , 17 , 243 , 129 , 97 , 40 , 114 , 86 , 91 , 97 , 16 , 120 , 131 , 131 , 97 , 40 , 124 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 51 , 20 , 97 , 18 , 109 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 47 , 96 , 36 , 130 , 1 , 82 , 127 , 65 , 99 , 99 , 101 , 115 , 115 , 67 , 111 , 110 , 116 , 114 , 111 , 108 , 58 , 32 , 99 , 97 , 110 , 32 , 111 , 110 , 108 , 121 , 32 , 114 , 101 , 110 , 111 , 117 , 110 , 99 , 101 , 96 , 68 , 130 , 1 , 82 , 110 , 16 , 57 , 55 , 182 , 50 , 185 , 144 , 51 , 55 , 185 , 16 , 57 , 178 , 182 , 51 , 96 , 137 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 40 , 158 , 86 , 91 , 80 , 80 , 86 , 91 , 96 , 0 , 51 , 97 , 12 , 241 , 129 , 133 , 133 , 97 , 18 , 142 , 131 , 131 , 97 , 29 , 128 , 86 , 91 , 97 , 18 , 152 , 145 , 144 , 97 , 56 , 78 , 86 , 91 , 97 , 34 , 79 , 86 , 91 , 97 , 18 , 165 , 97 , 38 , 77 , 86 , 91 , 129 , 129 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 18 , 252 , 87 , 128 , 71 , 16 , 21 , 97 , 18 , 213 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 48 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 18 , 247 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 121 , 86 , 91 , 97 , 19 , 188 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 129 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 19 , 66 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 19 , 102 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 16 , 21 , 97 , 19 , 132 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 253 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 19 , 188 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 69 , 86 , 91 , 97 , 19 , 200 , 51 , 133 , 133 , 51 , 97 , 38 , 166 , 86 , 91 , 80 , 80 , 97 , 18 , 119 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 97 , 19 , 236 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 20 , 94 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 57 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 114 , 101 , 115 , 101 , 116 , 77 , 105 , 110 , 116 , 101 , 114 , 80 , 97 , 117 , 115 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 96 , 68 , 130 , 1 , 82 , 127 , 118 , 101 , 32 , 112 , 97 , 117 , 115 , 101 , 114 , 32 , 114 , 111 , 108 , 101 , 32 , 116 , 111 , 32 , 117 , 110 , 112 , 97 , 117 , 115 , 101 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 20 , 102 , 97 , 40 , 192 , 86 , 91 , 86 , 91 , 97 , 20 , 128 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 20 , 235 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 54 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 114 , 101 , 115 , 101 , 116 , 77 , 105 , 110 , 116 , 101 , 114 , 80 , 97 , 117 , 115 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 96 , 68 , 130 , 1 , 82 , 117 , 29 , 153 , 72 , 27 , 90 , 91 , 157 , 25 , 92 , 136 , 28 , 155 , 219 , 25 , 72 , 29 , 27 , 200 , 27 , 90 , 91 , 157 , 96 , 82 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 41 , 18 , 86 , 91 , 97 , 20 , 255 , 51 , 130 , 97 , 41 , 223 , 86 , 91 , 80 , 86 , 91 , 97 , 21 , 10 , 97 , 38 , 77 , 86 , 91 , 130 , 130 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 21 , 97 , 87 , 128 , 71 , 16 , 21 , 97 , 21 , 58 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 48 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 21 , 92 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 121 , 86 , 91 , 97 , 22 , 33 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 129 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 21 , 167 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 21 , 203 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 16 , 21 , 97 , 21 , 233 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 57 , 253 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 22 , 33 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 69 , 86 , 91 , 97 , 16 , 108 , 51 , 134 , 134 , 134 , 97 , 38 , 166 , 86 , 91 , 96 , 13 , 129 , 129 , 84 , 129 , 16 , 97 , 22 , 61 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 144 , 145 , 32 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 144 , 80 , 129 , 86 , 91 , 97 , 22 , 98 , 130 , 51 , 131 , 97 , 36 , 35 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 41 , 223 , 86 , 91 , 97 , 22 , 116 , 97 , 38 , 77 , 86 , 91 , 97 , 22 , 140 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 22 , 168 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 237 , 86 , 91 , 96 , 10 , 84 , 131 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 129 , 22 , 144 , 132 , 144 , 131 , 22 , 97 , 23 , 12 , 87 , 128 , 21 , 97 , 22 , 229 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 147 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 23 , 7 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 240 , 86 , 91 , 97 , 23 , 98 , 86 , 91 , 52 , 21 , 97 , 23 , 42 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 89 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 23 , 98 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 166 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 23 , 136 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 233 , 86 , 91 , 97 , 23 , 145 , 129 , 97 , 39 , 14 , 86 , 91 , 97 , 23 , 173 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 59 , 86 , 91 , 97 , 23 , 185 , 135 , 135 , 135 , 135 , 97 , 39 , 135 , 86 , 91 , 80 , 80 , 80 , 97 , 14 , 227 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 97 , 23 , 222 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 51 , 97 , 24 , 230 , 86 , 91 , 97 , 24 , 80 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 55 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 114 , 101 , 115 , 101 , 116 , 77 , 105 , 110 , 116 , 101 , 114 , 80 , 97 , 117 , 115 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 96 , 68 , 130 , 1 , 82 , 127 , 118 , 101 , 32 , 112 , 97 , 117 , 115 , 101 , 114 , 32 , 114 , 111 , 108 , 101 , 32 , 116 , 111 , 32 , 112 , 97 , 117 , 115 , 101 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 20 , 102 , 97 , 43 , 31 , 86 , 91 , 96 , 10 , 84 , 96 , 0 , 144 , 97 , 39 , 16 , 144 , 97 , 24 , 112 , 144 , 97 , 255 , 255 , 22 , 132 , 97 , 60 , 125 , 86 , 91 , 97 , 11 , 22 , 145 , 144 , 97 , 60 , 148 , 86 , 91 , 96 , 14 , 129 , 129 , 84 , 129 , 16 , 97 , 22 , 61 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 24 , 180 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 17 , 128 , 84 , 96 , 255 , 25 , 22 , 145 , 21 , 21 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 97 , 24 , 223 , 144 , 131 , 97 , 43 , 92 , 86 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 1 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 147 , 144 , 147 , 22 , 132 , 82 , 145 , 144 , 82 , 144 , 32 , 84 , 96 , 255 , 22 , 144 , 86 , 91 , 96 , 96 , 96 , 7 , 128 , 84 , 97 , 11 , 43 , 144 , 97 , 55 , 94 , 86 , 91 , 96 , 10 , 84 , 96 , 0 , 144 , 97 , 25 , 54 , 144 , 97 , 255 , 255 , 22 , 97 , 39 , 16 , 97 , 60 , 182 , 86 , 91 , 97 , 255 , 255 , 22 , 97 , 24 , 112 , 131 , 97 , 39 , 16 , 97 , 60 , 125 , 86 , 91 , 96 , 0 , 51 , 129 , 97 , 25 , 84 , 130 , 134 , 97 , 29 , 128 , 86 , 91 , 144 , 80 , 131 , 129 , 16 , 21 , 97 , 25 , 180 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 100 , 101 , 99 , 114 , 101 , 97 , 115 , 101 , 100 , 32 , 97 , 108 , 108 , 111 , 119 , 97 , 110 , 99 , 101 , 32 , 98 , 101 , 108 , 111 , 119 , 96 , 68 , 130 , 1 , 82 , 100 , 32 , 122 , 101 , 114 , 111 , 96 , 216 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 15 , 2 , 130 , 134 , 134 , 132 , 3 , 97 , 34 , 79 , 86 , 91 , 96 , 0 , 51 , 97 , 12 , 241 , 129 , 133 , 133 , 97 , 36 , 151 , 86 , 91 , 96 , 96 , 96 , 13 , 128 , 84 , 128 , 96 , 32 , 2 , 96 , 32 , 1 , 96 , 64 , 81 , 144 , 129 , 1 , 96 , 64 , 82 , 128 , 146 , 145 , 144 , 129 , 129 , 82 , 96 , 32 , 1 , 130 , 128 , 84 , 128 , 21 , 97 , 11 , 164 , 87 , 96 , 32 , 2 , 130 , 1 , 145 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 144 , 91 , 129 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 82 , 96 , 1 , 144 , 145 , 1 , 144 , 96 , 32 , 1 , 128 , 131 , 17 , 97 , 26 , 9 , 87 , 80 , 80 , 80 , 80 , 80 , 144 , 80 , 144 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 26 , 90 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 97 , 26 , 128 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 209 , 86 , 91 , 96 , 12 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 96 , 64 , 81 , 144 , 129 , 82 , 127 , 254 , 20 , 152 , 67 , 164 , 64 , 75 , 67 , 105 , 157 , 68 , 108 , 153 , 201 , 190 , 45 , 122 , 91 , 252 , 139 , 214 , 110 , 21 , 202 , 76 , 250 , 213 , 202 , 40 , 17 , 221 , 155 , 144 , 96 , 32 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 161 , 80 , 86 , 91 , 97 , 26 , 220 , 97 , 38 , 77 , 86 , 91 , 96 , 10 , 84 , 130 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 129 , 22 , 144 , 131 , 144 , 131 , 22 , 97 , 27 , 64 , 87 , 128 , 21 , 97 , 27 , 25 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 147 , 86 , 91 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 27 , 59 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 58 , 240 , 86 , 91 , 97 , 27 , 150 , 86 , 91 , 52 , 21 , 97 , 27 , 94 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 89 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 27 , 150 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 166 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 27 , 188 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 59 , 233 , 86 , 91 , 97 , 27 , 197 , 129 , 97 , 39 , 14 , 86 , 91 , 97 , 27 , 225 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 59 , 86 , 91 , 97 , 27 , 237 , 51 , 134 , 134 , 51 , 97 , 39 , 135 , 86 , 91 , 80 , 80 , 80 , 97 , 18 , 119 , 96 , 1 , 96 , 9 , 85 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 28 , 21 , 87 , 96 , 17 , 84 , 96 , 255 , 22 , 97 , 11 , 22 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 11 , 22 , 86 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 28 , 101 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 28 , 141 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 28 , 155 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 28 , 186 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 97 , 39 , 16 , 97 , 255 , 255 , 132 , 22 , 16 , 97 , 29 , 41 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 44 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 102 , 101 , 96 , 68 , 130 , 1 , 82 , 107 , 101 , 32 , 112 , 101 , 114 , 99 , 101 , 110 , 116 , 97 , 103 , 101 , 96 , 160 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 80 , 80 , 96 , 10 , 128 , 84 , 97 , 255 , 255 , 25 , 22 , 97 , 255 , 255 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 97 , 11 , 22 , 144 , 97 , 43 , 104 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 144 , 82 , 96 , 64 , 144 , 145 , 32 , 1 , 84 , 97 , 29 , 118 , 129 , 97 , 40 , 114 , 86 , 91 , 97 , 16 , 120 , 131 , 131 , 97 , 40 , 158 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 145 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 4 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 147 , 144 , 148 , 22 , 130 , 82 , 145 , 144 , 145 , 82 , 32 , 84 , 144 , 86 , 91 , 96 , 0 , 84 , 96 , 255 , 22 , 21 , 97 , 29 , 254 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 129 , 144 , 82 , 96 , 36 , 130 , 1 , 82 , 127 , 73 , 110 , 105 , 116 , 105 , 97 , 108 , 105 , 122 , 101 , 100 , 58 , 32 , 65 , 108 , 114 , 101 , 97 , 100 , 121 , 32 , 105 , 110 , 105 , 116 , 105 , 97 , 108 , 105 , 122 , 101 , 100 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 97 , 30 , 115 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 54 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 70 , 101 , 101 , 32 , 82 , 101 , 99 , 105 , 112 , 105 , 96 , 68 , 130 , 1 , 82 , 117 , 6 , 86 , 231 , 66 , 4 , 22 , 70 , 71 , 38 , 87 , 55 , 50 , 6 , 54 , 22 , 226 , 119 , 66 , 6 , 38 , 82 , 3 , 96 , 84 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 30 , 153 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 60 , 209 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 97 , 31 , 6 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 46 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 65 , 100 , 109 , 105 , 110 , 32 , 65 , 100 , 100 , 114 , 96 , 68 , 130 , 1 , 82 , 109 , 6 , 87 , 55 , 50 , 6 , 54 , 22 , 226 , 119 , 66 , 6 , 38 , 82 , 3 , 96 , 148 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 31 , 15 , 129 , 97 , 43 , 114 , 86 , 91 , 80 , 96 , 0 , 128 , 84 , 96 , 1 , 96 , 255 , 25 , 145 , 130 , 22 , 23 , 144 , 145 , 85 , 96 , 10 , 128 , 84 , 97 , 255 , 255 , 151 , 144 , 151 , 22 , 96 , 1 , 96 , 1 , 96 , 176 , 27 , 3 , 25 , 144 , 151 , 22 , 150 , 144 , 150 , 23 , 98 , 1 , 0 , 0 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 150 , 135 , 22 , 2 , 23 , 144 , 149 , 85 , 96 , 12 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 147 , 144 , 148 , 22 , 146 , 144 , 146 , 23 , 144 , 146 , 85 , 96 , 18 , 145 , 144 , 145 , 85 , 96 , 17 , 128 , 84 , 144 , 146 , 22 , 144 , 21 , 21 , 23 , 144 , 85 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 31 , 167 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 96 , 18 , 85 , 86 , 91 , 96 , 12 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 31 , 214 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 152 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 11 , 84 , 16 , 97 , 31 , 254 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 55 , 245 , 86 , 91 , 96 , 11 , 84 , 97 , 32 , 12 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 129 , 17 , 21 , 97 , 32 , 43 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 144 , 97 , 56 , 97 , 86 , 91 , 96 , 11 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 21 , 97 , 32 , 177 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 47 , 96 , 36 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 84 , 111 , 107 , 101 , 110 , 32 , 115 , 104 , 111 , 117 , 96 , 68 , 130 , 1 , 82 , 110 , 27 , 25 , 8 , 27 , 155 , 221 , 8 , 24 , 153 , 72 , 29 , 152 , 91 , 26 , 89 , 96 , 138 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 13 , 128 , 84 , 96 , 1 , 129 , 1 , 144 , 145 , 85 , 127 , 215 , 182 , 153 , 1 , 5 , 113 , 145 , 1 , 218 , 190 , 183 , 113 , 68 , 242 , 163 , 56 , 92 , 128 , 51 , 172 , 211 , 175 , 151 , 233 , 66 , 58 , 105 , 94 , 129 , 173 , 30 , 181 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 97 , 33 , 116 , 87 , 96 , 14 , 128 , 84 , 96 , 1 , 128 , 130 , 1 , 144 , 146 , 85 , 127 , 187 , 123 , 74 , 69 , 77 , 195 , 73 , 57 , 35 , 72 , 47 , 7 , 130 , 35 , 41 , 237 , 25 , 232 , 36 , 78 , 255 , 88 , 44 , 194 , 4 , 248 , 85 , 76 , 54 , 32 , 195 , 253 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 96 , 0 , 144 , 129 , 82 , 96 , 16 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 145 , 23 , 144 , 85 , 91 , 80 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 15 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 96 , 1 , 23 , 144 , 85 , 86 , 91 , 97 , 33 , 164 , 130 , 130 , 97 , 24 , 230 , 86 , 91 , 97 , 18 , 119 , 87 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 128 , 134 , 82 , 146 , 82 , 128 , 132 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 147 , 23 , 144 , 146 , 85 , 144 , 81 , 51 , 146 , 133 , 145 , 127 , 47 , 135 , 136 , 17 , 126 , 126 , 255 , 29 , 130 , 233 , 38 , 236 , 121 , 73 , 1 , 209 , 124 , 120 , 2 , 74 , 80 , 39 , 9 , 64 , 48 , 69 , 64 , 167 , 51 , 101 , 111 , 13 , 145 , 144 , 164 , 80 , 80 , 86 , 91 , 96 , 0 , 97 , 24 , 223 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 43 , 173 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 130 , 22 , 99 , 121 , 101 , 219 , 11 , 96 , 224 , 27 , 20 , 128 , 97 , 11 , 22 , 87 , 80 , 99 , 1 , 255 , 201 , 167 , 96 , 224 , 27 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 131 , 22 , 20 , 97 , 11 , 22 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 34 , 177 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 36 , 128 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 97 , 112 , 112 , 114 , 111 , 118 , 101 , 32 , 102 , 114 , 111 , 109 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 96 , 68 , 130 , 1 , 82 , 99 , 114 , 101 , 115 , 115 , 96 , 224 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 35 , 18 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 34 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 97 , 112 , 112 , 114 , 111 , 118 , 101 , 32 , 116 , 111 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 96 , 68 , 130 , 1 , 82 , 97 , 115 , 115 , 96 , 240 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 129 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 4 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 148 , 135 , 22 , 128 , 132 , 82 , 148 , 130 , 82 , 145 , 130 , 144 , 32 , 133 , 144 , 85 , 144 , 81 , 132 , 129 , 82 , 127 , 140 , 91 , 225 , 229 , 235 , 236 , 125 , 91 , 209 , 79 , 113 , 66 , 125 , 30 , 132 , 243 , 221 , 3 , 20 , 192 , 247 , 178 , 41 , 30 , 91 , 32 , 10 , 200 , 199 , 195 , 185 , 37 , 145 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 80 , 80 , 80 , 86 , 91 , 96 , 13 , 128 , 84 , 97 , 35 , 131 , 144 , 96 , 1 , 144 , 97 , 61 , 33 , 86 , 91 , 129 , 84 , 129 , 16 , 97 , 35 , 147 , 87 , 97 , 35 , 147 , 97 , 56 , 190 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 144 , 145 , 32 , 1 , 84 , 96 , 13 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 146 , 22 , 145 , 131 , 144 , 129 , 16 , 97 , 35 , 191 , 87 , 97 , 35 , 191 , 97 , 56 , 190 , 86 , 91 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 96 , 0 , 97 , 1 , 0 , 10 , 129 , 84 , 129 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 2 , 25 , 22 , 144 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 2 , 23 , 144 , 85 , 80 , 96 , 13 , 128 , 84 , 128 , 97 , 35 , 254 , 87 , 97 , 35 , 254 , 97 , 61 , 52 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 32 , 144 , 32 , 129 , 1 , 96 , 0 , 25 , 144 , 129 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 144 , 85 , 1 , 144 , 85 , 80 , 86 , 91 , 96 , 0 , 97 , 36 , 47 , 132 , 132 , 97 , 29 , 128 , 86 , 91 , 144 , 80 , 96 , 0 , 25 , 129 , 20 , 97 , 14 , 227 , 87 , 129 , 129 , 16 , 21 , 97 , 36 , 138 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 29 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 105 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 97 , 108 , 108 , 111 , 119 , 97 , 110 , 99 , 101 , 0 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 14 , 227 , 132 , 132 , 132 , 132 , 3 , 97 , 34 , 79 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 36 , 251 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 102 , 114 , 111 , 109 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 96 , 68 , 130 , 1 , 82 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 216 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 37 , 93 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 35 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 116 , 111 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 96 , 68 , 130 , 1 , 82 , 98 , 101 , 115 , 115 , 96 , 232 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 37 , 104 , 131 , 131 , 131 , 97 , 43 , 252 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 129 , 129 , 16 , 21 , 97 , 37 , 224 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 38 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 97 , 109 , 111 , 117 , 110 , 116 , 32 , 101 , 120 , 99 , 101 , 101 , 100 , 115 , 32 , 98 , 96 , 68 , 130 , 1 , 82 , 101 , 97 , 108 , 97 , 110 , 99 , 101 , 96 , 208 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 128 , 133 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 128 , 130 , 32 , 134 , 134 , 3 , 144 , 85 , 146 , 134 , 22 , 128 , 130 , 82 , 144 , 131 , 144 , 32 , 128 , 84 , 134 , 1 , 144 , 85 , 145 , 81 , 127 , 221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239 , 144 , 97 , 38 , 64 , 144 , 134 , 129 , 82 , 96 , 32 , 1 , 144 , 86 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 97 , 14 , 227 , 86 , 91 , 96 , 2 , 96 , 9 , 84 , 3 , 97 , 38 , 159 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 31 , 96 , 36 , 130 , 1 , 82 , 127 , 82 , 101 , 101 , 110 , 116 , 114 , 97 , 110 , 99 , 121 , 71 , 117 , 97 , 114 , 100 , 58 , 32 , 114 , 101 , 101 , 110 , 116 , 114 , 97 , 110 , 116 , 32 , 99 , 97 , 108 , 108 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 2 , 96 , 9 , 85 , 86 , 91 , 97 , 38 , 176 , 132 , 131 , 97 , 41 , 223 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 38 , 250 , 87 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 144 , 131 , 21 , 97 , 8 , 252 , 2 , 144 , 132 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 38 , 244 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 97 , 14 , 227 , 86 , 91 , 97 , 14 , 227 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 130 , 132 , 97 , 44 , 7 , 86 , 91 , 96 , 0 , 96 , 18 , 84 , 48 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 99 , 24 , 22 , 13 , 221 , 96 , 64 , 81 , 129 , 99 , 255 , 255 , 255 , 255 , 22 , 96 , 224 , 27 , 129 , 82 , 96 , 4 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 90 , 250 , 21 , 128 , 21 , 97 , 39 , 81 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 39 , 117 , 145 , 144 , 97 , 57 , 228 , 86 , 91 , 97 , 39 , 127 , 144 , 132 , 97 , 56 , 78 , 86 , 91 , 17 , 21 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 0 , 97 , 39 , 167 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 21 , 97 , 39 , 161 , 87 , 131 , 97 , 24 , 88 , 86 , 91 , 52 , 97 , 24 , 88 , 86 , 91 , 144 , 80 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 21 , 97 , 39 , 201 , 87 , 97 , 39 , 196 , 130 , 133 , 97 , 61 , 33 , 86 , 91 , 97 , 39 , 211 , 86 , 91 , 97 , 39 , 211 , 130 , 52 , 97 , 61 , 33 , 86 , 91 , 144 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 97 , 40 , 40 , 87 , 96 , 10 , 84 , 96 , 64 , 81 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 144 , 97 , 8 , 252 , 132 , 21 , 2 , 144 , 132 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 40 , 34 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 97 , 40 , 96 , 86 , 91 , 97 , 40 , 61 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 135 , 48 , 132 , 97 , 44 , 106 , 86 , 91 , 96 , 10 , 84 , 97 , 40 , 96 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 135 , 129 , 22 , 145 , 137 , 145 , 98 , 1 , 0 , 0 , 144 , 145 , 4 , 22 , 133 , 97 , 44 , 106 , 86 , 91 , 97 , 40 , 106 , 131 , 130 , 97 , 41 , 18 , 86 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 97 , 20 , 255 , 129 , 51 , 97 , 44 , 162 , 86 , 91 , 97 , 40 , 134 , 130 , 130 , 97 , 33 , 154 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 97 , 16 , 120 , 144 , 130 , 97 , 34 , 5 , 86 , 91 , 97 , 40 , 168 , 130 , 130 , 97 , 44 , 251 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 2 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 97 , 16 , 120 , 144 , 130 , 97 , 45 , 98 , 86 , 91 , 97 , 40 , 200 , 97 , 45 , 119 , 86 , 91 , 96 , 8 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 127 , 93 , 185 , 238 , 10 , 73 , 91 , 242 , 230 , 255 , 156 , 145 , 167 , 131 , 76 , 27 , 164 , 253 , 210 , 68 , 165 , 232 , 170 , 78 , 83 , 123 , 211 , 138 , 234 , 228 , 176 , 115 , 170 , 51 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 161 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 41 , 104 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 31 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 109 , 105 , 110 , 116 , 32 , 116 , 111 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 41 , 116 , 96 , 0 , 131 , 131 , 97 , 43 , 252 , 86 , 91 , 128 , 96 , 5 , 96 , 0 , 130 , 130 , 84 , 97 , 41 , 134 , 145 , 144 , 97 , 56 , 78 , 86 , 91 , 144 , 145 , 85 , 80 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 3 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 128 , 84 , 134 , 1 , 144 , 85 , 81 , 132 , 129 , 82 , 127 , 221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239 , 145 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 80 , 80 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 97 , 42 , 63 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 33 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 98 , 117 , 114 , 110 , 32 , 102 , 114 , 111 , 109 , 32 , 116 , 104 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 96 , 68 , 130 , 1 , 82 , 96 , 115 , 96 , 248 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 97 , 42 , 75 , 130 , 96 , 0 , 131 , 97 , 43 , 252 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 130 , 22 , 96 , 0 , 144 , 129 , 82 , 96 , 3 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 129 , 129 , 16 , 21 , 97 , 42 , 191 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 34 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 98 , 117 , 114 , 110 , 32 , 97 , 109 , 111 , 117 , 110 , 116 , 32 , 101 , 120 , 99 , 101 , 101 , 100 , 115 , 32 , 98 , 97 , 108 , 97 , 110 , 96 , 68 , 130 , 1 , 82 , 97 , 99 , 101 , 96 , 240 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 0 , 129 , 129 , 82 , 96 , 3 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 134 , 134 , 3 , 144 , 85 , 96 , 5 , 128 , 84 , 135 , 144 , 3 , 144 , 85 , 81 , 133 , 129 , 82 , 145 , 146 , 145 , 127 , 221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239 , 145 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 163 , 80 , 80 , 80 , 86 , 91 , 97 , 43 , 39 , 97 , 45 , 192 , 86 , 91 , 96 , 8 , 128 , 84 , 96 , 255 , 25 , 22 , 96 , 1 , 23 , 144 , 85 , 127 , 98 , 231 , 140 , 234 , 1 , 190 , 227 , 32 , 205 , 78 , 66 , 2 , 112 , 181 , 234 , 116 , 0 , 13 , 17 , 176 , 201 , 247 , 71 , 84 , 235 , 219 , 252 , 84 , 75 , 5 , 162 , 88 , 97 , 40 , 245 , 51 , 144 , 86 , 91 , 96 , 0 , 97 , 24 , 223 , 131 , 131 , 97 , 46 , 6 , 86 , 91 , 96 , 0 , 97 , 11 , 22 , 130 , 84 , 144 , 86 , 91 , 97 , 43 , 138 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 70 , 131 , 57 , 129 , 81 , 145 , 82 , 130 , 97 , 46 , 48 , 86 , 91 , 97 , 43 , 149 , 96 , 0 , 130 , 97 , 46 , 48 , 86 , 91 , 97 , 20 , 255 , 96 , 0 , 128 , 81 , 96 , 32 , 97 , 62 , 38 , 131 , 57 , 129 , 81 , 145 , 82 , 130 , 97 , 46 , 48 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 1 , 131 , 1 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 84 , 97 , 43 , 244 , 87 , 80 , 129 , 84 , 96 , 1 , 129 , 129 , 1 , 132 , 85 , 96 , 0 , 132 , 129 , 82 , 96 , 32 , 128 , 130 , 32 , 144 , 147 , 1 , 132 , 144 , 85 , 132 , 84 , 132 , 130 , 82 , 130 , 134 , 1 , 144 , 147 , 82 , 96 , 64 , 144 , 32 , 145 , 144 , 145 , 85 , 97 , 11 , 22 , 86 , 91 , 80 , 96 , 0 , 97 , 11 , 22 , 86 , 91 , 97 , 16 , 120 , 131 , 131 , 131 , 97 , 46 , 58 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 36 , 130 , 1 , 82 , 96 , 68 , 129 , 1 , 130 , 144 , 82 , 97 , 16 , 120 , 144 , 132 , 144 , 99 , 169 , 5 , 156 , 187 , 96 , 224 , 27 , 144 , 96 , 100 , 1 , 91 , 96 , 64 , 128 , 81 , 96 , 31 , 25 , 129 , 132 , 3 , 1 , 129 , 82 , 145 , 144 , 82 , 96 , 32 , 129 , 1 , 128 , 81 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 22 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 144 , 147 , 22 , 146 , 144 , 146 , 23 , 144 , 145 , 82 , 97 , 46 , 160 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 128 , 133 , 22 , 96 , 36 , 131 , 1 , 82 , 131 , 22 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 129 , 1 , 130 , 144 , 82 , 97 , 14 , 227 , 144 , 133 , 144 , 99 , 35 , 184 , 114 , 221 , 96 , 224 , 27 , 144 , 96 , 132 , 1 , 97 , 44 , 51 , 86 , 91 , 97 , 44 , 172 , 130 , 130 , 97 , 24 , 230 , 86 , 91 , 97 , 18 , 119 , 87 , 97 , 44 , 185 , 129 , 97 , 47 , 114 , 86 , 91 , 97 , 44 , 196 , 131 , 96 , 32 , 97 , 47 , 132 , 86 , 91 , 96 , 64 , 81 , 96 , 32 , 1 , 97 , 44 , 213 , 146 , 145 , 144 , 97 , 61 , 74 , 86 , 91 , 96 , 64 , 128 , 81 , 96 , 31 , 25 , 129 , 132 , 3 , 1 , 129 , 82 , 144 , 130 , 144 , 82 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 130 , 82 , 97 , 11 , 216 , 145 , 96 , 4 , 1 , 97 , 51 , 248 , 86 , 91 , 97 , 45 , 5 , 130 , 130 , 97 , 24 , 230 , 86 , 91 , 21 , 97 , 18 , 119 , 87 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 144 , 129 , 82 , 96 , 64 , 128 , 131 , 32 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 128 , 133 , 82 , 146 , 82 , 128 , 131 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 85 , 81 , 51 , 146 , 133 , 145 , 127 , 246 , 57 , 31 , 92 , 50 , 217 , 198 , 157 , 42 , 71 , 234 , 103 , 11 , 68 , 41 , 116 , 181 , 57 , 53 , 209 , 237 , 199 , 253 , 100 , 235 , 33 , 224 , 71 , 168 , 57 , 23 , 27 , 145 , 144 , 164 , 80 , 80 , 86 , 91 , 96 , 0 , 97 , 24 , 223 , 131 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 49 , 32 , 86 , 91 , 96 , 8 , 84 , 96 , 255 , 22 , 97 , 20 , 102 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 20 , 96 , 36 , 130 , 1 , 82 , 115 , 20 , 24 , 93 , 92 , 216 , 88 , 155 , 25 , 78 , 136 , 27 , 155 , 221 , 8 , 28 , 24 , 93 , 92 , 217 , 89 , 96 , 98 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 8 , 84 , 96 , 255 , 22 , 21 , 97 , 20 , 102 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 16 , 96 , 36 , 130 , 1 , 82 , 111 , 20 , 24 , 93 , 92 , 216 , 88 , 155 , 25 , 78 , 136 , 28 , 24 , 93 , 92 , 217 , 89 , 96 , 130 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 130 , 96 , 0 , 1 , 130 , 129 , 84 , 129 , 16 , 97 , 46 , 29 , 87 , 97 , 46 , 29 , 97 , 56 , 190 , 86 , 91 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 84 , 144 , 80 , 146 , 145 , 80 , 80 , 86 , 91 , 97 , 18 , 119 , 130 , 130 , 97 , 40 , 124 , 86 , 91 , 96 , 8 , 84 , 96 , 255 , 22 , 21 , 97 , 16 , 120 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 42 , 96 , 36 , 130 , 1 , 82 , 127 , 69 , 82 , 67 , 50 , 48 , 80 , 97 , 117 , 115 , 97 , 98 , 108 , 101 , 58 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 116 , 114 , 97 , 110 , 115 , 102 , 101 , 114 , 32 , 119 , 104 , 96 , 68 , 130 , 1 , 82 , 105 , 26 , 91 , 25 , 72 , 28 , 24 , 93 , 92 , 217 , 89 , 96 , 178 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 97 , 46 , 245 , 130 , 96 , 64 , 81 , 128 , 96 , 64 , 1 , 96 , 64 , 82 , 128 , 96 , 32 , 129 , 82 , 96 , 32 , 1 , 127 , 83 , 97 , 102 , 101 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 108 , 111 , 119 , 45 , 108 , 101 , 118 , 101 , 108 , 32 , 99 , 97 , 108 , 108 , 32 , 102 , 97 , 105 , 108 , 101 , 100 , 129 , 82 , 80 , 133 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 97 , 50 , 26 , 144 , 146 , 145 , 144 , 99 , 255 , 255 , 255 , 255 , 22 , 86 , 91 , 128 , 81 , 144 , 145 , 80 , 21 , 97 , 16 , 120 , 87 , 128 , 128 , 96 , 32 , 1 , 144 , 81 , 129 , 1 , 144 , 97 , 47 , 19 , 145 , 144 , 97 , 61 , 191 , 86 , 91 , 97 , 16 , 120 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 42 , 96 , 36 , 130 , 1 , 82 , 127 , 83 , 97 , 102 , 101 , 69 , 82 , 67 , 50 , 48 , 58 , 32 , 69 , 82 , 67 , 50 , 48 , 32 , 111 , 112 , 101 , 114 , 97 , 116 , 105 , 111 , 110 , 32 , 100 , 105 , 100 , 32 , 110 , 96 , 68 , 130 , 1 , 82 , 105 , 27 , 221 , 8 , 28 , 221 , 88 , 216 , 217 , 89 , 89 , 96 , 178 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 96 , 97 , 11 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 96 , 20 , 91 , 96 , 96 , 96 , 0 , 97 , 47 , 147 , 131 , 96 , 2 , 97 , 60 , 125 , 86 , 91 , 97 , 47 , 158 , 144 , 96 , 2 , 97 , 56 , 78 , 86 , 91 , 103 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 129 , 17 , 21 , 97 , 47 , 182 , 87 , 97 , 47 , 182 , 97 , 61 , 220 , 86 , 91 , 96 , 64 , 81 , 144 , 128 , 130 , 82 , 128 , 96 , 31 , 1 , 96 , 31 , 25 , 22 , 96 , 32 , 1 , 130 , 1 , 96 , 64 , 82 , 128 , 21 , 97 , 47 , 224 , 87 , 96 , 32 , 130 , 1 , 129 , 128 , 54 , 131 , 55 , 1 , 144 , 80 , 91 , 80 , 144 , 80 , 96 , 3 , 96 , 252 , 27 , 129 , 96 , 0 , 129 , 81 , 129 , 16 , 97 , 47 , 251 , 87 , 97 , 47 , 251 , 97 , 56 , 190 , 86 , 91 , 96 , 32 , 1 , 1 , 144 , 96 , 1 , 96 , 1 , 96 , 248 , 27 , 3 , 25 , 22 , 144 , 129 , 96 , 0 , 26 , 144 , 83 , 80 , 96 , 15 , 96 , 251 , 27 , 129 , 96 , 1 , 129 , 81 , 129 , 16 , 97 , 48 , 42 , 87 , 97 , 48 , 42 , 97 , 56 , 190 , 86 , 91 , 96 , 32 , 1 , 1 , 144 , 96 , 1 , 96 , 1 , 96 , 248 , 27 , 3 , 25 , 22 , 144 , 129 , 96 , 0 , 26 , 144 , 83 , 80 , 96 , 0 , 97 , 48 , 78 , 132 , 96 , 2 , 97 , 60 , 125 , 86 , 91 , 97 , 48 , 89 , 144 , 96 , 1 , 97 , 56 , 78 , 86 , 91 , 144 , 80 , 91 , 96 , 1 , 129 , 17 , 21 , 97 , 48 , 209 , 87 , 111 , 24 , 24 , 153 , 25 , 154 , 26 , 155 , 27 , 156 , 28 , 176 , 177 , 49 , 178 , 50 , 179 , 96 , 129 , 27 , 133 , 96 , 15 , 22 , 96 , 16 , 129 , 16 , 97 , 48 , 141 , 87 , 97 , 48 , 141 , 97 , 56 , 190 , 86 , 91 , 26 , 96 , 248 , 27 , 130 , 130 , 129 , 81 , 129 , 16 , 97 , 48 , 163 , 87 , 97 , 48 , 163 , 97 , 56 , 190 , 86 , 91 , 96 , 32 , 1 , 1 , 144 , 96 , 1 , 96 , 1 , 96 , 248 , 27 , 3 , 25 , 22 , 144 , 129 , 96 , 0 , 26 , 144 , 83 , 80 , 96 , 4 , 148 , 144 , 148 , 28 , 147 , 97 , 48 , 202 , 129 , 97 , 61 , 242 , 86 , 91 , 144 , 80 , 97 , 48 , 92 , 86 , 91 , 80 , 131 , 21 , 97 , 24 , 223 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 129 , 144 , 82 , 96 , 36 , 130 , 1 , 82 , 127 , 83 , 116 , 114 , 105 , 110 , 103 , 115 , 58 , 32 , 104 , 101 , 120 , 32 , 108 , 101 , 110 , 103 , 116 , 104 , 32 , 105 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 129 , 129 , 82 , 96 , 1 , 131 , 1 , 96 , 32 , 82 , 96 , 64 , 129 , 32 , 84 , 128 , 21 , 97 , 50 , 9 , 87 , 96 , 0 , 97 , 49 , 68 , 96 , 1 , 131 , 97 , 61 , 33 , 86 , 91 , 133 , 84 , 144 , 145 , 80 , 96 , 0 , 144 , 97 , 49 , 88 , 144 , 96 , 1 , 144 , 97 , 61 , 33 , 86 , 91 , 144 , 80 , 129 , 129 , 20 , 97 , 49 , 189 , 87 , 96 , 0 , 134 , 96 , 0 , 1 , 130 , 129 , 84 , 129 , 16 , 97 , 49 , 120 , 87 , 97 , 49 , 120 , 97 , 56 , 190 , 86 , 91 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 84 , 144 , 80 , 128 , 135 , 96 , 0 , 1 , 132 , 129 , 84 , 129 , 16 , 97 , 49 , 155 , 87 , 97 , 49 , 155 , 97 , 56 , 190 , 86 , 91 , 96 , 0 , 145 , 130 , 82 , 96 , 32 , 128 , 131 , 32 , 144 , 145 , 1 , 146 , 144 , 146 , 85 , 145 , 130 , 82 , 96 , 1 , 136 , 1 , 144 , 82 , 96 , 64 , 144 , 32 , 131 , 144 , 85 , 91 , 133 , 84 , 134 , 144 , 128 , 97 , 49 , 206 , 87 , 97 , 49 , 206 , 97 , 61 , 52 , 86 , 91 , 96 , 1 , 144 , 3 , 129 , 129 , 144 , 96 , 0 , 82 , 96 , 32 , 96 , 0 , 32 , 1 , 96 , 0 , 144 , 85 , 144 , 85 , 133 , 96 , 1 , 1 , 96 , 0 , 134 , 129 , 82 , 96 , 32 , 1 , 144 , 129 , 82 , 96 , 32 , 1 , 96 , 0 , 32 , 96 , 0 , 144 , 85 , 96 , 1 , 147 , 80 , 80 , 80 , 80 , 97 , 11 , 22 , 86 , 91 , 96 , 0 , 145 , 80 , 80 , 97 , 11 , 22 , 86 , 91 , 80 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 96 , 97 , 50 , 41 , 132 , 132 , 96 , 0 , 133 , 97 , 50 , 49 , 86 , 91 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 96 , 130 , 71 , 16 , 21 , 97 , 50 , 146 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 38 , 96 , 36 , 130 , 1 , 82 , 127 , 65 , 100 , 100 , 114 , 101 , 115 , 115 , 58 , 32 , 105 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 98 , 97 , 108 , 97 , 110 , 99 , 101 , 32 , 102 , 111 , 96 , 68 , 130 , 1 , 82 , 101 , 28 , 136 , 24 , 216 , 91 , 27 , 96 , 210 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 11 , 216 , 86 , 91 , 96 , 0 , 128 , 134 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 133 , 135 , 96 , 64 , 81 , 97 , 50 , 174 , 145 , 144 , 97 , 62 , 9 , 86 , 91 , 96 , 0 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 133 , 135 , 90 , 241 , 146 , 80 , 80 , 80 , 61 , 128 , 96 , 0 , 129 , 20 , 97 , 50 , 235 , 87 , 96 , 64 , 81 , 145 , 80 , 96 , 31 , 25 , 96 , 63 , 61 , 1 , 22 , 130 , 1 , 96 , 64 , 82 , 61 , 130 , 82 , 61 , 96 , 0 , 96 , 32 , 132 , 1 , 62 , 97 , 50 , 240 , 86 , 91 , 96 , 96 , 145 , 80 , 91 , 80 , 145 , 80 , 145 , 80 , 97 , 51 , 1 , 135 , 131 , 131 , 135 , 97 , 51 , 12 , 86 , 91 , 151 , 150 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 96 , 131 , 21 , 97 , 51 , 123 , 87 , 130 , 81 , 96 , 0 , 3 , 97 , 51 , 116 , 87 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 59 , 97 , 51 , 116 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 29 , 96 , 36 , 130 , 1 , 82 , 127 , 65 , 100 , 100 , 114 , 101 , 115 , 115 , 58 , 32 , 99 , 97 , 108 , 108 , 32 , 116 , 111 , 32 , 110 , 111 , 110 , 45 , 99 , 111 , 110 , 116 , 114 , 97 , 99 , 116 , 0 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 11 , 216 , 86 , 91 , 80 , 129 , 97 , 50 , 41 , 86 , 91 , 97 , 50 , 41 , 131 , 131 , 129 , 81 , 21 , 97 , 51 , 144 , 87 , 129 , 81 , 128 , 131 , 96 , 32 , 1 , 253 , 91 , 128 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 11 , 216 , 145 , 144 , 97 , 51 , 248 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 51 , 188 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 129 , 22 , 129 , 20 , 97 , 24 , 223 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 91 , 131 , 129 , 16 , 21 , 97 , 51 , 239 , 87 , 129 , 129 , 1 , 81 , 131 , 130 , 1 , 82 , 96 , 32 , 1 , 97 , 51 , 215 , 86 , 91 , 80 , 80 , 96 , 0 , 145 , 1 , 82 , 86 , 91 , 96 , 32 , 129 , 82 , 96 , 0 , 130 , 81 , 128 , 96 , 32 , 132 , 1 , 82 , 97 , 52 , 23 , 129 , 96 , 64 , 133 , 1 , 96 , 32 , 135 , 1 , 97 , 51 , 212 , 86 , 91 , 96 , 31 , 1 , 96 , 31 , 25 , 22 , 145 , 144 , 145 , 1 , 96 , 64 , 1 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 20 , 255 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 53 , 99 , 255 , 255 , 255 , 255 , 129 , 22 , 129 , 20 , 97 , 28 , 54 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 52 , 103 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 52 , 114 , 129 , 97 , 52 , 43 , 86 , 91 , 145 , 80 , 97 , 52 , 128 , 96 , 32 , 132 , 1 , 97 , 52 , 64 , 86 , 91 , 144 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 52 , 156 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 52 , 167 , 129 , 97 , 52 , 43 , 86 , 91 , 148 , 96 , 32 , 147 , 144 , 147 , 1 , 53 , 147 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 96 , 132 , 134 , 3 , 18 , 21 , 97 , 52 , 202 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 53 , 97 , 52 , 213 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 80 , 96 , 32 , 132 , 1 , 53 , 97 , 52 , 229 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 149 , 146 , 148 , 80 , 80 , 80 , 96 , 64 , 145 , 144 , 145 , 1 , 53 , 144 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 53 , 8 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 53 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 53 , 34 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 145 , 80 , 96 , 32 , 131 , 1 , 53 , 97 , 53 , 52 , 129 , 97 , 52 , 43 , 86 , 91 , 128 , 145 , 80 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 96 , 132 , 134 , 3 , 18 , 21 , 97 , 53 , 84 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 53 , 97 , 53 , 95 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 80 , 96 , 32 , 132 , 1 , 53 , 145 , 80 , 96 , 64 , 132 , 1 , 53 , 97 , 53 , 118 , 129 , 97 , 52 , 43 , 86 , 91 , 128 , 145 , 80 , 80 , 146 , 80 , 146 , 80 , 146 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 53 , 147 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 97 , 24 , 223 , 129 , 97 , 52 , 43 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 128 , 133 , 135 , 3 , 18 , 21 , 97 , 53 , 180 , 87 , 96 , 0 , 128 , 253 , 91 , 132 , 53 , 97 , 53 , 191 , 129 , 97 , 52 , 43 , 86 , 91 , 147 , 80 , 96 , 32 , 133 , 1 , 53 , 97 , 53 , 207 , 129 , 97 , 52 , 43 , 86 , 91 , 146 , 80 , 96 , 64 , 133 , 1 , 53 , 145 , 80 , 96 , 96 , 133 , 1 , 53 , 97 , 53 , 230 , 129 , 97 , 52 , 43 , 86 , 91 , 147 , 150 , 146 , 149 , 80 , 144 , 147 , 80 , 80 , 86 , 91 , 128 , 21 , 21 , 129 , 20 , 97 , 20 , 255 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 54 , 17 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 97 , 24 , 223 , 129 , 97 , 53 , 241 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 54 , 47 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 80 , 128 , 53 , 146 , 96 , 32 , 144 , 145 , 1 , 53 , 145 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 130 , 81 , 130 , 130 , 1 , 129 , 144 , 82 , 96 , 0 , 145 , 144 , 132 , 130 , 1 , 144 , 96 , 64 , 133 , 1 , 144 , 132 , 91 , 129 , 129 , 16 , 21 , 97 , 54 , 127 , 87 , 131 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 131 , 82 , 146 , 132 , 1 , 146 , 145 , 132 , 1 , 145 , 96 , 1 , 1 , 97 , 54 , 90 , 86 , 91 , 80 , 144 , 150 , 149 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 128 , 53 , 97 , 255 , 255 , 129 , 22 , 129 , 20 , 97 , 28 , 54 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 54 , 176 , 87 , 96 , 0 , 128 , 253 , 91 , 97 , 52 , 114 , 131 , 97 , 54 , 139 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 54 , 204 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 54 , 215 , 129 , 97 , 52 , 43 , 86 , 91 , 145 , 80 , 96 , 32 , 131 , 1 , 53 , 97 , 53 , 52 , 129 , 97 , 52 , 43 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 192 , 135 , 137 , 3 , 18 , 21 , 97 , 55 , 0 , 87 , 96 , 0 , 128 , 253 , 91 , 97 , 55 , 9 , 135 , 97 , 54 , 139 , 86 , 91 , 149 , 80 , 96 , 32 , 135 , 1 , 53 , 97 , 55 , 25 , 129 , 97 , 52 , 43 , 86 , 91 , 148 , 80 , 96 , 64 , 135 , 1 , 53 , 97 , 55 , 41 , 129 , 97 , 52 , 43 , 86 , 91 , 147 , 80 , 96 , 96 , 135 , 1 , 53 , 146 , 80 , 96 , 128 , 135 , 1 , 53 , 97 , 55 , 64 , 129 , 97 , 53 , 241 , 86 , 91 , 145 , 80 , 96 , 160 , 135 , 1 , 53 , 97 , 55 , 80 , 129 , 97 , 52 , 43 , 86 , 91 , 128 , 145 , 80 , 80 , 146 , 149 , 80 , 146 , 149 , 80 , 146 , 149 , 86 , 91 , 96 , 1 , 129 , 129 , 28 , 144 , 130 , 22 , 128 , 97 , 55 , 114 , 87 , 96 , 127 , 130 , 22 , 145 , 80 , 91 , 96 , 32 , 130 , 16 , 129 , 3 , 97 , 55 , 146 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 34 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 145 , 144 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 57 , 144 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 79 , 110 , 108 , 121 , 32 , 104 , 97 , 110 , 100 , 108 , 96 , 64 , 130 , 1 , 82 , 127 , 101 , 114 , 32 , 99 , 97 , 110 , 32 , 99 , 97 , 108 , 108 , 32 , 116 , 104 , 105 , 115 , 32 , 102 , 117 , 110 , 99 , 116 , 105 , 111 , 110 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 110 , 111 , 96 , 64 , 130 , 1 , 82 , 98 , 110 , 99 , 101 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 17 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 128 , 130 , 1 , 128 , 130 , 17 , 21 , 97 , 11 , 22 , 87 , 97 , 11 , 22 , 97 , 56 , 56 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 58 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 78 , 111 , 110 , 99 , 101 , 32 , 109 , 117 , 115 , 116 , 96 , 64 , 130 , 1 , 82 , 127 , 32 , 110 , 111 , 116 , 32 , 105 , 110 , 99 , 114 , 101 , 109 , 101 , 110 , 116 , 32 , 109 , 111 , 114 , 101 , 32 , 116 , 104 , 97 , 110 , 32 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 50 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 96 , 0 , 96 , 1 , 130 , 1 , 97 , 56 , 230 , 87 , 97 , 56 , 230 , 97 , 56 , 56 , 86 , 91 , 80 , 96 , 1 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 109 , 117 , 115 , 116 , 32 , 104 , 97 , 118 , 101 , 32 , 109 , 105 , 110 , 116 , 101 , 114 , 32 , 114 , 96 , 64 , 130 , 1 , 82 , 98 , 111 , 108 , 101 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 41 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 110 , 97 , 116 , 105 , 118 , 96 , 64 , 130 , 1 , 82 , 104 , 101 , 32 , 98 , 97 , 108 , 97 , 110 , 99 , 101 , 96 , 184 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 69 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 78 , 97 , 116 , 105 , 118 , 101 , 32 , 117 , 110 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 32 , 96 , 64 , 130 , 1 , 82 , 127 , 105 , 115 , 32 , 110 , 111 , 116 , 32 , 97 , 108 , 108 , 111 , 119 , 101 , 100 , 32 , 102 , 111 , 114 , 32 , 116 , 104 , 105 , 115 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 119 , 114 , 96 , 96 , 130 , 1 , 82 , 100 , 48 , 184 , 56 , 50 , 185 , 96 , 217 , 27 , 96 , 128 , 130 , 1 , 82 , 96 , 160 , 1 , 144 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 57 , 246 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 81 , 145 , 144 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 40 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 115 , 117 , 102 , 102 , 105 , 99 , 105 , 101 , 110 , 116 , 32 , 69 , 82 , 67 , 50 , 48 , 96 , 64 , 130 , 1 , 82 , 103 , 32 , 98 , 97 , 108 , 97 , 110 , 99 , 101 , 96 , 192 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 46 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 104 , 105 , 115 , 116 , 111 , 114 , 105 , 99 , 97 , 108 , 96 , 64 , 130 , 1 , 82 , 109 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 144 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 57 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 97 , 109 , 111 , 117 , 110 , 116 , 32 , 112 , 114 , 111 , 96 , 64 , 130 , 1 , 82 , 127 , 118 , 105 , 100 , 101 , 100 , 32 , 102 , 111 , 114 , 32 , 110 , 97 , 116 , 105 , 118 , 101 , 32 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 67 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 78 , 97 , 116 , 105 , 118 , 101 , 32 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 32 , 105 , 115 , 96 , 64 , 130 , 1 , 82 , 127 , 32 , 110 , 111 , 116 , 32 , 97 , 108 , 108 , 111 , 119 , 101 , 100 , 32 , 102 , 111 , 114 , 32 , 116 , 104 , 105 , 115 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 119 , 114 , 97 , 112 , 96 , 96 , 130 , 1 , 82 , 98 , 56 , 50 , 185 , 96 , 233 , 27 , 96 , 128 , 130 , 1 , 82 , 96 , 160 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 45 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 118 , 97 , 108 , 117 , 101 , 32 , 115 , 101 , 110 , 116 , 96 , 64 , 130 , 1 , 82 , 108 , 32 , 102 , 111 , 114 , 32 , 119 , 114 , 97 , 112 , 112 , 105 , 110 , 103 , 96 , 152 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 97 , 100 , 100 , 114 , 96 , 64 , 130 , 1 , 82 , 98 , 101 , 115 , 115 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 50 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 70 , 101 , 101 , 32 , 82 , 101 , 99 , 105 , 112 , 105 , 101 , 110 , 116 , 32 , 99 , 97 , 110 , 110 , 96 , 64 , 130 , 1 , 82 , 113 , 111 , 116 , 32 , 98 , 101 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 112 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 34 , 144 , 130 , 1 , 82 , 127 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 116 , 111 , 107 , 101 , 110 , 32 , 97 , 109 , 111 , 117 , 96 , 64 , 130 , 1 , 82 , 97 , 27 , 157 , 96 , 242 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 128 , 130 , 2 , 129 , 21 , 130 , 130 , 4 , 132 , 20 , 23 , 97 , 11 , 22 , 87 , 97 , 11 , 22 , 97 , 56 , 56 , 86 , 91 , 96 , 0 , 130 , 97 , 60 , 177 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 18 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 4 , 144 , 86 , 91 , 97 , 255 , 255 , 130 , 129 , 22 , 130 , 130 , 22 , 3 , 144 , 128 , 130 , 17 , 21 , 97 , 50 , 19 , 87 , 97 , 50 , 19 , 97 , 56 , 56 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 48 , 144 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 103 , 105 , 98 , 108 , 101 , 84 , 111 , 107 , 101 , 110 , 87 , 114 , 97 , 112 , 112 , 101 , 114 , 58 , 32 , 72 , 97 , 110 , 100 , 108 , 101 , 114 , 32 , 65 , 100 , 96 , 64 , 130 , 1 , 82 , 111 , 6 , 71 , 38 , 87 , 55 , 50 , 6 , 54 , 22 , 226 , 119 , 66 , 6 , 38 , 82 , 3 , 96 , 132 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 129 , 129 , 3 , 129 , 129 , 17 , 21 , 97 , 11 , 22 , 87 , 97 , 11 , 22 , 97 , 56 , 56 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 49 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 127 , 65 , 99 , 99 , 101 , 115 , 115 , 67 , 111 , 110 , 116 , 114 , 111 , 108 , 58 , 32 , 97 , 99 , 99 , 111 , 117 , 110 , 116 , 32 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 129 , 82 , 96 , 0 , 131 , 81 , 97 , 61 , 130 , 129 , 96 , 23 , 133 , 1 , 96 , 32 , 136 , 1 , 97 , 51 , 212 , 86 , 91 , 112 , 1 , 3 , 75 , 153 , 3 , 107 , 75 , 155 , 155 , 75 , 115 , 57 , 3 , 147 , 123 , 99 , 41 , 96 , 125 , 27 , 96 , 23 , 145 , 132 , 1 , 145 , 130 , 1 , 82 , 131 , 81 , 97 , 61 , 179 , 129 , 96 , 40 , 132 , 1 , 96 , 32 , 136 , 1 , 97 , 51 , 212 , 86 , 91 , 1 , 96 , 40 , 1 , 148 , 147 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 61 , 209 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 97 , 24 , 223 , 129 , 97 , 53 , 241 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 65 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 96 , 0 , 129 , 97 , 62 , 1 , 87 , 97 , 62 , 1 , 97 , 56 , 56 , 86 , 91 , 80 , 96 , 0 , 25 , 1 , 144 , 86 , 91 , 96 , 0 , 130 , 81 , 97 , 62 , 27 , 129 , 132 , 96 , 32 , 135 , 1 , 97 , 51 , 212 , 86 , 91 , 145 , 144 , 145 , 1 , 146 , 145 , 80 , 80 , 86 , 254 , 101 , 215 , 162 , 142 , 50 , 101 , 179 , 122 , 100 , 116 , 146 , 159 , 51 , 101 , 33 , 179 , 50 , 193 , 104 , 27 , 147 , 63 , 108 , 185 , 243 , 55 , 102 , 115 , 68 , 13 , 134 , 42 , 159 , 45 , 240 , 254 , 210 , 199 , 118 , 72 , 222 , 88 , 96 , 164 , 204 , 80 , 140 , 208 , 129 , 140 , 133 , 184 , 184 , 161 , 171 , 76 , 238 , 239 , 141 , 152 , 28 , 137 , 86 , 166 , 162 , 100 , 105 , 112 , 102 , 115 , 88 , 34 , 18 , 32 , 178 , 146 , 213 , 143 , 104 , 179 , 51 , 146 , 89 , 134 , 143 , 195 , 133 , 98 , 181 , 91 , 159 , 88 , 25 , 143 , 97 , 73 , 62 , 116 , 56 , 5 , 135 , 103 , 39 , 145 , 124 , 199 , 100 , 115 , 111 , 108 , 99 , 67 , 0 , 8 , 18 , 0 , 51] ;
    #[doc = "The deployed bytecode of the contract."]
    pub static FUNGIBLETOKENWRAPPERCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FungibleTokenWrapperContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FungibleTokenWrapperContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FungibleTokenWrapperContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FungibleTokenWrapperContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FungibleTokenWrapperContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(FungibleTokenWrapperContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FungibleTokenWrapperContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers` client at"]
        #[doc = r" `address`. The contract derefs to a `ethers::Contract` object."]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FUNGIBLETOKENWRAPPERCONTRACT_ABI.clone(),
                client,
            ))
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" - If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" - The default poll duration is 7 seconds."]
        #[doc = r" - The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter, "../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                FUNGIBLETOKENWRAPPERCONTRACT_ABI.clone(),
                FUNGIBLETOKENWRAPPERCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINTER_ROLE` (0xd5391393) function"]
        pub fn minter_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 57, 19, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PAUSER_ROLE` (0xe63ab1e9) function"]
        pub fn pauser_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 58, 177, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `add` (0xfc97a652) function"]
        pub fn add(
            &self,
            token_address: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 151, 166, 82], (token_address, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x42966c68) function"]
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnFrom` (0x79cc6790) function"]
        pub fn burn_from(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 204, 103, 144], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feePercentage` (0xa001ecdd) function"]
        pub fn fee_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([160, 1, 236, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeRecipient` (0x46904840) function"]
        pub fn fee_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 144, 72, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountToWrap` (0x96cd4dfe) function"]
        pub fn get_amount_to_wrap(
            &self,
            deposit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([150, 205, 77, 254], deposit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFee` (0xced72f87) function"]
        pub fn get_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([206, 215, 47, 135], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFeeFromAmount` (0x85c00ae8) function"]
        pub fn get_fee_from_amount(
            &self,
            amount_to_wrap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([133, 192, 10, 232], amount_to_wrap)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProposalNonce` (0x0b27fb9a) function"]
        pub fn get_proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleMember` (0x9010d07c) function"]
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleMemberCount` (0xca15c873) function"]
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokens` (0xaa6ca808) function"]
        pub fn get_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([170, 108, 168, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handler` (0xc80916d4) function"]
        pub fn handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 9, 22, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `historicalTokens` (0x85d14834) function"]
        pub fn historical_tokens(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([133, 209, 72, 52], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `historicallyValid` (0xb1cba258) function"]
        pub fn historically_valid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 203, 162, 88], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xf63ebb45) function"]
        pub fn initialize(
            &self,
            fee_percentage: u16,
            fee_recipient: ::ethers::core::types::Address,
            handler: ::ethers::core::types::Address,
            limit: ::ethers::core::types::U256,
            is_native_allowed: bool,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [246, 62, 187, 69],
                    (
                        fee_percentage,
                        fee_recipient,
                        handler,
                        limit,
                        is_native_allowed,
                        admin,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialized` (0x158ef93e) function"]
        pub fn initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isNativeAllowed` (0xb3e4083f) function"]
        pub fn is_native_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 228, 8, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidToken` (0xc1876453) function"]
        pub fn is_valid_token(
            &self,
            token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([193, 135, 100, 83], token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposalNonce` (0xcc3c74a1) function"]
        pub fn proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove` (0x1c4a1436) function"]
        pub fn remove(
            &self,
            token_address: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 74, 20, 54], (token_address, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFee` (0xc2ae4720) function"]
        pub fn set_fee(
            &self,
            fee_percentage: u16,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 174, 71, 32], (fee_percentage, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeRecipient` (0x07184f1c) function"]
        pub fn set_fee_recipient(
            &self,
            fee_recipient: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 24, 79, 28], (fee_recipient, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0xbac426d0) function"]
        pub fn set_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 196, 38, 208], handler)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNativeAllowed` (0x8b5478b9) function"]
        pub fn set_native_allowed(
            &self,
            is_native_allowed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 84, 120, 185], is_native_allowed)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String>
        {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokens` (0x4f64b2be) function"]
        pub fn tokens(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([79, 100, 178, 190], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrap` (0x39f47693) function"]
        pub fn unwrap(
            &self,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 244, 118, 147], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapAndSendTo` (0x4808285e) function"]
        pub fn unwrap_and_send_to(
            &self,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [72, 8, 40, 94],
                    (token_address, amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapFor` (0x261c80b6) function"]
        pub fn unwrap_for(
            &self,
            sender: ::ethers::core::types::Address,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 28, 128, 182],
                    (sender, token_address, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateLimit` (0xfae0959a) function"]
        pub fn update_limit(
            &self,
            limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 224, 149, 154], limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `valid` (0xac8a260c) function"]
        pub fn valid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 138, 38, 12], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrap` (0xbf376c7a) function"]
        pub fn wrap(
            &self,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 55, 108, 122], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapFor` (0x2ca69388) function"]
        pub fn wrap_for(
            &self,
            sender: ::ethers::core::types::Address,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 166, 147, 136],
                    (sender, token_address, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapForAndSendTo` (0x7b2e30d6) function"]
        pub fn wrap_for_and_send_to(
            &self,
            sender: ::ethers::core::types::Address,
            token_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 46, 48, 214],
                    (sender, token_address, amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrappingLimit` (0x1f914382) function"]
        pub fn wrapping_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([31, 145, 67, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `HandlerUpdated` event"]
        pub fn handler_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HandlerUpdatedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PausedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        #[doc = r" Returns an `Event` builder for all the events of this contract."]
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FungibleTokenWrapperContractEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>>
        for FungibleTokenWrapperContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "HandlerUpdated", abi = "HandlerUpdated(address)")]
    pub struct HandlerUpdatedFilter {
        pub handler: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RoleGranted",
        abi = "RoleGranted(bytes32,address,address)"
    )]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RoleRevoked",
        abi = "RoleRevoked(bytes32,address,address)"
    )]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all of the contract's events"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum FungibleTokenWrapperContractEvents {
        ApprovalFilter(ApprovalFilter),
        HandlerUpdatedFilter(HandlerUpdatedFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FungibleTokenWrapperContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::ApprovalFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = HandlerUpdatedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::HandlerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::PausedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleAdminChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleGrantedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::TransferFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::UnpausedFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FungibleTokenWrapperContractEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HandlerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<HandlerUpdatedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: HandlerUpdatedFilter) -> Self {
            Self::HandlerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter>
        for FungibleTokenWrapperContractEvents
    {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    #[doc = "Container type for all input parameters for the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "PAUSER_ROLE", abi = "PAUSER_ROLE()")]
    pub struct PauserRoleCall;
    #[doc = "Container type for all input parameters for the `add` function with signature `add(address,uint32)` and selector `0xfc97a652`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "add", abi = "add(address,uint32)")]
    pub struct AddCall {
        pub token_address: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `burnFrom` function with signature `burnFrom(address,uint256)` and selector `0x79cc6790`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "burnFrom", abi = "burnFrom(address,uint256)")]
    pub struct BurnFromCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "decreaseAllowance",
        abi = "decreaseAllowance(address,uint256)"
    )]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `feePercentage` function with signature `feePercentage()` and selector `0xa001ecdd`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feePercentage", abi = "feePercentage()")]
    pub struct FeePercentageCall;
    #[doc = "Container type for all input parameters for the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeRecipient", abi = "feeRecipient()")]
    pub struct FeeRecipientCall;
    #[doc = "Container type for all input parameters for the `getAmountToWrap` function with signature `getAmountToWrap(uint256)` and selector `0x96cd4dfe`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAmountToWrap", abi = "getAmountToWrap(uint256)")]
    pub struct GetAmountToWrapCall {
        pub deposit: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getFee` function with signature `getFee()` and selector `0xced72f87`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getFee", abi = "getFee()")]
    pub struct GetFeeCall;
    #[doc = "Container type for all input parameters for the `getFeeFromAmount` function with signature `getFeeFromAmount(uint256)` and selector `0x85c00ae8`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getFeeFromAmount", abi = "getFeeFromAmount(uint256)")]
    pub struct GetFeeFromAmountCall {
        pub amount_to_wrap: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    #[doc = "Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getTokens` function with signature `getTokens()` and selector `0xaa6ca808`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTokens", abi = "getTokens()")]
    pub struct GetTokensCall;
    #[doc = "Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `handler` function with signature `handler()` and selector `0xc80916d4`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "handler", abi = "handler()")]
    pub struct HandlerCall;
    #[doc = "Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `historicalTokens` function with signature `historicalTokens(uint256)` and selector `0x85d14834`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "historicalTokens", abi = "historicalTokens(uint256)")]
    pub struct HistoricalTokensCall(pub ::ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `historicallyValid` function with signature `historicallyValid(address)` and selector `0xb1cba258`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "historicallyValid", abi = "historicallyValid(address)")]
    pub struct HistoricallyValidCall(pub ::ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "increaseAllowance",
        abi = "increaseAllowance(address,uint256)"
    )]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint16,address,address,uint256,bool,address)` and selector `0xf63ebb45`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(uint16,address,address,uint256,bool,address)"
    )]
    pub struct InitializeCall {
        pub fee_percentage: u16,
        pub fee_recipient: ::ethers::core::types::Address,
        pub handler: ::ethers::core::types::Address,
        pub limit: ::ethers::core::types::U256,
        pub is_native_allowed: bool,
        pub admin: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `0x158ef93e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    #[doc = "Container type for all input parameters for the `isNativeAllowed` function with signature `isNativeAllowed()` and selector `0xb3e4083f`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isNativeAllowed", abi = "isNativeAllowed()")]
    pub struct IsNativeAllowedCall;
    #[doc = "Container type for all input parameters for the `isValidToken` function with signature `isValidToken(address)` and selector `0xc1876453`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isValidToken", abi = "isValidToken(address)")]
    pub struct IsValidTokenCall {
        pub token_address: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    #[doc = "Container type for all input parameters for the `remove` function with signature `remove(address,uint32)` and selector `0x1c4a1436`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "remove", abi = "remove(address,uint32)")]
    pub struct RemoveCall {
        pub token_address: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFee` function with signature `setFee(uint16,uint32)` and selector `0xc2ae4720`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFee", abi = "setFee(uint16,uint32)")]
    pub struct SetFeeCall {
        pub fee_percentage: u16,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setFeeRecipient` function with signature `setFeeRecipient(address,uint32)` and selector `0x07184f1c`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setFeeRecipient",
        abi = "setFeeRecipient(address,uint32)"
    )]
    pub struct SetFeeRecipientCall {
        pub fee_recipient: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setHandler` function with signature `setHandler(address)` and selector `0xbac426d0`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address)")]
    pub struct SetHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setNativeAllowed` function with signature `setNativeAllowed(bool)` and selector `0x8b5478b9`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setNativeAllowed", abi = "setNativeAllowed(bool)")]
    pub struct SetNativeAllowedCall {
        pub is_native_allowed: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `tokens` function with signature `tokens(uint256)` and selector `0x4f64b2be`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "tokens", abi = "tokens(uint256)")]
    pub struct TokensCall(pub ::ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "transferFrom",
        abi = "transferFrom(address,address,uint256)"
    )]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[doc = "Container type for all input parameters for the `unwrap` function with signature `unwrap(address,uint256)` and selector `0x39f47693`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unwrap", abi = "unwrap(address,uint256)")]
    pub struct UnwrapCall {
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwrapAndSendTo` function with signature `unwrapAndSendTo(address,uint256,address)` and selector `0x4808285e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unwrapAndSendTo",
        abi = "unwrapAndSendTo(address,uint256,address)"
    )]
    pub struct UnwrapAndSendToCall {
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unwrapFor` function with signature `unwrapFor(address,address,uint256)` and selector `0x261c80b6`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unwrapFor", abi = "unwrapFor(address,address,uint256)")]
    pub struct UnwrapForCall {
        pub sender: ::ethers::core::types::Address,
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateLimit` function with signature `updateLimit(uint256)` and selector `0xfae0959a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateLimit", abi = "updateLimit(uint256)")]
    pub struct UpdateLimitCall {
        pub limit: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `valid` function with signature `valid(address)` and selector `0xac8a260c`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "valid", abi = "valid(address)")]
    pub struct ValidCall(pub ::ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `wrap` function with signature `wrap(address,uint256)` and selector `0xbf376c7a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "wrap", abi = "wrap(address,uint256)")]
    pub struct WrapCall {
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrapFor` function with signature `wrapFor(address,address,uint256)` and selector `0x2ca69388`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "wrapFor", abi = "wrapFor(address,address,uint256)")]
    pub struct WrapForCall {
        pub sender: ::ethers::core::types::Address,
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrapForAndSendTo` function with signature `wrapForAndSendTo(address,address,uint256,address)` and selector `0x7b2e30d6`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "wrapForAndSendTo",
        abi = "wrapForAndSendTo(address,address,uint256,address)"
    )]
    pub struct WrapForAndSendToCall {
        pub sender: ::ethers::core::types::Address,
        pub token_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `wrappingLimit` function with signature `wrappingLimit()` and selector `0x1f914382`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "wrappingLimit", abi = "wrappingLimit()")]
    pub struct WrappingLimitCall;
    #[doc = "Container type for all of the contract's call "]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum FungibleTokenWrapperContractCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MinterRole(MinterRoleCall),
        PauserRole(PauserRoleCall),
        Add(AddCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnFrom(BurnFromCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        FeePercentage(FeePercentageCall),
        FeeRecipient(FeeRecipientCall),
        GetAmountToWrap(GetAmountToWrapCall),
        GetFee(GetFeeCall),
        GetFeeFromAmount(GetFeeFromAmountCall),
        GetProposalNonce(GetProposalNonceCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetTokens(GetTokensCall),
        GrantRole(GrantRoleCall),
        Handler(HandlerCall),
        HasRole(HasRoleCall),
        HistoricalTokens(HistoricalTokensCall),
        HistoricallyValid(HistoricallyValidCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsNativeAllowed(IsNativeAllowedCall),
        IsValidToken(IsValidTokenCall),
        Mint(MintCall),
        Name(NameCall),
        Pause(PauseCall),
        Paused(PausedCall),
        ProposalNonce(ProposalNonceCall),
        Remove(RemoveCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetFee(SetFeeCall),
        SetFeeRecipient(SetFeeRecipientCall),
        SetHandler(SetHandlerCall),
        SetNativeAllowed(SetNativeAllowedCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        Tokens(TokensCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
        Unwrap(UnwrapCall),
        UnwrapAndSendTo(UnwrapAndSendToCall),
        UnwrapFor(UnwrapForCall),
        UpdateLimit(UpdateLimitCall),
        Valid(ValidCall),
        Wrap(WrapCall),
        WrapFor(WrapForCall),
        WrapForAndSendTo(WrapForAndSendToCall),
        WrappingLimit(WrappingLimitCall),
    }
    impl ::ethers::core::abi::AbiDecode for FungibleTokenWrapperContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <MinterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinterRole(decoded));
            }
            if let Ok(decoded) =
                <PauserRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauserRole(decoded));
            }
            if let Ok(decoded) =
                <AddCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Add(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnFrom(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok (decoded) = < DecreaseAllowanceCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: DecreaseAllowance (decoded)) }
            if let Ok(decoded) =
                <FeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FeePercentage(decoded));
            }
            if let Ok(decoded) =
                <FeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FeeRecipient(decoded));
            }
            if let Ok(decoded) =
                <GetAmountToWrapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetAmountToWrap(decoded));
            }
            if let Ok(decoded) =
                <GetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFee(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFromAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetFeeFromAmount(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok (decoded) = < GetRoleMemberCountCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: GetRoleMemberCount (decoded)) }
            if let Ok(decoded) =
                <GetTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTokens(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HistoricalTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HistoricalTokens(decoded));
            }
            if let Ok (decoded) = < HistoricallyValidCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: HistoricallyValid (decoded)) }
            if let Ok (decoded) = < IncreaseAllowanceCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: IncreaseAllowance (decoded)) }
            if let Ok(decoded) =
                <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Initialized(decoded));
            }
            if let Ok(decoded) =
                <IsNativeAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsNativeAllowed(decoded));
            }
            if let Ok(decoded) =
                <IsValidTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsValidToken(decoded));
            }
            if let Ok(decoded) =
                <MintCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) =
                <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RemoveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Remove(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetFeeRecipient(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <SetNativeAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetNativeAllowed(decoded));
            }
            if let Ok (decoded) = < SupportsInterfaceCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: SupportsInterface (decoded)) }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Tokens(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UnwrapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Unwrap(decoded));
            }
            if let Ok(decoded) =
                <UnwrapAndSendToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UnwrapAndSendTo(decoded));
            }
            if let Ok(decoded) =
                <UnwrapForCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnwrapFor(decoded));
            }
            if let Ok(decoded) =
                <UpdateLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UpdateLimit(decoded));
            }
            if let Ok(decoded) =
                <ValidCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Valid(decoded));
            }
            if let Ok(decoded) =
                <WrapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Wrap(decoded));
            }
            if let Ok(decoded) =
                <WrapForCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrapFor(decoded));
            }
            if let Ok(decoded) =
                <WrapForAndSendToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::WrapForAndSendTo(decoded));
            }
            if let Ok(decoded) =
                <WrappingLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::WrappingLimit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FungibleTokenWrapperContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Add(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountToWrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFeeFromAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Handler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HistoricalTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HistoricallyValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsNativeAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Remove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNativeAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Tokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unwrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapAndSendTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Valid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Wrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapForAndSendTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrappingLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FungibleTokenWrapperContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinterRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PauserRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Add(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAmountToWrap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeFromAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMember(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Handler(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HistoricalTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HistoricallyValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsNativeAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsValidToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Remove(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetNativeAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unwrap(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapAndSendTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnwrapFor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Valid(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wrap(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapForAndSendTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WrappingLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<MinterRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: MinterRoleCall) -> Self {
            Self::MinterRole(value)
        }
    }
    impl ::core::convert::From<PauserRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: PauserRoleCall) -> Self {
            Self::PauserRole(value)
        }
    }
    impl ::core::convert::From<AddCall> for FungibleTokenWrapperContractCalls {
        fn from(value: AddCall) -> Self {
            Self::Add(value)
        }
    }
    impl ::core::convert::From<AllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for FungibleTokenWrapperContractCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for FungibleTokenWrapperContractCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnFromCall> for FungibleTokenWrapperContractCalls {
        fn from(value: BurnFromCall) -> Self {
            Self::BurnFrom(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for FungibleTokenWrapperContractCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<FeePercentageCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: FeePercentageCall) -> Self {
            Self::FeePercentage(value)
        }
    }
    impl ::core::convert::From<FeeRecipientCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: FeeRecipientCall) -> Self {
            Self::FeeRecipient(value)
        }
    }
    impl ::core::convert::From<GetAmountToWrapCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetAmountToWrapCall) -> Self {
            Self::GetAmountToWrap(value)
        }
    }
    impl ::core::convert::From<GetFeeCall> for FungibleTokenWrapperContractCalls {
        fn from(value: GetFeeCall) -> Self {
            Self::GetFee(value)
        }
    }
    impl ::core::convert::From<GetFeeFromAmountCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetFeeFromAmountCall) -> Self {
            Self::GetFeeFromAmount(value)
        }
    }
    impl ::core::convert::From<GetProposalNonceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetProposalNonceCall) -> Self {
            Self::GetProposalNonce(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GetTokensCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GetTokensCall) -> Self {
            Self::GetTokens(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HandlerCall> for FungibleTokenWrapperContractCalls {
        fn from(value: HandlerCall) -> Self {
            Self::Handler(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for FungibleTokenWrapperContractCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<HistoricalTokensCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: HistoricalTokensCall) -> Self {
            Self::HistoricalTokens(value)
        }
    }
    impl ::core::convert::From<HistoricallyValidCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: HistoricallyValidCall) -> Self {
            Self::HistoricallyValid(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitializeCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<IsNativeAllowedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: IsNativeAllowedCall) -> Self {
            Self::IsNativeAllowed(value)
        }
    }
    impl ::core::convert::From<IsValidTokenCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: IsValidTokenCall) -> Self {
            Self::IsValidToken(value)
        }
    }
    impl ::core::convert::From<MintCall> for FungibleTokenWrapperContractCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for FungibleTokenWrapperContractCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<PauseCall> for FungibleTokenWrapperContractCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for FungibleTokenWrapperContractCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RemoveCall> for FungibleTokenWrapperContractCalls {
        fn from(value: RemoveCall) -> Self {
            Self::Remove(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for FungibleTokenWrapperContractCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SetFeeRecipientCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SetFeeRecipientCall) -> Self {
            Self::SetFeeRecipient(value)
        }
    }
    impl ::core::convert::From<SetHandlerCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SetHandlerCall) -> Self {
            Self::SetHandler(value)
        }
    }
    impl ::core::convert::From<SetNativeAllowedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SetNativeAllowedCall) -> Self {
            Self::SetNativeAllowed(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for FungibleTokenWrapperContractCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokensCall> for FungibleTokenWrapperContractCalls {
        fn from(value: TokensCall) -> Self {
            Self::Tokens(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for FungibleTokenWrapperContractCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for FungibleTokenWrapperContractCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UnwrapCall> for FungibleTokenWrapperContractCalls {
        fn from(value: UnwrapCall) -> Self {
            Self::Unwrap(value)
        }
    }
    impl ::core::convert::From<UnwrapAndSendToCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: UnwrapAndSendToCall) -> Self {
            Self::UnwrapAndSendTo(value)
        }
    }
    impl ::core::convert::From<UnwrapForCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: UnwrapForCall) -> Self {
            Self::UnwrapFor(value)
        }
    }
    impl ::core::convert::From<UpdateLimitCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: UpdateLimitCall) -> Self {
            Self::UpdateLimit(value)
        }
    }
    impl ::core::convert::From<ValidCall> for FungibleTokenWrapperContractCalls {
        fn from(value: ValidCall) -> Self {
            Self::Valid(value)
        }
    }
    impl ::core::convert::From<WrapCall> for FungibleTokenWrapperContractCalls {
        fn from(value: WrapCall) -> Self {
            Self::Wrap(value)
        }
    }
    impl ::core::convert::From<WrapForCall> for FungibleTokenWrapperContractCalls {
        fn from(value: WrapForCall) -> Self {
            Self::WrapFor(value)
        }
    }
    impl ::core::convert::From<WrapForAndSendToCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: WrapForAndSendToCall) -> Self {
            Self::WrapForAndSendTo(value)
        }
    }
    impl ::core::convert::From<WrappingLimitCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(value: WrappingLimitCall) -> Self {
            Self::WrappingLimit(value)
        }
    }
    #[doc = "Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `0xd5391393`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinterRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PauserRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `feePercentage` function with signature `feePercentage()` and selector `0xa001ecdd`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeePercentageReturn(pub u16);
    #[doc = "Container type for all return fields from the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeRecipientReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAmountToWrap` function with signature `getAmountToWrap(uint256)` and selector `0x96cd4dfe`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAmountToWrapReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getFee` function with signature `getFee()` and selector `0xced72f87`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetFeeReturn(pub u16);
    #[doc = "Container type for all return fields from the `getFeeFromAmount` function with signature `getFeeFromAmount(uint256)` and selector `0x85c00ae8`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetFeeFromAmountReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetProposalNonceReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getTokens` function with signature `getTokens()` and selector `0xaa6ca808`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTokensReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    #[doc = "Container type for all return fields from the `handler` function with signature `handler()` and selector `0xc80916d4`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HandlerReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HasRoleReturn(pub bool);
    #[doc = "Container type for all return fields from the `historicalTokens` function with signature `historicalTokens(uint256)` and selector `0x85d14834`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HistoricalTokensReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `historicallyValid` function with signature `historicallyValid(address)` and selector `0xb1cba258`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HistoricallyValidReturn(pub bool);
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `initialized` function with signature `initialized()` and selector `0x158ef93e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InitializedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isNativeAllowed` function with signature `isNativeAllowed()` and selector `0xb3e4083f`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsNativeAllowedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isValidToken` function with signature `isValidToken(address)` and selector `0xc1876453`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsValidTokenReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NameReturn(pub ::std::string::String);
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalNonceReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    #[doc = "Container type for all return fields from the `tokens` function with signature `tokens(uint256)` and selector `0x4f64b2be`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokensReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferFromReturn(pub bool);
    #[doc = "Container type for all return fields from the `valid` function with signature `valid(address)` and selector `0xac8a260c`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ValidReturn(pub bool);
    #[doc = "Container type for all return fields from the `wrappingLimit` function with signature `wrappingLimit()` and selector `0x1f914382`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WrappingLimitReturn(pub ::ethers::core::types::U256);
}
