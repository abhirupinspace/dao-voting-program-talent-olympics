DAO Voting Program

Table of Contents
Introduction
Features
Technologies
Setup
Prerequisites
Installation
Testing
Usage
Contributing
License
Introduction
Welcome to the DAO Voting Program repository! This project implements a decentralized autonomous organization (DAO) voting system on the Solana blockchain using Anchor framework. It allows users to create proposals, vote on them, and view voting results transparently and securely.

Features
Proposal Creation: Users can create proposals with a title and description.
Voting: Stakeholders can vote on proposals, supporting or rejecting them.
Results Viewing: Provides an interface to view voting results in real-time.
Technologies
Anchor Framework
Solana Blockchain
Rust Programming Language
Setup
Prerequisites
Before running this project locally, ensure you have the following installed:

Rust (nightly version recommended)
Solana Tool Suite (solana command-line tool)
Git
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/abhirupinspace/dao-voting-program-talent-olympics.git
cd dao-voting-program-talent-olympics
Build the Project:

bash
Copy code
cargo build --release
Testing
To run the tests:

bash
Copy code
cargo test
Usage
Deploying the Program:

Deploy the program to a local Solana cluster:

bash
Copy code
anchor deploy
Interacting with the Program:

Once deployed, interact with the program using its frontend or API endpoints.
