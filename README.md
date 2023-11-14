<p><a target="_blank" href="https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u" id="edit-in-eraser-github-link"><img alt="Edit in Eraser" src="https://firebasestorage.googleapis.com/v0/b/second-petal-295822.appspot.com/o/images%2Fgithub%2FOpen%20in%20Eraser.svg?alt=media&amp;token=968381c8-a7e7-472a-8ed6-4a6626da5501"></a></p>

# MedWallet Server

## High Level Architecture

![HLD Architecture](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---uzSQ26XwRZWl6XBTdNnEt---figure---08fOkGLTLePHsOu7btjsPw.png "HLD Architecture")

### **Client(**[﻿github repo](https://github.com/MedWallet/MedWalletMobile) **)**

![Client](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---tZLKUFmYh2PckaM3mTHrG---figure---Unfdard2avAr7dxdj6h50w.png "Client")

- **UI**: Interface for user actions and interactions.
- **Key management**: Managing encryption keys (generation, storage, etc.) locally on the client device.
- **Document encryption and decryption**: Encrypting data before it's sent to the server. Decrypting data retrieved from the server.

### **Server**

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

### **Blockchain Integration**

![Solana Blockchain](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---AX4rTPdfmq_GbABamwOjL---figure---OW9wbWYX7OlKMjNh2XDQ3Q.png "Solana Blockchain")

- **RPC Node**: Enables interaction with the Solana blockchain through Remote Procedure Calls.
- **On-Chain Program (Smart Contract)**: The blockchain-resident program that handles the logic for transactions.
- **Transaction**: Represents the execution of a smart contract on the blockchain.

## Development

### Migrations

We are using [sqlx] for interacting with a PostgreSQL instance.

The accompanying [sqlx-cli] can be used to manage [schema migrations][migrations].

You can check the repo for more info, but basic usage is as follows.

_Create migration_

```shell
$ sqlx migrate add <name>
```

A new file (or two in case the first migration was added with `-r`) in the
`migrations/` directory. This is a standard `SQL` file that you can add table
creation queries.

In case of reversible migrations (enabled for this project), there will be two
files per migration `*.up.sql` and `*.down.sql`, corresponding to creation and reversal of a migration. When you revert a migration, the `*.down.sql` will be used, and applying the migrations will use `*.up.sql`.

_Run migration_

```shell
$ sqlx migrate run
```

_Revert last migration_

```shell
$ sqlx migrate revert
```

_Revert up to a migration_

```shell
$ sqlx migrate revert --target-version <TARGET_VERSION>
```

The `TARGET_VERSION` can be checked with, `sqlx migrate info`, eg.

```shell
$ sqlx migrate info
20231111154432/installed init
20231111154451/installed user table
20231114162015/pending documents
```

Here `20231111154432` is a target version for migration corresponding to `init`.

[sqlx]: https://github.com/launchbadge/sqlx/tree/main
[sqlx-cli]: https://github.com/launchbadge/sqlx/blob/main/sqlx-cli
[migrations]: https://en.wikipedia.org/wiki/Schema_migration

<!--- Eraser file: https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u --->
