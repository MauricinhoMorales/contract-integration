use ethers_contract::Contract;
use ethers_core::{
    abi::{Abi, Uint},
    types::{Address, H256, U256},
};
use ethers_providers::GOERLI;
use ethers_signers::{LocalWallet, Wallet};
use eyre::Result;

mod calls;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    // -----------------------------------------------CONSTANTS ----------------------------------------------------------
    let contract_address = "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b".parse::<Address>()?;

    let account_address = "0x768e02d0b50fcBc97163CBe70285236e97Ff3001".parse::<Address>()?;

    let abi: Abi = serde_json::from_str(
        r#"[
              {
                "type": "constructor",
                "name": "",
                "inputs": [],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "event",
                "name": "Approval",
                "inputs": [
                  {
                    "type": "address",
                    "name": "owner",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "spender",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "value",
                    "indexed": false,
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "DelegateChanged",
                "inputs": [
                  {
                    "type": "address",
                    "name": "delegator",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "fromDelegate",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "toDelegate",
                    "indexed": true,
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "DelegateVotesChanged",
                "inputs": [
                  {
                    "type": "address",
                    "name": "delegate",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "previousBalance",
                    "indexed": false,
                    "internalType": "uint256"
                  },
                  {
                    "type": "uint256",
                    "name": "newBalance",
                    "indexed": false,
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "PlatformFeeInfoUpdated",
                "inputs": [
                  {
                    "type": "address",
                    "name": "platformFeeRecipient",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "platformFeeBps",
                    "indexed": false,
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "PrimarySaleRecipientUpdated",
                "inputs": [
                  {
                    "type": "address",
                    "name": "recipient",
                    "indexed": true,
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "RoleAdminChanged",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "indexed": true,
                    "internalType": "bytes32"
                  },
                  {
                    "type": "bytes32",
                    "name": "previousAdminRole",
                    "indexed": true,
                    "internalType": "bytes32"
                  },
                  {
                    "type": "bytes32",
                    "name": "newAdminRole",
                    "indexed": true,
                    "internalType": "bytes32"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "RoleGranted",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "indexed": true,
                    "internalType": "bytes32"
                  },
                  {
                    "type": "address",
                    "name": "account",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "sender",
                    "indexed": true,
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "RoleRevoked",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "indexed": true,
                    "internalType": "bytes32"
                  },
                  {
                    "type": "address",
                    "name": "account",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "sender",
                    "indexed": true,
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "TokensMinted",
                "inputs": [
                  {
                    "type": "address",
                    "name": "mintedTo",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "quantityMinted",
                    "indexed": false,
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "TokensMintedWithSignature",
                "inputs": [
                  {
                    "type": "address",
                    "name": "signer",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "mintedTo",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "tuple",
                    "name": "mintRequest",
                    "components": [
                      {
                        "type": "address",
                        "name": "to",
                        "internalType": "address"
                      },
                      {
                        "type": "address",
                        "name": "primarySaleRecipient",
                        "internalType": "address"
                      },
                      {
                        "type": "uint256",
                        "name": "quantity",
                        "internalType": "uint256"
                      },
                      {
                        "type": "uint256",
                        "name": "price",
                        "internalType": "uint256"
                      },
                      {
                        "type": "address",
                        "name": "currency",
                        "internalType": "address"
                      },
                      {
                        "type": "uint128",
                        "name": "validityStartTimestamp",
                        "internalType": "uint128"
                      },
                      {
                        "type": "uint128",
                        "name": "validityEndTimestamp",
                        "internalType": "uint128"
                      },
                      {
                        "type": "bytes32",
                        "name": "uid",
                        "internalType": "bytes32"
                      }
                    ],
                    "indexed": false,
                    "internalType": "struct ITokenERC20.MintRequest"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "event",
                "name": "Transfer",
                "inputs": [
                  {
                    "type": "address",
                    "name": "from",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "to",
                    "indexed": true,
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "value",
                    "indexed": false,
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "anonymous": false
              },
              {
                "type": "function",
                "name": "DEFAULT_ADMIN_ROLE",
                "inputs": [],
                "outputs": [
                  {
                    "type": "bytes32",
                    "name": "",
                    "internalType": "bytes32"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "DOMAIN_SEPARATOR",
                "inputs": [],
                "outputs": [
                  {
                    "type": "bytes32",
                    "name": "",
                    "internalType": "bytes32"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "allowance",
                "inputs": [
                  {
                    "type": "address",
                    "name": "owner",
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "spender",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "approve",
                "inputs": [
                  {
                    "type": "address",
                    "name": "spender",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "amount",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "balanceOf",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "burn",
                "inputs": [
                  {
                    "type": "uint256",
                    "name": "amount",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "burnFrom",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "amount",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "checkpoints",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  },
                  {
                    "type": "uint32",
                    "name": "pos",
                    "internalType": "uint32"
                  }
                ],
                "outputs": [
                  {
                    "type": "tuple",
                    "name": "",
                    "components": [
                      {
                        "type": "uint32",
                        "name": "fromBlock",
                        "internalType": "uint32"
                      },
                      {
                        "type": "uint224",
                        "name": "votes",
                        "internalType": "uint224"
                      }
                    ],
                    "internalType": "struct ERC20VotesUpgradeable.Checkpoint"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "contractType",
                "inputs": [],
                "outputs": [
                  {
                    "type": "bytes32",
                    "name": "",
                    "internalType": "bytes32"
                  }
                ],
                "stateMutability": "pure"
              },
              {
                "type": "function",
                "name": "contractURI",
                "inputs": [],
                "outputs": [
                  {
                    "type": "string",
                    "name": "",
                    "internalType": "string"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "contractVersion",
                "inputs": [],
                "outputs": [
                  {
                    "type": "uint8",
                    "name": "",
                    "internalType": "uint8"
                  }
                ],
                "stateMutability": "pure"
              },
              {
                "type": "function",
                "name": "decimals",
                "inputs": [],
                "outputs": [
                  {
                    "type": "uint8",
                    "name": "",
                    "internalType": "uint8"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "decreaseAllowance",
                "inputs": [
                  {
                    "type": "address",
                    "name": "spender",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "subtractedValue",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "delegate",
                "inputs": [
                  {
                    "type": "address",
                    "name": "delegatee",
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "delegateBySig",
                "inputs": [
                  {
                    "type": "address",
                    "name": "delegatee",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "nonce",
                    "internalType": "uint256"
                  },
                  {
                    "type": "uint256",
                    "name": "expiry",
                    "internalType": "uint256"
                  },
                  {
                    "type": "uint8",
                    "name": "v",
                    "internalType": "uint8"
                  },
                  {
                    "type": "bytes32",
                    "name": "r",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "bytes32",
                    "name": "s",
                    "internalType": "bytes32"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "delegates",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "address",
                    "name": "",
                    "internalType": "address"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getPastTotalSupply",
                "inputs": [
                  {
                    "type": "uint256",
                    "name": "blockNumber",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getPastVotes",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "blockNumber",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getPlatformFeeInfo",
                "inputs": [],
                "outputs": [
                  {
                    "type": "address",
                    "name": "",
                    "internalType": "address"
                  },
                  {
                    "type": "uint16",
                    "name": "",
                    "internalType": "uint16"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getRoleAdmin",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  }
                ],
                "outputs": [
                  {
                    "type": "bytes32",
                    "name": "",
                    "internalType": "bytes32"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getRoleMember",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "uint256",
                    "name": "index",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "address",
                    "name": "",
                    "internalType": "address"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getRoleMemberCount",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "getVotes",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "grantRole",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "hasRole",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "increaseAllowance",
                "inputs": [
                  {
                    "type": "address",
                    "name": "spender",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "addedValue",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "initialize",
                "inputs": [
                  {
                    "type": "address",
                    "name": "_defaultAdmin",
                    "internalType": "address"
                  },
                  {
                    "type": "string",
                    "name": "_name",
                    "internalType": "string"
                  },
                  {
                    "type": "string",
                    "name": "_symbol",
                    "internalType": "string"
                  },
                  {
                    "type": "string",
                    "name": "_contractURI",
                    "internalType": "string"
                  },
                  {
                    "type": "address[]",
                    "name": "_trustedForwarders",
                    "internalType": "address[]"
                  },
                  {
                    "type": "address",
                    "name": "_primarySaleRecipient",
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "_platformFeeRecipient",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "_platformFeeBps",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "isTrustedForwarder",
                "inputs": [
                  {
                    "type": "address",
                    "name": "forwarder",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "mintTo",
                "inputs": [
                  {
                    "type": "address",
                    "name": "to",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "amount",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "mintWithSignature",
                "inputs": [
                  {
                    "type": "tuple",
                    "name": "_req",
                    "components": [
                      {
                        "type": "address",
                        "name": "to",
                        "internalType": "address"
                      },
                      {
                        "type": "address",
                        "name": "primarySaleRecipient",
                        "internalType": "address"
                      },
                      {
                        "type": "uint256",
                        "name": "quantity",
                        "internalType": "uint256"
                      },
                      {
                        "type": "uint256",
                        "name": "price",
                        "internalType": "uint256"
                      },
                      {
                        "type": "address",
                        "name": "currency",
                        "internalType": "address"
                      },
                      {
                        "type": "uint128",
                        "name": "validityStartTimestamp",
                        "internalType": "uint128"
                      },
                      {
                        "type": "uint128",
                        "name": "validityEndTimestamp",
                        "internalType": "uint128"
                      },
                      {
                        "type": "bytes32",
                        "name": "uid",
                        "internalType": "bytes32"
                      }
                    ],
                    "internalType": "struct ITokenERC20.MintRequest"
                  },
                  {
                    "type": "bytes",
                    "name": "_signature",
                    "internalType": "bytes"
                  }
                ],
                "outputs": [],
                "stateMutability": "payable"
              },
              {
                "type": "function",
                "name": "multicall",
                "inputs": [
                  {
                    "type": "bytes[]",
                    "name": "data",
                    "internalType": "bytes[]"
                  }
                ],
                "outputs": [
                  {
                    "type": "bytes[]",
                    "name": "results",
                    "internalType": "bytes[]"
                  }
                ],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "name",
                "inputs": [],
                "outputs": [
                  {
                    "type": "string",
                    "name": "",
                    "internalType": "string"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "nonces",
                "inputs": [
                  {
                    "type": "address",
                    "name": "owner",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "numCheckpoints",
                "inputs": [
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [
                  {
                    "type": "uint32",
                    "name": "",
                    "internalType": "uint32"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "permit",
                "inputs": [
                  {
                    "type": "address",
                    "name": "owner",
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "spender",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "value",
                    "internalType": "uint256"
                  },
                  {
                    "type": "uint256",
                    "name": "deadline",
                    "internalType": "uint256"
                  },
                  {
                    "type": "uint8",
                    "name": "v",
                    "internalType": "uint8"
                  },
                  {
                    "type": "bytes32",
                    "name": "r",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "bytes32",
                    "name": "s",
                    "internalType": "bytes32"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "primarySaleRecipient",
                "inputs": [],
                "outputs": [
                  {
                    "type": "address",
                    "name": "",
                    "internalType": "address"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "renounceRole",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "revokeRole",
                "inputs": [
                  {
                    "type": "bytes32",
                    "name": "role",
                    "internalType": "bytes32"
                  },
                  {
                    "type": "address",
                    "name": "account",
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "setContractURI",
                "inputs": [
                  {
                    "type": "string",
                    "name": "_uri",
                    "internalType": "string"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "setPlatformFeeInfo",
                "inputs": [
                  {
                    "type": "address",
                    "name": "_platformFeeRecipient",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "_platformFeeBps",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "setPrimarySaleRecipient",
                "inputs": [
                  {
                    "type": "address",
                    "name": "_saleRecipient",
                    "internalType": "address"
                  }
                ],
                "outputs": [],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "supportsInterface",
                "inputs": [
                  {
                    "type": "bytes4",
                    "name": "interfaceId",
                    "internalType": "bytes4"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "symbol",
                "inputs": [],
                "outputs": [
                  {
                    "type": "string",
                    "name": "",
                    "internalType": "string"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "totalSupply",
                "inputs": [],
                "outputs": [
                  {
                    "type": "uint256",
                    "name": "",
                    "internalType": "uint256"
                  }
                ],
                "stateMutability": "view"
              },
              {
                "type": "function",
                "name": "transfer",
                "inputs": [
                  {
                    "type": "address",
                    "name": "to",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "amount",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "transferFrom",
                "inputs": [
                  {
                    "type": "address",
                    "name": "from",
                    "internalType": "address"
                  },
                  {
                    "type": "address",
                    "name": "to",
                    "internalType": "address"
                  },
                  {
                    "type": "uint256",
                    "name": "amount",
                    "internalType": "uint256"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  }
                ],
                "stateMutability": "nonpayable"
              },
              {
                "type": "function",
                "name": "verify",
                "inputs": [
                  {
                    "type": "tuple",
                    "name": "_req",
                    "components": [
                      {
                        "type": "address",
                        "name": "to",
                        "internalType": "address"
                      },
                      {
                        "type": "address",
                        "name": "primarySaleRecipient",
                        "internalType": "address"
                      },
                      {
                        "type": "uint256",
                        "name": "quantity",
                        "internalType": "uint256"
                      },
                      {
                        "type": "uint256",
                        "name": "price",
                        "internalType": "uint256"
                      },
                      {
                        "type": "address",
                        "name": "currency",
                        "internalType": "address"
                      },
                      {
                        "type": "uint128",
                        "name": "validityStartTimestamp",
                        "internalType": "uint128"
                      },
                      {
                        "type": "uint128",
                        "name": "validityEndTimestamp",
                        "internalType": "uint128"
                      },
                      {
                        "type": "bytes32",
                        "name": "uid",
                        "internalType": "bytes32"
                      }
                    ],
                    "internalType": "struct ITokenERC20.MintRequest"
                  },
                  {
                    "type": "bytes",
                    "name": "_signature",
                    "internalType": "bytes"
                  }
                ],
                "outputs": [
                  {
                    "type": "bool",
                    "name": "",
                    "internalType": "bool"
                  },
                  {
                    "type": "address",
                    "name": "",
                    "internalType": "address"
                  }
                ],
                "stateMutability": "view"
              }
            ]"#,
    )?;
    println!("1. OBTENIDO EL ABI");

    //-------------------------------------------SUCCESSFULL TESTS-------------------------------------------------------

    // let result = calls::total_supply(contract_address).await?;
    // println!("TOTAL SUPPLY: {}", result.total_supply);

    // let result = calls::contract_type(contract_address).await?;
    // println!("CONTRACT TYPE: {}", result.contract_type);

    // let result = calls::allowance(contract_address, account_address, account_address).await?;
    // println!("ALLOWANCE: {}", result.allowance);

    // let result = calls::name(contract_address).await?;
    // println!("TOKEN CONTRACT NAME: {}", result.name);

    // let result = calls::symbol(contract_address).await?;
    // println!("TOKEN CONTRACT SYMBOL: {}", result.symbol);

    //--------------------------------------------TESTING HOW CAN WE MADE SIGN TRANSACTIONS WITH METAMASK ACCOUNT---------------------

    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");

    // Non-constant methods are executed via the `send()` call on the method builder.
    let call = contract
        .method::<_, (Address, U256)>("mintTo", (account_address, U256::from(100000000)))?;

    // TO DO: Ve aqui si puedes ver como lograr crear un wallet con lo de ethers-providers y asi hacer {wallet.signtransaction(call)} y le pasas el call que esta justo arriba

    // let wallet =
    //     LocalWallet::new(&mut "a0554bccb5a4cbd6e2f74754f66deee5410b67806361e629d1a71299abc8f6db");

    let pending_tx = call.send().await?;

    // `await`ing on the pending transaction resolves to a transaction receipt
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);

    //----------------------------------------------------------------------------------------------------------------------------------

    Ok(())
}
