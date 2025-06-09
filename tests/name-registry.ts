import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NameRegistry } from "../target/types/name_registry";
import { expect } from "chai";

describe("name-registry", () => {
  beforeEach(async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    console.log("Initialization tx:", tx);
  });

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.nameRegistry as Program<NameRegistry>;
  const user = (provider.wallet as anchor.Wallet).payer;

  it("Register name", async () => {
    const [recordPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("record"), user.publicKey.toBuffer()],
      program.programId
    );

    const existing = await provider.connection.getAccountInfo(recordPda);
    expect(existing).to.equal(null);

    const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    const configAccount = await program.account.config.fetch(configPda);
    const adminPubkey = configAccount.admin;

    const name = "test.name";
    const accountsObj = {
      record: recordPda,
      config: configPda,
      admin: adminPubkey,
      user: user.publicKey,
    };
    
    await program.methods
      .registerName(name, null)
      .accounts(accountsObj)
      .signers([user])
      .rpc();

    const recordAccount = await program.account.nameRecord.fetch(recordPda);
    expect(recordAccount.owner.toBase58()).to.equal(user.publicKey.toBase58());
    expect(recordAccount.name).to.equal(name);
  });
});
