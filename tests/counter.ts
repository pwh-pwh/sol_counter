import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Keypair,PublicKey } from "@solana/web3.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  const counterAccount = new Keypair();

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId
  );


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
        .rpc();
    console.log("Your transaction signature1", tx);

    const accountData = await program.account.counter.fetch(
        counterPDA
    );
    console.log(`Counter: ${accountData.count}`);
  });

  it("Increment", async () => {
        // Add your test here.
        const tx = await program.methods.increment()
            .rpc();
        console.log("Your transaction signature2", tx);

        const accountData = await program.account.counter.fetch(
            counterPDA
        );
        console.log(`Counter: ${accountData.count}`);
      }
    );
});
