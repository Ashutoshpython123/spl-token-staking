import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplStake } from "../target/types/spl_stake";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"
import { getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID, createMint, mintTo } from "@solana/spl-token"
import { secret } from "./env";


describe("spl-stake", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplStake as Program<SplStake>;

  const admin = Keypair.fromSecretKey(Buffer.from(secret))
  const user = Keypair.generate();
  const poolInfo = Keypair.generate();
  const userInfo = Keypair.generate();

  let token: Token;
  let adminTokenAccount: PublicKey;
  let userTokenAccount: PublicKey;

  before(async () => {
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        user.publicKey,
        10 * LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    const mint = await createMint(
      program.provider.connection,
      admin,
      admin.publicKey,
      null,
      9,
      undefined,
      {},
      TOKEN_PROGRAM_ID,
    );

    adminTokenAccount = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      admin,
      mint,
      admin.publicKey,
    );

    userTokenAccount = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      user,
      mint,
      user.publicKey,
    );

    console.log("adminTokenAccount, userTokenAccount")

    await mintTo(
      program.provider.connection,
      admin,
      mint,
      userTokenAccount.address,
      admin.publicKey,
      1000000000000,
    )
  })

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature");
  });
});
