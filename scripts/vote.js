const anchor = require('@project-serum/anchor');

async function main() {
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DaoVoting;
    const proposalPubkey = new anchor.web3.PublicKey("your-proposal-public-key");
    const userPubkey = new anchor.web3.PublicKey("your-user-public-key");

    await program.rpc.vote(true, {
        accounts: {
            proposal: proposalPubkey,
            user: userPubkey,
        },
    });

    console.log("Voted successfully");
}

main().then(() => console.log('Success')).catch(err => console.error(err));
