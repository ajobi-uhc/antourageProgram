import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Antourage } from "../target/types/antourage";
import * as spl from "@solana/spl-token"

describe("antourage", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.Provider.local("https://api.devnet.solana.com");
  anchor.setProvider(provider);

  const program = anchor.workspace.Antourage as Program<Antourage>;

  it("Is initialized!", async () => {
    const user = anchor.web3.Keypair.fromSecretKey(
      new Uint8Array(
        JSON.parse(
          require("fs").readFileSync(
            "/Users/aryajakkli/.config/solana/id.json",
            "utf8"
          )
        )
      )
    );

    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

      let [program_signer_pubkey, signer_bump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("creator")],
        program.programId
      );


    console.log("program signer address", program_signer_pubkey.toBase58());

    let redLionMint = new anchor.web3.PublicKey(
      "9Ws9gSV9v9oxQjc8RjSJG9UJbVzU2FUmaBhYc7mx6PH5"
    );
    let redLionTokenAcc = new anchor.web3.PublicKey(
      "2891V2b9eEhEaGJBuu8Q8cvacBJ9PBzpVRMWgZ1v6VrT"
    );
    let [redLionMetadataAcc, bumpMetadata] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          redLionMint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

    let [counter, counterBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("counter")],
        program.programId
      );

    let golfMint = new anchor.web3.PublicKey(
      "7i6xxTAzEafx2VqSiu3M9juBVswb7PkxFAWhMVHwwt82"
    );
  
    let [golfMetadata, bumpMetGolf] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          golfMint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

    let [me, me_seeds] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        golfMint.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const [golfTokenAccount, tokenbump] =   await anchor.web3.PublicKey.findProgramAddress(
      [user.publicKey.toBuffer(), spl.TOKEN_PROGRAM_ID.toBuffer(), golfMint.toBuffer()],
      spl.ASSOCIATED_TOKEN_PROGRAM_ID
    )



    let rando = anchor.web3.Keypair.generate();

    console.log("counter",counter.toBase58())

    // let initTX = await program.rpc.initialize({
    //   accounts:{
    //     admin:user.publicKey,
    //     counter:counter,
    //     systemProgram:anchor.web3.SystemProgram.programId
    //   },
    //   signers:[]
    // })

    let programTX = await program.rpc.buyBall(counterBump, signer_bump, {
      accounts:{
  
        programPdaSigner:program_signer_pubkey,
        user:user.publicKey,
        redLionTokenAccount:redLionTokenAcc,
        redLionMintAccount:redLionMint,
        redLionMetadataAccount:redLionMetadataAcc,
        golfTokenAccount:golfTokenAccount,
        golfMintAccount:golfMint,
        golfMasterEdition:me,
        golfMetadataAccount:golfMetadata,
        counter:counter,
        tokenProgram:spl.TOKEN_PROGRAM_ID,
        tokenMetadataProgram:TOKEN_METADATA_PROGRAM_ID,
        systemProgram:anchor.web3.SystemProgram.programId
  
      },
      signers:[]
    })
    
    console.log("Your transaction signature", programTX);
  });
});
