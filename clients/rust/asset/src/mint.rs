use std::path::PathBuf;

use borsh::BorshDeserialize;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Creates a full asset by allocating any extensions, writing the data to them, and then calling
/// `create` to finalize the asset.
use crate::{
    instructions::{
        Allocate, AllocateInstructionArgs, Create, CreateInstructionArgs, Write,
        WriteInstructionArgs,
    },
    types::{Extension, ExtensionType, Standard},
};

/// Mint instruction args.
pub struct MintIxArgs {
    pub accounts: MintAccounts,
    pub asset_args: AssetArgs,
    pub extension_args: Vec<ExtensionArgs>,
}

/// Mint instruction accounts.
pub struct MintAccounts {
    pub asset: Pubkey,
    pub owner: Pubkey,
    /// If not specified, the owner is used as the payer.
    pub payer: Option<Pubkey>,
}

/// Mint instruction asset sub-args.
pub struct AssetArgs {
    pub name: String,
    pub standard: Standard,
    pub mutable: bool,
}

/// Mint instruction extension sub-args.
pub struct ExtensionArgs {
    pub extension_type: ExtensionType,
    pub data: Vec<u8>,
    pub chunked: bool,
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetFile {
    pub name: String,
    pub standard: Standard,
    pub mutable: bool,
    pub extensions: Vec<JsonExtension>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub owner: Pubkey,
    pub asset_keypair_path: Option<PathBuf>,
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonExtension {
    pub extension_type: ExtensionType,
    pub value: ExtensionValue,
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionValue {
    JsonCreator(Vec<JsonCreator>),
    JsonAttribute(Vec<JsonAttribute>),
    JsonLink(Vec<JsonLink>),
    JsonBlob(JsonBlob),
    JsonMetadata(JsonMetadata),
}

impl ExtensionValue {
    pub fn into_data(self) -> Vec<u8> {
        match self {
            Self::JsonCreator(value) => value.into_iter().fold(vec![], |mut acc, creator| {
                acc.extend(creator.into_data());
                acc
            }),
            Self::JsonAttribute(value) => value.into_iter().fold(vec![], |mut acc, attribute| {
                acc.extend(attribute.into_data());
                acc
            }),
            Self::JsonLink(value) => value.into_iter().fold(vec![], |mut acc, link| {
                acc.extend(link.into_data());
                acc
            }),
            Self::JsonBlob(value) => value.into_data(),
            Self::JsonMetadata(value) => value.into_data(),
        }
    }
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonCreator {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    address: Pubkey,
    verified: bool,
    share: u8,
}

impl JsonCreator {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub fn into_data(self) -> Vec<u8> {
        let mut data = vec![];
        data.extend(self.address.to_bytes());
        data.extend([self.verified as u8, self.share]);
        data.extend([0; 6]);
        data
    }

    pub fn from_data(data: &[u8]) -> Self {
        Self {
            address: Pubkey::try_from_slice(&data[0..32]).unwrap(),
            verified: data[32] != 0,
            share: data[33],
        }
    }
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonAttribute {
    pub name: String,
    pub value: String,
}

impl JsonAttribute {
    pub fn into_data(self) -> Vec<u8> {
        let mut data = vec![];

        let name_bytes = self.name.into_bytes();
        data.push(name_bytes.len() as u8);
        data.extend(name_bytes);

        let value_bytes = self.value.into_bytes();
        data.push(value_bytes.len() as u8);
        data.extend(value_bytes);

        data
    }
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonLink {
    pub name: String,
    pub uri: String,
}

impl JsonLink {
    pub fn into_data(self) -> Vec<u8> {
        let mut data = vec![];

        let name_bytes = self.name.into_bytes();
        data.push(name_bytes.len() as u8);
        data.extend(name_bytes);

        let uri_bytes = self.uri.into_bytes();
        data.push(uri_bytes.len() as u8);
        data.extend(uri_bytes);

        data
    }
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonBlob {
    pub content_type: String,
    pub path: String,
}

impl JsonBlob {
    pub fn into_data(self) -> Vec<u8> {
        let mut data = vec![];

        let content_type_bytes = self.content_type.into_bytes();
        data.push(content_type_bytes.len() as u8);
        data.extend(content_type_bytes);

        let path = PathBuf::from(self.path);
        let blob_data = std::fs::read(path).expect("failed to read blob file");
        data.extend(blob_data);

        data
    }
}

/// A type suitable for JSON serde de/serialization.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonMetadata {
    pub symbol: String,
    pub uri: String,
}

impl JsonMetadata {
    pub fn into_data(self) -> Vec<u8> {
        let mut data = vec![];

        let symbol_bytes = self.symbol.into_bytes();
        data.push(symbol_bytes.len() as u8);
        data.extend(symbol_bytes);

        let uri_bytes = self.uri.into_bytes();
        data.push(uri_bytes.len() as u8);
        data.extend(uri_bytes);

        data
    }
}

/// Errors returned from the mint command.
#[derive(Debug, Error)]
pub enum MintError {
    #[error("Invalid extension type: {0}")]
    InvalidExtensionType(String),
    #[error("Invalid extension data: {0}")]
    InvalidExtensionData(String),
}

const MAX_IX_DATA_SIZE: usize = 925;

/// Returns a vector of instructions to fully mint an asset, including with extensions.
/// The instructions are returned in the order they should be executed.
pub fn mint(args: MintIxArgs) -> Result<Vec<Instruction>, MintError> {
    let mut instructions = vec![];

    let payer = args.accounts.payer.unwrap_or(args.accounts.owner);

    // Extension allocation instructions.
    for extension in args.extension_args.iter() {
        let ix_args = AllocateInstructionArgs {
            extension: Extension {
                extension_type: extension.extension_type.clone(),
                length: extension.data.len() as u32,
                data: if extension.chunked {
                    None
                } else {
                    Some(extension.data.clone())
                },
            },
        };

        instructions.push(
            Allocate {
                asset: args.accounts.asset,
                payer: Some(payer),
                system_program: Some(solana_program::system_program::id()),
            }
            .instruction(ix_args),
        );

        // Write data instructions.
        if extension.chunked {
            for chunk in extension.data.chunks(MAX_IX_DATA_SIZE) {
                let ix_args = WriteInstructionArgs {
                    overwrite: false,
                    bytes: chunk.to_vec(),
                };

                instructions.push(
                    Write {
                        asset: args.accounts.asset,
                        payer,
                        system_program: solana_program::system_program::id(),
                    }
                    .instruction(ix_args),
                );
            }
        }
    }

    // Finalize the asset by creating it.
    let ix_args = CreateInstructionArgs {
        name: args.asset_args.name,
        standard: args.asset_args.standard,
        mutable: args.asset_args.mutable,
    };

    instructions.push(
        Create {
            asset: args.accounts.asset,
            authority: args.accounts.owner,
            holder: args.accounts.owner,
            payer: Some(payer),
            system_program: Some(solana_program::system_program::id()),
        }
        .instruction(ix_args),
    );

    Ok(instructions)
}