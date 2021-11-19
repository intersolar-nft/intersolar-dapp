import * as anchor from '@project-serum/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import * as splToken from '@solana/spl-token';
import * as assert from 'assert';
import { IntersolarTypeMapper } from '../target/types/intersolar_type_mapper';

const PREFIX = "intersolar-type-mapper"

const PLANET_SYMBOL = "PLANET"
const PLANET_TYPE = 7


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

describe('intersolar-type-mapper', () => {

  it('initialize should succeed', async () => {
    anchor.setProvider(anchor.Provider.env());
    const connection = anchor.Provider.env().connection;
    const setup = await doSetup(connection);

    const [intersolarTypeMapperPlanetMappingPublicKey, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(PLANET_SYMBOL), setup.payerKeypair.publicKey.toBuffer()],
      intersolarTypeMapperProgram.programId
    );

    await intersolarTypeMapperProgram.rpc.initialize(bump, PLANET_SYMBOL, PLANET_TYPE, {
      accounts: {
        intersolarTypeMapper: intersolarTypeMapperPlanetMappingPublicKey,
        updateAuthority: setup.payerKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [
        setup.payerKeypair
      ]
    });

    const intersolarTypeMapperPlanetMappingAccount = await intersolarTypeMapperProgram.account.intersolarTypeMapper.fetch(intersolarTypeMapperPlanetMappingPublicKey);

    assert.equal(intersolarTypeMapperPlanetMappingAccount.rType, PLANET_TYPE);
  });
});
