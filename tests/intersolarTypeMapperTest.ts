import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as splToken from '@solana/spl-token';
import * as assert from 'assert';
import { IntersolarTypeMapper } from '../target/types/intersolar_type_mapper';
import {INTERSOLAR_TYPE_MAPPER_PREFIX, PLANET_SYMBOL, PLANET_TYPE} from "../migrations/constants";

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
  intersolarTypeMapperPublicKey: PublicKey,
  program: PublicKey
}

export async function setupTypeMapper(connection: Connection, updateAuthority: Keypair) : Promise<TypeMapperSetup> {

  const [intersolarTypeMapperPublicKey, bump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(INTERSOLAR_TYPE_MAPPER_PREFIX), Buffer.from(PLANET_SYMBOL), updateAuthority.publicKey.toBuffer()],
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
    program: intersolarTypeMapperProgram.programId
  }
}

describe('intersolar-type-mapper', () => {

  it('initialize should succeed', async () => {
    anchor.setProvider(anchor.Provider.env());
    const connection = anchor.Provider.env().connection;
    const setup = await doSetup(connection);

    const typeMapperSetup = await setupTypeMapper(connection, setup.payerKeypair);

    const intersolarTypeMapperPlanetMappingAccount = await intersolarTypeMapperProgram.account.intersolarTypeMapper.fetch(typeMapperSetup.intersolarTypeMapperPublicKey);

    assert.equal(intersolarTypeMapperPlanetMappingAccount.rType, PLANET_TYPE);
  });
});
