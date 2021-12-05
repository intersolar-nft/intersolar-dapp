// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
import * as anchor from '@project-serum/anchor';
import {INTERSOLAR_TYPE_MAPPER_PREFIX, PLANET_SYMBOL, PLANET_TYPE} from "./constants";
import { IntersolarTypeMapper } from '../target/types/intersolar_type_mapper';

const intersolarTypeMapperProgram = anchor.workspace.IntersolarTypeMapper as anchor.Program<IntersolarTypeMapper>;

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  const payer = anchor.getProvider().wallet

  // Add your deploy script here.
  const [intersolarTypeMapperPlanetPublicKey, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(INTERSOLAR_TYPE_MAPPER_PREFIX), Buffer.from(PLANET_SYMBOL), payer.publicKey.toBuffer()],
      intersolarTypeMapperProgram.programId
  );

  try {
    await intersolarTypeMapperProgram.account.intersolarTypeMapper.fetch(intersolarTypeMapperPlanetPublicKey);
  } catch (e) {
    console.log(e)
    payer.signTransaction()
    // TODO if error not found
    await intersolarTypeMapperProgram.rpc.initialize(bump, PLANET_SYMBOL, PLANET_TYPE, {
      accounts: {
        intersolarTypeMapper: intersolarTypeMapperPlanetPublicKey,
        updateAuthority: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [
        payer
      ]
    });
  }
}
