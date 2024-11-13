# Eigenlayer and EVM tooling showcase

## Goals
* get some familiarity with the [Eigenlayer protocol](https://docs.eigenlayer.xyz/)
* get some hands-on familiarity with EVM tooling ([foundry-rs](https://book.getfoundry.sh/), [alloy-rs](https://alloy.rs/))

## What is Eigenlayer?
* a platform where you can recruit validators (operators) to secure your decentralized protocols (oracles, bridges, proofs...)
* works by restaking assets via Eigenlayer, which allows the same stake to secure multiple protocols

The protocol is based on the concept of **AVSs** and **tasks**:

**AVS (Actively Validated Service)** is a:
* a definition of a task or tasks which need to be regularly completed (respond to oracle data request, carry out bridging request, ...)
* a definition of rewards, slashing, task response acceptance criteria, economic demands on candidate validators etc.

Basic operation, **validators (operators):**
* register at Eingelayer and expose some stake (restake via Eigenlayer)
* register for AVSs which provide attractive rewards
* monitor new task creation at AVSs and provide correct responses in timely manner to accrue rewards, otherwise get slashed

**Delegators:**
* don't have the capability to provide validation services but want to partake in rewards, so they delegate stake to a validator to receive part of their validation rewards.

**AVSs:**
* write smart contracts which define their tasks, correctness criteria, economic security criteria...
* write offchain code to unburden onchain logic, write ready made task resolution services for potential validators to run

## Hands on exploration

Prerequisites:
* [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
* [foundry](https://book.getfoundry.sh/getting-started/installation)

### Mainnet exploration

Eigenlayer has two sets of smart contracts: [core protocol contracts](https://github.com/Layr-Labs/eigenlayer-contracts) and [AVS specific contracts]() expected to be to some extent modified and deployed by each AVS. Furthermore we want an AVS to look at, we chose the flagship [EigenDA]() AVS by Eigenlayer. We have all repos here as dependencies to obtain mainnet-deployed contract addresses and ABIs.

There are many natural questions to ask:
* how many AVSs are there currently?
* what information on them can be found onchain?
* how many operators are there?
* what stake does an operator have at Eigenlayer (asset types, amounts)?
* how many AVSs is an operator signed up for, with what stake?
* what kinds of stake does an AVS accept? are there any other criteria on candidate operators?
* ...

### Fork testing

In the last section we looked at read-only exploratory properties of the protocol. What if we want to test how it actually works? That's best done by interacting with it.

Some natural interaction-style questions, simplest to most complex, could be:
* as an asset holder wanting rewards, how do I restake to a chosen Eigenlayer operator?
* as a potential validator, how do I register with Eigenlayer?
* as a validator, how do I figure out task, reward and slashing conditions for an AVS? how do I then register for the AVS?
* how do I monitor tasks and provide task responses?
* how do I deregister when I no longer want to provide service for an AVS?
* how do I withdraw my rewards?
* as a protocol wanting to recruit validators, how do I build an AVS?

We'll fork mainnet to try our interactions. We have different options for writing them:

#### Using Solidity with foundry's anvil and forge

#### Using Rust (alloy-rs)