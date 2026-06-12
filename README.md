Project Name: Decentralized Black Box dApp
A full-stack decentralized application (dApp) built on the Stellar blockchain utilizing Soroban smart contracts. This dApp acts as an on-chain "black box" for ride-hailing/transport drivers. It allows drivers to purchase insurance packages, and enables an authorized AI Server to automatically analyze accident telemetry, process claims, and permanently store verified accident reports with programmatic compensation calculation on-chain.

Table of Contents
Technologies Used

Smart-Contract Info

Project Setup Guide

Technologies Used
Smart Contract: Rust, Soroban-SDK (#![no_std] environment)

Wallet Integration: Freighter Wallet (Stellar/Soroban extension)

Development Environment: Soroban Studio / Stellar CLI

Smart-Contract Info
All core blockchain logic and state management are handled by the smart contract located in the source directory.

Path to Smart Contract: ./src/lib.rs

Network: Soroban Testnet

Data Structures Defined
Driver: Stores insurance data including expiry_timestamp (u64) and is_active (bool).

AccidentReport: Stores permanently archived claim data including driver_address, video_hash (IPFS reference), victim_address, compensation_amount (i128), and settled status.

Functions Implemented inside the Smart Contract
initialize(env: Env, admin: Address, ai_server: Address) Initializes the contract instance by permanently assigning the system administrator and the authorized AI Server address. Can only be invoked once.

buy_insurance(env: Env, driver: Address) Allows a driver to purchase or extend their insurance policy. Requires driver authentication and extends the policy coverage expiration by 30 days (2,592,000 seconds) from the current ledger timestamp.

trigger_compensation(env: Env, incident_id: BytesN<32>, driver: Address, victim: Address, video_hash: BytesN<32>, damage_level: u32) Invoked exclusively by the authorized ai_server to process a claim. It validates the driver's active insurance status, dynamically evaluates the damage_level to calculate tier-based compensation (Level 1: 500, Level 2: 1000, Level 3: 1500), and records a permanent, unalterable AccidentReport onto the ledger storage.

Project Setup Guide
Prerequisites
Ensure you have the following installed on your local machine or available in your development environment:

Rust toolchain (with wasm32-unknown-unknown target)

Stellar CLI

Freighter Wallet Browser Extension (configured to Testnet mode)
