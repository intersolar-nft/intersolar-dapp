import * as anchor from '@project-serum/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import * as splToken from '@solana/spl-token';
import * as assert from 'assert';
import { Intersolar } from '../target/types/intersolar';

const PREFIX = "intersolar"

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

describe('intersolar', () => {

  it('initialize should succeed', async () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.Provider.env());

    const connection = anchor.Provider.env().connection;

    const setup = await setupMint(connection);

    const program = anchor.workspace.Intersolar as anchor.Program<Intersolar>;
    const [intersolarPublicKey, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), setup.mint.publicKey.toBuffer()],
      program.programId
    );

    await program.rpc.initialize(
      bump,
      {
        accounts: {
          intersolar: intersolarPublicKey,
          user: setup.receiverKeypair.publicKey,
          tokenMint: setup.mint.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [
          setup.receiverKeypair,
        ]
      });
  });
});
