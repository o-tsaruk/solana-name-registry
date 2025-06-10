import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NameRegistry } from "../target/types/name_registry";
import { expect } from "chai";

describe("name-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.nameRegistry as Program<NameRegistry>;
  const user = (provider.wallet as anchor.Wallet).payer;
  let recordPda: anchor.web3.PublicKey;

  const name = "test.name";
  const metadata = {
    jobTitle: "Solana Dev",
    bio: "I'm building dApps",
    iconLink: null,
    githubLink: null,
    socialLink: null,
  };

  beforeEach(async () => {
    const [_recordPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("record"), Buffer.from(name)],
      program.programId
    );
    recordPda = _recordPda;
  });

  it("Register name", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    const existing = await provider.connection.getAccountInfo(recordPda);
    expect(existing).to.equal(null);

    const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    const configAccount = await program.account.config.fetch(configPda);
    const adminPubkey = configAccount.admin;

    const accounts = {
      record: recordPda,
      config: configPda,
      admin: adminPubkey,
      user: user.publicKey,
    };

    await program.methods
      .registerName(name, metadata)
      .accounts(accounts)
      .signers([user])
      .rpc();

    const recordAccount = await program.account.nameRecord.fetch(recordPda);
    expect(recordAccount.owner.toBase58()).to.equal(user.publicKey.toBase58());
    expect(recordAccount.name).to.equal(name);
    expect(recordAccount.metadata).to.deep.equal(metadata);
  });

  it("Update metadata", async () => {
    const accounts = {
      record: recordPda,
      user: user.publicKey,
    };
    metadata.githubLink = "https://github.com/example";
    metadata.socialLink = "https://twitter.com/example";

    await program.methods
      .updateMetadata(name, metadata)
      .accounts(accounts)
      .signers([user])
      .rpc();

    const recordAccount = await program.account.nameRecord.fetch(recordPda);
    expect(recordAccount.metadata).to.deep.equal(metadata);
  });

  it("Transfer name", async () => {
    const newUser = anchor.web3.Keypair.generate();
    const accounts = {
      record: recordPda,
      user: user.publicKey,
      newUser: newUser.publicKey,
    };

    await program.methods
      .transferName(name)
      .accounts(accounts)
      .signers([user])
      .rpc();

    const recordAccount = await program.account.nameRecord.fetch(recordPda);
    expect(recordAccount.owner.toBase58()).to.equal(
      newUser.publicKey.toBase58()
    );
  });
});
