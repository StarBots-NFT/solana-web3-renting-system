import * as anchor from '@project-serum/anchor';
import fs from 'fs';
import * as bs58 from 'bs58';
import {web3, utils, BN, Accounts} from '@project-serum/anchor'
import {IdlAccountItem} from "@project-serum/anchor/dist/cjs/idl";
import {ASSOCIATED_PROGRAM_ID} from "@project-serum/anchor/src/utils/token";

const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);


async function main() {
    // Read the generated IDL.
    // Address of the deployed program.
    const programId = new anchor.web3.PublicKey('2QofFxHF1vPYKp6hUhi83iZpwLfXSKZsb1QKZwZoDSR8');

    const keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([
        224, 34, 123, 251, 222, 121, 159, 205, 198, 219, 97,
        146, 169, 24, 121, 140, 212, 142, 48, 10, 14, 98,
        238, 27, 166, 100, 78, 216, 75, 148, 248, 40, 29,
        122, 163, 221, 110, 183, 140, 40, 33, 251, 150, 160,
        88, 6, 228, 137, 59, 211, 106, 141, 186, 236, 69,
        56, 215, 190, 227, 33, 108, 44, 209, 120
    ]))
    // Generate the program client from IDL.

    const idl = await anchor.Program.fetchIdl(programId.toString());
    const program = new anchor.Program(idl, programId);
    const mint = new anchor.web3.PublicKey("92XUg3MoidJjaHZYF6TM2V9sDjJRffAu7tmTNP5DVdmS")
    const item = await web3.PublicKey.findProgramAddress(
        [Buffer.from('ballot'), mint.toBuffer(), provider.wallet.publicKey.toBuffer()],
        program.programId,
    )

    const [treasurerPublicKey] = await web3.PublicKey.findProgramAddress(
        [Buffer.from('treasurer'), item[0].toBuffer()],
        program.programId,
    )
    let treasurer = treasurerPublicKey

    const nftAta = new anchor.web3.PublicKey("12GPpLZ44QzYCnFWwE125eG25EF2UyamHAjUR1Fc2Q4U");
    let nftHolder = await utils.token.associatedAddress({
        mint: mint,
        owner: treasurerPublicKey,
    })

    console.log('idl: ', idl.instructions[0].accounts)

    const tx = await program.rpc.listItem(new BN(1), new BN(2), new BN(3),{
        accounts: {
            authority: provider.wallet.publicKey,
            item: item[0],
            treasurer: treasurer,
            mint: mint,
            nftHolder: nftHolder,
            nftAta: nftAta,
            systemProgram: web3.SystemProgram.programId,
            tokenProgram: utils.token.TOKEN_PROGRAM_ID,
            associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
            rent: web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [keypair]
    })
    console.log('tx: ', tx)
}

main().then(() => console.log('Success'));
