const anchor = require('@project-serum/anchor');

const splToken = require('@solana/spl-token');

describe('intersolar', () => {

  it('Is initialized!', async () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.Provider.env());

    const connection = anchor.Provider.env().connection;

    const payerKeypair = anchor.web3.Keypair.generate();

    const payerAirdropSignature = await connection.requestAirdrop(
      payerKeypair.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );

    await connection.confirmTransaction(payerAirdropSignature);

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

    const intersolarKeypair = anchor.web3.Keypair.generate();

    const program = anchor.workspace.Intersolar;
    const tx = await program.rpc.initialize(
      {
        accounts: {
          intersolar: intersolarKeypair.publicKey,
          user: receiverKeypair.publicKey,
          tokenMint: mint.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [
          receiverKeypair,
          intersolarKeypair
        ]
      });
    console.log("Your transaction signature", tx);
  });
});
