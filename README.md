# intersolar-dapp

# Programs

## Intersolar

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
// TODO 
```