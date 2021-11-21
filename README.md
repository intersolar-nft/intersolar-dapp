# intersolar-dapp

Decentralized application for the [Intersolar NFT](https://intersolar-nft.com/) project.

# Programs

Solana on-chain programs that provide the functionality needed by [Intersolar](https://intersolar-nft.com/). Written using the [Anchor](https://github.com/project-serum/anchor) framework.

## Intersolar

This program is the root of all intersolar assets. It stores the name and type and provides renaming functionality to the token holder.

### PDA

['intersolar', mint_id]

### Account

```
#[account]
pub struct Intersolar {
    // The NFT token_mint this account belongs to
    pub mint: Pubkey,
    // The type of this intersolar object (e.g. 0 for "Planet", 1 for "Ship", ...)
    pub r#type: u8,
    // The name of the intersolar object (can be changed by the NFT owner)
    pub name: Option<String>,
    // The PDA bump of the intersolar account
    pub bump: u8
}
```

## IntersolarTypeMapper

This program is used to find the intersolar type (planet, ship, etc.) for a given metadata. It does this by having a PDA consisting of ['intersolar-type-mapper', metadata_symbol, metadata_update_authority]

### PDA

['intersolar-type-mapper', metadata_symbol, metadata_update_authority]

### Account

```
#[account]
pub struct IntersolarTypeMapper {
    // The corresponding intersolar type for the PDA symbol
    pub r#type: u8,
    // The PDA bump of the intersolar_type_mapper account
    pub bump: u8
}
```