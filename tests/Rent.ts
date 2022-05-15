import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Rent } from "../target/types/rent";

describe("Rent", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Rent as Program<Rent>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
