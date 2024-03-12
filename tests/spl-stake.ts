import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplStake } from "../target/types/spl_stake";

describe("spl-stake", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplStake as Program<SplStake>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
