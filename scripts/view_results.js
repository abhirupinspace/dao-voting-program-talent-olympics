const anchor = require('@project-serum/anchor');

async function main() {
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DaoVoting;
    const proposalPubkey = new anchor.web3.PublicKey("your-proposal-public-key");

    await program.rpc.viewResults({
        accounts: {
            proposal: proposalPubkey,
        },
    });
}

main().then(() => console.log('Success')).catch(err => console.error(err));
