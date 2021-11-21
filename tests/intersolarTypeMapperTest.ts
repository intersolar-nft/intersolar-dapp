import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as splToken from '@solana/spl-token';
import * as assert from 'assert';
import { IntersolarTypeMapper } from '../target/types/intersolar_type_mapper';

export const PREFIX = "intersolar-type-mapper"

export const PLANET_SYMBOL = "PLANET"
export const PLANET_TYPE = 0


const intersolarTypeMapperProgram = anchor.workspace.IntersolarTypeMapper as anchor.Program<IntersolarTypeMapper>;

interface Setup {
  payerKeypair: Keypair,
}

async function doSetup(connection: Connection): Promise<Setup> {
  const payerKeypair = anchor.web3.Keypair.generate();

  await connection.confirmTransaction(await connection.requestAirdrop(
    payerKeypair.publicKey,
    anchor.web3.LAMPORTS_PER_SOL,
  ));

  return {
    payerKeypair
  };
}

export interface TypeMapperSetup {
  bump: number,
  intersolarTypeMapperPublicKey: PublicKey
}

export async function setupTyperMapper(connection: Connection, updateAuthority: Keypair) : Promise<TypeMapperSetup> {
  const [intersolarTypeMapperPublicKey, bump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), Buffer.from(PLANET_SYMBOL), updateAuthority.publicKey.toBuffer()],
    intersolarTypeMapperProgram.programId
  );

  await intersolarTypeMapperProgram.rpc.initialize(bump, PLANET_SYMBOL, PLANET_TYPE, {
    accounts: {
      intersolarTypeMapper: intersolarTypeMapperPublicKey,
      updateAuthority: updateAuthority.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    },
    signers: [
      updateAuthority
    ]
  });

  return {
    bump,
    intersolarTypeMapperPublicKey,
  }
}

describe('intersolar-type-mapper', () => {

  it('initialize should succeed', async () => {
    anchor.setProvider(anchor.Provider.env());
    const connection = anchor.Provider.env().connection;
    const setup = await doSetup(connection);

    const typeMapperSetup = await setupTyperMapper(connection, setup.payerKeypair);

    const intersolarTypeMapperPlanetMappingAccount = await intersolarTypeMapperProgram.account.intersolarTypeMapper.fetch(typeMapperSetup.intersolarTypeMapperPublicKey);

    assert.equal(intersolarTypeMapperPlanetMappingAccount.rType, PLANET_TYPE);
  });
});
