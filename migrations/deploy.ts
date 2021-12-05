// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
import * as anchor from "@project-serum/anchor";
import {
  INTERSOLAR_TYPE_MAPPER_PREFIX,
  PLANET_SYMBOL,
  PLANET_TYPE,
} from "./constants";
import { IntersolarTypeMapper } from "../target/types/intersolar_type_mapper";
import { Connection } from "@metaplex/js";

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  const intersolarTypeMapperProgram = anchor.workspace
    .IntersolarTypeMapper as anchor.Program<IntersolarTypeMapper>;

  const payer = anchor.getProvider().wallet;

  console.log(
    "type mapper program id",
    intersolarTypeMapperProgram.programId.toBase58()
  );

  // Add your deploy script here.
  const [intersolarTypeMapperPlanetPublicKey, bump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(INTERSOLAR_TYPE_MAPPER_PREFIX),
        Buffer.from(PLANET_SYMBOL),
        payer.publicKey.toBuffer(),
      ],
      intersolarTypeMapperProgram.programId
    );

  console.log(
    "TypeMapper Account pubkey:",
    intersolarTypeMapperPlanetPublicKey.toBase58()
  );

  const existingAccount =
    await intersolarTypeMapperProgram.account.intersolarTypeMapper.fetchNullable(
      intersolarTypeMapperPlanetPublicKey
    );
  if (existingAccount) {
    console.log("TypeMapper Account already exists:", existingAccount);
  } else {
    console.log("TypeMapper Account does not exist yet, creating...");

    const tx = new anchor.web3.Transaction({
      feePayer: payer.publicKey,
      recentBlockhash: (
        await anchor.getProvider().connection.getRecentBlockhash()
      ).blockhash,
    });

    tx.add(
      intersolarTypeMapperProgram.transaction.initialize(
        bump,
        PLANET_SYMBOL,
        PLANET_TYPE,
        {
          accounts: {
            intersolarTypeMapper: intersolarTypeMapperPlanetPublicKey,
            updateAuthority: payer.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
        }
      )
    );
    const tx2 = await payer.signTransaction(tx);
    const result = await anchor.web3.sendAndConfirmRawTransaction(
      anchor.getProvider().connection,
      tx2.serialize()
    );

    console.log("Transaction done:", result);
  }
};
