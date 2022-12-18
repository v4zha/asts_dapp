pub use std_event::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod std_event {
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
    #[doc = "StdEvent was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"stdContractAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"astEvents\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"organizer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"winner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum ASTEvent.EventState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approval\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"enum Student.TokenKind\",\"name\":\"kind\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"tokenAmt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"stakeAmt\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenCounter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"organizer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum Student.TokenKind\",\"name\":\"kind\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"tokenAmt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"stakeAmt\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createASTEvent\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"winStd\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"endASTEvent\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStdEventLive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveEvent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getASTEventData\",\"outputs\":[{\"internalType\":\"struct ASTEvent.EventDetails\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"organizer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"winner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum ASTEvent.EventState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approval\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"enum Student.TokenKind\",\"name\":\"kind\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"tokenAmt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"stakeAmt\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"eventName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"enum Student.TokenKind\",\"name\":\"kind\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"tokenAmt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"stakeAmt\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitEvent\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static STDEVENT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct StdEvent<M>(ethers::contract::Contract<M>);
    impl<M> Clone for StdEvent<M> {
        fn clone(&self) -> Self {
            StdEvent(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for StdEvent<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for StdEvent<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(StdEvent))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> StdEvent<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), STDEVENT_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveEvent` (0x5f1cd158) function"]
        pub fn approve_event(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([95, 28, 209, 88], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `astEvents` (0x947e3bb7) function"]
        pub fn ast_events(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                String,
                String,
                ethers::core::types::Address,
                ethers::core::types::Address,
                u8,
                bool,
                u8,
                u32,
                u32,
            ),
        > {
            self.0
                .method_hash([148, 126, 59, 183], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createASTEvent` (0xf08304ac) function"]
        pub fn create_ast_event(
            &self,
            name: String,
            metadata: String,
            organizer: ethers::core::types::Address,
            kind: u8,
            token_amt: u32,
            stake_amt: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [240, 131, 4, 172],
                    (name, metadata, organizer, kind, token_amt, stake_amt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `endASTEvent` (0xd754eb3d) function"]
        pub fn end_ast_event(
            &self,
            win_std: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 84, 235, 61], (win_std, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getASTEventData` (0x201ed3aa) function"]
        pub fn get_ast_event_data(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, EventDetails> {
            self.0
                .method_hash([32, 30, 211, 170], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0x42842e0e) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xb88d4fde) function"]
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStdEventLive` (0x271a121d) function"]
        pub fn set_std_event_live(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 26, 18, 29], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitEvent` (0x7357d870) function"]
        pub fn submit_event(
            &self,
            event_name: String,
            metadata: String,
            kind: u8,
            token_amt: u32,
            stake_amt: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [115, 87, 216, 112],
                    (event_name, metadata, kind, token_amt, stake_amt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenCounter` (0xd082e381) function"]
        pub fn token_counter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([208, 130, 227, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenURI` (0xc87b56dd) function"]
        pub fn token_uri(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, StdEventEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for StdEvent<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StdEventEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for StdEventEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(StdEventEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(StdEventEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(StdEventEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for StdEventEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StdEventEvents::ApprovalFilter(element) => element.fmt(f),
                StdEventEvents::ApprovalForAllFilter(element) => element.fmt(f),
                StdEventEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `approveEvent` function with signature `approveEvent(uint256)` and selector `[95, 28, 209, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approveEvent", abi = "approveEvent(uint256)")]
    pub struct ApproveEventCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `astEvents` function with signature `astEvents(uint256)` and selector `[148, 126, 59, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "astEvents", abi = "astEvents(uint256)")]
    pub struct AstEventsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createASTEvent` function with signature `createASTEvent(string,string,address,uint8,uint32,uint32)` and selector `[240, 131, 4, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "createASTEvent",
        abi = "createASTEvent(string,string,address,uint8,uint32,uint32)"
    )]
    pub struct CreateASTEventCall {
        pub name: String,
        pub metadata: String,
        pub organizer: ethers::core::types::Address,
        pub kind: u8,
        pub token_amt: u32,
        pub stake_amt: u32,
    }
    #[doc = "Container type for all input parameters for the `endASTEvent` function with signature `endASTEvent(address,uint256)` and selector `[215, 84, 235, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "endASTEvent", abi = "endASTEvent(address,uint256)")]
    pub struct EndASTEventCall {
        pub win_std: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getASTEventData` function with signature `getASTEventData(uint256)` and selector `[32, 30, 211, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getASTEventData", abi = "getASTEventData(uint256)")]
    pub struct GetASTEventDataCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `[66, 132, 46, 14]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `[184, 141, 79, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `setStdEventLive` function with signature `setStdEventLive(uint256)` and selector `[39, 26, 18, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStdEventLive", abi = "setStdEventLive(uint256)")]
    pub struct SetStdEventLiveCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `submitEvent` function with signature `submitEvent(string,string,uint8,uint32,uint32)` and selector `[115, 87, 216, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "submitEvent",
        abi = "submitEvent(string,string,uint8,uint32,uint32)"
    )]
    pub struct SubmitEventCall {
        pub event_name: String,
        pub metadata: String,
        pub kind: u8,
        pub token_amt: u32,
        pub stake_amt: u32,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `tokenCounter` function with signature `tokenCounter()` and selector `[208, 130, 227, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenCounter", abi = "tokenCounter()")]
    pub struct TokenCounterCall;
    #[doc = "Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StdEventCalls {
        Approve(ApproveCall),
        ApproveEvent(ApproveEventCall),
        AstEvents(AstEventsCall),
        BalanceOf(BalanceOfCall),
        CreateASTEvent(CreateASTEventCall),
        EndASTEvent(EndASTEventCall),
        GetASTEventData(GetASTEventDataCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetStdEventLive(SetStdEventLiveCall),
        SubmitEvent(SubmitEventCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenCounter(TokenCounterCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for StdEventCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveEventCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::ApproveEvent(decoded));
            }
            if let Ok(decoded) =
                <AstEventsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::AstEvents(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CreateASTEventCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::CreateASTEvent(decoded));
            }
            if let Ok(decoded) =
                <EndASTEventCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::EndASTEvent(decoded));
            }
            if let Ok(decoded) =
                <GetASTEventDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::GetASTEventData(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(StdEventCalls::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithFromAndToAndDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(StdEventCalls::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SetStdEventLiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::SetStdEventLive(decoded));
            }
            if let Ok(decoded) =
                <SubmitEventCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::SubmitEvent(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenCounterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::TokenCounter(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StdEventCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for StdEventCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                StdEventCalls::Approve(element) => element.encode(),
                StdEventCalls::ApproveEvent(element) => element.encode(),
                StdEventCalls::AstEvents(element) => element.encode(),
                StdEventCalls::BalanceOf(element) => element.encode(),
                StdEventCalls::CreateASTEvent(element) => element.encode(),
                StdEventCalls::EndASTEvent(element) => element.encode(),
                StdEventCalls::GetASTEventData(element) => element.encode(),
                StdEventCalls::GetApproved(element) => element.encode(),
                StdEventCalls::IsApprovedForAll(element) => element.encode(),
                StdEventCalls::Name(element) => element.encode(),
                StdEventCalls::Owner(element) => element.encode(),
                StdEventCalls::OwnerOf(element) => element.encode(),
                StdEventCalls::SafeTransferFrom(element) => element.encode(),
                StdEventCalls::SafeTransferFromWithFromAndToAndData(element) => element.encode(),
                StdEventCalls::SetApprovalForAll(element) => element.encode(),
                StdEventCalls::SetStdEventLive(element) => element.encode(),
                StdEventCalls::SubmitEvent(element) => element.encode(),
                StdEventCalls::SupportsInterface(element) => element.encode(),
                StdEventCalls::Symbol(element) => element.encode(),
                StdEventCalls::TokenCounter(element) => element.encode(),
                StdEventCalls::TokenURI(element) => element.encode(),
                StdEventCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for StdEventCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StdEventCalls::Approve(element) => element.fmt(f),
                StdEventCalls::ApproveEvent(element) => element.fmt(f),
                StdEventCalls::AstEvents(element) => element.fmt(f),
                StdEventCalls::BalanceOf(element) => element.fmt(f),
                StdEventCalls::CreateASTEvent(element) => element.fmt(f),
                StdEventCalls::EndASTEvent(element) => element.fmt(f),
                StdEventCalls::GetASTEventData(element) => element.fmt(f),
                StdEventCalls::GetApproved(element) => element.fmt(f),
                StdEventCalls::IsApprovedForAll(element) => element.fmt(f),
                StdEventCalls::Name(element) => element.fmt(f),
                StdEventCalls::Owner(element) => element.fmt(f),
                StdEventCalls::OwnerOf(element) => element.fmt(f),
                StdEventCalls::SafeTransferFrom(element) => element.fmt(f),
                StdEventCalls::SafeTransferFromWithFromAndToAndData(element) => element.fmt(f),
                StdEventCalls::SetApprovalForAll(element) => element.fmt(f),
                StdEventCalls::SetStdEventLive(element) => element.fmt(f),
                StdEventCalls::SubmitEvent(element) => element.fmt(f),
                StdEventCalls::SupportsInterface(element) => element.fmt(f),
                StdEventCalls::Symbol(element) => element.fmt(f),
                StdEventCalls::TokenCounter(element) => element.fmt(f),
                StdEventCalls::TokenURI(element) => element.fmt(f),
                StdEventCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveCall> for StdEventCalls {
        fn from(var: ApproveCall) -> Self {
            StdEventCalls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveEventCall> for StdEventCalls {
        fn from(var: ApproveEventCall) -> Self {
            StdEventCalls::ApproveEvent(var)
        }
    }
    impl ::std::convert::From<AstEventsCall> for StdEventCalls {
        fn from(var: AstEventsCall) -> Self {
            StdEventCalls::AstEvents(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for StdEventCalls {
        fn from(var: BalanceOfCall) -> Self {
            StdEventCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CreateASTEventCall> for StdEventCalls {
        fn from(var: CreateASTEventCall) -> Self {
            StdEventCalls::CreateASTEvent(var)
        }
    }
    impl ::std::convert::From<EndASTEventCall> for StdEventCalls {
        fn from(var: EndASTEventCall) -> Self {
            StdEventCalls::EndASTEvent(var)
        }
    }
    impl ::std::convert::From<GetASTEventDataCall> for StdEventCalls {
        fn from(var: GetASTEventDataCall) -> Self {
            StdEventCalls::GetASTEventData(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for StdEventCalls {
        fn from(var: GetApprovedCall) -> Self {
            StdEventCalls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for StdEventCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            StdEventCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<NameCall> for StdEventCalls {
        fn from(var: NameCall) -> Self {
            StdEventCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for StdEventCalls {
        fn from(var: OwnerCall) -> Self {
            StdEventCalls::Owner(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for StdEventCalls {
        fn from(var: OwnerOfCall) -> Self {
            StdEventCalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for StdEventCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            StdEventCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithFromAndToAndDataCall> for StdEventCalls {
        fn from(var: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            StdEventCalls::SafeTransferFromWithFromAndToAndData(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for StdEventCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            StdEventCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SetStdEventLiveCall> for StdEventCalls {
        fn from(var: SetStdEventLiveCall) -> Self {
            StdEventCalls::SetStdEventLive(var)
        }
    }
    impl ::std::convert::From<SubmitEventCall> for StdEventCalls {
        fn from(var: SubmitEventCall) -> Self {
            StdEventCalls::SubmitEvent(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for StdEventCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            StdEventCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for StdEventCalls {
        fn from(var: SymbolCall) -> Self {
            StdEventCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenCounterCall> for StdEventCalls {
        fn from(var: TokenCounterCall) -> Self {
            StdEventCalls::TokenCounter(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for StdEventCalls {
        fn from(var: TokenURICall) -> Self {
            StdEventCalls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for StdEventCalls {
        fn from(var: TransferFromCall) -> Self {
            StdEventCalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `approveEvent` function with signature `approveEvent(uint256)` and selector `[95, 28, 209, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApproveEventReturn(pub bool);
    #[doc = "Container type for all return fields from the `astEvents` function with signature `astEvents(uint256)` and selector `[148, 126, 59, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AstEventsReturn {
        pub name: String,
        pub metadata: String,
        pub organizer: ethers::core::types::Address,
        pub winner: ethers::core::types::Address,
        pub state: u8,
        pub approval: bool,
        pub kind: u8,
        pub token_amt: u32,
        pub stake_amt: u32,
    }
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getASTEventData` function with signature `getASTEventData(uint256)` and selector `[32, 30, 211, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetASTEventDataReturn(pub EventDetails);
    #[doc = "Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetApprovedReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerOfReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `tokenCounter` function with signature `tokenCounter()` and selector `[208, 130, 227, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenCounterReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenURIReturn(pub String);
    #[doc = "`EventDetails(string,string,address,address,uint8,bool,uint8,uint32,uint32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EventDetails {
        pub name: String,
        pub metadata: String,
        pub organizer: ethers::core::types::Address,
        pub winner: ethers::core::types::Address,
        pub state: u8,
        pub approval: bool,
        pub kind: u8,
        pub token_amt: u32,
        pub stake_amt: u32,
    }
}
