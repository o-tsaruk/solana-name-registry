import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NameRegistry } from "../target/types/name_registry";
import { Keypair } from "@solana/web3.js";

describe("name-registry", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.nameRegistry as Program<NameRegistry>;
  const user = new Keypair();
  const payer = provider.wallet as anchor.Wallet;

  it("Initialization", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        payer: payer.publicKey,
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
