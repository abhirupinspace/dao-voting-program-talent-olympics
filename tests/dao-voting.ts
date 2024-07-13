import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DaoVoting } from '../target/types/dao_voting';
import { assert } from 'chai';

describe('dao-voting', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.DaoVoting as Program<DaoVoting>;

  it('Creates a proposal', async () => {
    const proposal = anchor.web3.Keypair.generate();
    await program.rpc.createProposal('Test Proposal', 'This is a test proposal', {
      accounts: {
        proposal: proposal.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [proposal],
    });

    const account = await program.account.proposal.fetch(proposal.publicKey);
    assert.equal(account.title, 'Test Proposal');
    assert.equal(account.description, 'This is a test proposal');
    assert.equal(account.votesFor, 0);
    assert.equal(account.votesAgainst, 0);
  });

  it('Votes on a proposal', async () => {
    const proposal = anchor.web3.Keypair.generate();
    await program.rpc.createProposal('Test Proposal', 'This is a test proposal', {
      accounts: {
        proposal: proposal.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [proposal],
    });

    const user = anchor.web3.Keypair.generate();
    await program.rpc.vote(true, {
      accounts: {
        proposal: proposal.publicKey,
        user: user.publicKey,
      },
      signers: [user],
    });

    const account = await program.account.proposal.fetch(proposal.publicKey);
    assert.equal(account.votesFor, 1);
    assert.equal(account.votesAgainst, 0);
  });

  it('Views proposal results', async () => {
    const proposal = anchor.web3.Keypair.generate();
    await program.rpc.createProposal('Test Proposal', 'This is a test proposal', {
      accounts: {
        proposal: proposal.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [proposal],
    });

    await program.rpc.viewResults({
      accounts: {
        proposal: proposal.publicKey,
      },
    });

    const account = await program.account.proposal.fetch(proposal.publicKey);
    console.log('Proposal:', account);
  });
});
