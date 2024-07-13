const anchor = require('@project-serum/anchor');

async function main() {
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DaoVoting;
    const proposal = anchor.web3.Keypair.generate();

    await program.rpc.createProposal("Proposal Title", "Proposal Description", {
        accounts: {
            proposal: proposal.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [proposal],
    });

    console.log("Proposal created: ", proposal.publicKey.toString());
}

main().then(() => console.log('Success')).catch(err => console.error(err));
