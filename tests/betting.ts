import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Betting } from "../target/types/betting";

const {
  SystemProgram,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  clusterApiUrl,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
  Transaction,
  sendAndConfirmTransaction,
} = anchor.web3;

import keypair1 from "../id.json";
import keypair2 from "../id2.json";
import keypair3 from "../id3.json";
import { BN } from "bn.js";

const sleep = (ms: number): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

enum Winner {
  Left,
  Right,
}

describe("betting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Betting as Program<Betting>;

  it("Is initialized!", async () => {
    // Add your test here.
    const provider = anchor.AnchorProvider.env();
    // const signer1 = Keypair.fromSecretKey(Uint8Array.from(keypair1));
    // const signer2 = Keypair.fromSecretKey(Uint8Array.from(keypair2));
    // const signer3 = Keypair.fromSecretKey(Uint8Array.from(keypair3));

    let signer1 = Keypair.generate();
    let signer2 = Keypair.generate();
    let signer3 = Keypair.generate();

    const AIRDROP_AMOUNT = 10000000000;

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        signer1.publicKey,
        AIRDROP_AMOUNT
      ),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        signer2.publicKey,
        AIRDROP_AMOUNT
      ),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        signer3.publicKey,
        AIRDROP_AMOUNT
      ),
      "confirmed"
    );

    let [battlePDA] = await PublicKey.findProgramAddress(
      [Buffer.from("battle"), signer1.publicKey.toBuffer()],
      program.programId
    );
    console.log("battlePDA = ", battlePDA.toBase58());

    let [escrowPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("escrow")],
      program.programId
    );

    let slot = await provider.connection.getSlot("finalized");
    let time = await provider.connection.getBlockTime(slot);
    console.log("time = ", time);

    await provider.connection.confirmTransaction(
      await program.rpc.createBattle(new BN(time), new BN(time + 10), {
        accounts: {
          authority: signer1.publicKey,
          battle: battlePDA,
          escrow: escrowPDA,
          rentSysvar: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        signers: [signer1],
      })
    );

    let [userBettingPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("bet"), signer2.publicKey.toBuffer()],
      program.programId
    );
    console.log("userBettingPDA = ", userBettingPDA.toBase58());

    let left = {left: true};

    await provider.connection.confirmTransaction(
      await program.rpc.bet(left, new BN(1000000000), {
        accounts: {
          authority: signer2.publicKey,
          admin: signer1.publicKey,
          userBetting: userBettingPDA,
          battle: battlePDA,
          escrow: escrowPDA,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        signers: [signer2],
      })
    );
  });
});
