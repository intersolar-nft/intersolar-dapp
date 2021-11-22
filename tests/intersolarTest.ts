import * as anchor from '@project-serum/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import * as splToken from '@solana/spl-token';
import * as assert from 'assert';
import { Intersolar } from '../target/types/intersolar';
import { AnyPublicKey, programs } from '@metaplex/js';
import { PLANET_SYMBOL, PLANET_TYPE, setupTypeMapper } from './intersolarTypeMapperTest';

const PREFIX = "intersolar"

const intersolarProgram = anchor.workspace.Intersolar as anchor.Program<Intersolar>;

interface MintSetup {
  payerKeypair: Keypair,
  mint: splToken.Token,
  receiverKeypair: Keypair,
  receiverTokenAccount: splToken.AccountInfo,
}

async function setupMint(connection: Connection): Promise<MintSetup> {
  const payerKeypair = anchor.web3.Keypair.generate();

  await connection.confirmTransaction(await connection.requestAirdrop(
    payerKeypair.publicKey,
    anchor.web3.LAMPORTS_PER_SOL,
  ));

  const mint = await splToken.Token.createMint(
    connection,
    payerKeypair,
    payerKeypair.publicKey,
    null,
    0,
    splToken.TOKEN_PROGRAM_ID,
  );

  const receiverKeypair = anchor.web3.Keypair.generate();

  const receiverAirdropSignature = await connection.requestAirdrop(
    receiverKeypair.publicKey,
    anchor.web3.LAMPORTS_PER_SOL,
  );

  await connection.confirmTransaction(receiverAirdropSignature);

  const receiverTokenAccount = await mint.getOrCreateAssociatedAccountInfo(
    receiverKeypair.publicKey,
  );

  await mint.mintTo(
    receiverTokenAccount.address,
    payerKeypair,
    [],
    1,
  );

  return {
    payerKeypair,
    mint,
    receiverKeypair,
    receiverTokenAccount
  };
}

interface MetadataSetup extends MintSetup {
  metadata: AnyPublicKey
}

async function setupMetadata(connection: Connection): Promise<MetadataSetup> {
  const setup = await setupMint(connection);

  const metadataPDA = await programs.metadata.Metadata.getPDA(setup.mint.publicKey);

  const createMetadataInstruction = new programs.metadata.CreateMetadata(
    { feePayer: setup.payerKeypair.publicKey, },
    {
      metadata: metadataPDA,
      metadataData: new programs.metadata.MetadataDataData({
        name: "#1",
        uri: "https://intersolar-nft.web.app/favicon-32x32.png",
        symbol: PLANET_SYMBOL,
        sellerFeeBasisPoints: 0.075,
        creators: [
          new programs.metadata.Creator(
            {
              address: setup.payerKeypair.publicKey.toString(),
              share: 100,
              verified: true
            }
          )
        ]
      }),
      mint: setup.mint.publicKey,
      mintAuthority: setup.payerKeypair.publicKey,
      updateAuthority: setup.payerKeypair.publicKey
    }
  )

  const signature = await anchor.web3.sendAndConfirmTransaction(
    connection,
    createMetadataInstruction,
    [setup.payerKeypair]
  )

  return {
    ...setup,
    metadata: metadataPDA
  }
}

interface IntersolarSetup extends MetadataSetup {
  intersolarPublicKey,
  bump
}

async function setupIntersolar(connection: Connection): Promise<IntersolarSetup> {
  const setup = await setupMetadata(connection);

  const typeMapperSetup = await setupTypeMapper(connection, setup.payerKeypair);

  const [intersolarPublicKey, bump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), setup.mint.publicKey.toBuffer()],
    intersolarProgram.programId
  );

  await intersolarProgram.rpc.initialize(
    bump,
    PLANET_SYMBOL,
    {
      accounts: {
        intersolar: intersolarPublicKey,
        typeMapper: typeMapperSetup.intersolarTypeMapperPublicKey,
        updateAuthority: setup.payerKeypair.publicKey,
        user: setup.receiverKeypair.publicKey,
        mint: setup.mint.publicKey,
        metadata: setup.metadata,
        typeMapperProgram: typeMapperSetup.program,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [
        setup.receiverKeypair,
      ]
    });
  return {
    ...setup,
    intersolarPublicKey,
    bump,
  };
}

describe('intersolar', () => {

  it('initialize should succeed', async () => {
    console.log(`intersolarProgram.programId: `, intersolarProgram.programId.toString());
    anchor.setProvider(anchor.Provider.env());
    const connection = anchor.Provider.env().connection;
    await setupIntersolar(connection);
  });

  xit('rename should succeed', async () => {
    anchor.setProvider(anchor.Provider.env());
    const connection = anchor.Provider.env().connection;
    const myMaxLengthName = "12345678901234567890123456789012"; // 32 bytes
    const setup = await setupIntersolar(connection);

    await intersolarProgram.rpc.rename(
      myMaxLengthName, {
      accounts: {
        intersolar: setup.intersolarPublicKey,
        user: setup.receiverKeypair.publicKey,
        mint: setup.mint.publicKey,
        tokenAccount: setup.receiverTokenAccount.address,
      },
      signers: [
        setup.receiverKeypair,
      ],
    });
    const intersolarAcc = await intersolarProgram.account.intersolar.fetch(setup.intersolarPublicKey);
    assert.equal(intersolarAcc.name, myMaxLengthName);
  });

  // TODO: Check creating two intersolar accounts for one mint fails
});
