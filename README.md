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
    pub mint: Pubkey,
    pub key: u8,
    pub name: Option<String>,
    pub bump: u8,
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
    pub r#type: u8,
    pub bump: u8
}
```