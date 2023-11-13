<p><a target="_blank" href="https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u" id="edit-in-eraser-github-link"><img alt="Edit in Eraser" src="https://firebasestorage.googleapis.com/v0/b/second-petal-295822.appspot.com/o/images%2Fgithub%2FOpen%20in%20Eraser.svg?alt=media&amp;token=968381c8-a7e7-472a-8ed6-4a6626da5501"></a></p>

# MedWallet Server
## High Level Architecture
![HLD Architecture](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---uzSQ26XwRZWl6XBTdNnEt---figure---08fOkGLTLePHsOu7btjsPw.png "HLD Architecture")



## **Client(**[﻿github repo](https://github.com/MedWallet/MedWalletMobile) **)**
![Client](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---tZLKUFmYh2PckaM3mTHrG---figure---Unfdard2avAr7dxdj6h50w.png "Client")

- **UI**: Interface for user actions and interactions.
- **Key management**: Managing encryption keys (generation, storage, etc.) locally on the client device.
- **Document encryption and decryption**: Encrypting data before it's sent to the server. Decrypting data retrieved from the server.
## **Server**
![Backend](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---kuNmPGrSyfiPWfknXpzY3---figure---QDltSN54dWkqYNjSA2OQwA.png "Backend")

- **API Gateway**: Entry point for routing client requests to appropriate backend services.
- **User Management (Auth & Profile)**: Manages user authentication and profile data.
- **Main Service**:
    - **Manage User**: Handles operations related to user data.
    - **Access Control**: Manages access to documents, ensuring proper user permissions.
    - **Manage Doc**: Responsible for health document-related functionalities.
- **Health Document Management**: Deals with the encryption and decryption of health documents.
- **Storage (IPFS)**: Uses InterPlanetary File System for decentralized document storage.
- **FHE processes (FHE, ZKP)**: Implements Full Homomorphic Encryption compatible operations from user's documents.
## **Blockchain Integration**
![Solana Blockchain](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---AX4rTPdfmq_GbABamwOjL---figure---OW9wbWYX7OlKMjNh2XDQ3Q.png "Solana Blockchain")

- **RPC Node**: Enables interaction with the Solana blockchain through Remote Procedure Calls.
- **On-Chain Program (Smart Contract)**: The blockchain-resident program that handles the logic for transactions.
- **Transaction**: Represents the execution of a smart contract on the blockchain.



<!--- Eraser file: https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u --->