# Walkthrough

## Scenario

A workflow to deploy an application to a server can be modelled by the following steps:

1. Build the application.
2. Launch a server.
3. Transfer the application to the server.
4. Transfer application configuration to the server.
5. Run the application.
6. Allocate a domain name for the service.
7. Attach a domain name to the server.
8. When done, cleaning up the server, domain name, and built artifacts.

### Execution Variations

With these steps, there are many variations in the creation process:

1. Building and deploying from scratch.
2. Building and deploying to an existing server.
3. Building and upgrading an existing installation.
4. Using an existing build and replacing the server.
5. Changing the domain name attached to an existing server.

Similarly, there are a number of variations for cleaning up resources:

1. Clean up of all resources.
2. Clean up of resources that were successfully created, even though other resources were not created.
3. Clean up of resources still exist, even though others were terminated by an actor external to the process.
4. Nothing to clean up because they have already been cleaned up.

For automation to be useful to users, users should be able to easily choose to execute actions that matches their intent.

### Peace Model

When using Peace, this can be modelled using the following [item]s:

| Name              | Description                                          |
|:------------------|:-----------------------------------------------------|
| App Artifact      | Manages building of the application binary.          |
| Server            | Manages launching / cleaning up the server.          |
| Server Artifact   | Manages the application transfer to the server.      |
| Server Config     | Manages the application configuration on the server. |
| Server Service    | Manages the application service on the server.       |
| Domain Allocation | Manages the domain name allocation for the service.  |
| Domain Attachment | Handles linking the domain name to the server.       |

Notably the mapping of steps to items is nearly one to one. It is recommended that items are named after the state it manages rather than an action, because `"the server has been cleaned up"` makes more sense than `"the server launch has been cleaned up"`<sup>1</sup>.

---

<sup>1</sup> user feedback messages are yet to be implemented.


## Initialization

The following parameters should be defined by the user when initializing the workflow.

| Item               | Initialization Parameters                                    |
|:-------------------|:-------------------------------------------------------------|
| App Artifact       | App repository path                                          |
| Server             | Cloud credentials, SSH public key, Image ID, Instance type   |
| Server Artifact    | SSH private key, Destination path                            |
| Server Config      | SSH private key, Configuration source path, Destination path |
| Server App Service | SSH private key, App execution args                          |
| Domain Allocation  | Cloud credentials, Domain name                               |
| Domain Attachment  | Cloud credentials                                            |

The following list collates the initialization parameters:

|  # | Parameter          | Example                         |
|---:|:-------------------|:--------------------------------|
|  1 | `AppRepoPath`      | `/path/to/app_repo`             |
|  2 | `ConfigSourcePath` | `/path/to/app_config`           |
|  3 | `CloudCredentials` | AccessKey123<br /> SecretKey123 |
|  4 | `SshPrivateKey`    | `~/.ssh/id_rsa`                 |
|  5 | `SshPublicKey`     | `~/.ssh/id_rsa.pub`             |
|  6 | `ImageId`          | ami‑0123456789                  |
|  7 | `InstanceType`     | t4g.nano                        |
|  8 | `AppDestPath`      | `/opt/app`                      |
|  9 | `AppExecArgs`      | `--port 1234`                   |
| 10 | `DomainName`       | demo.something.com              |


## State Inspection

The following states are inspected and presented to the user:

| Item              | Logical State          | Physical State                           |
|:------------------|:-----------------------|:-----------------------------------------|
| App Artifact      | App binary             | Binary and source code modification time |
| Server            | Server existence       | Instance ID, IP address                  |
| Server Artifact   | App binary (on server) | n/a                                      |
| Server Config     | App config (on server) | n/a                                      |
| Server Service    | App service status     | Process ID, service start time           |
| Domain Allocation | Domain name existence  | n/a                                      |
| Domain Attachment | DNS configuration      | Record values                            |

The physical state of the **Domain Attachment** relies on the physical state of the server.


---

<!-- Maybe use the following table to show data values before/after, or maybe data access -->


| Item               | `AppRepoPath` | `ConfigSourcePath` | `CloudCredentials` | `SshPrivateKey` | `SshPublicKey` | `ImageId` | `InstanceType` | `AppDestPath` | `AppExecArgs` | `DomainName` |
|:-------------------|:--------------|:-------------------|:-------------------|:----------------|:---------------|:----------|:---------------|:--------------|:--------------|:-------------|
| App Artifact       |               |                    |                    |                 |                |           |                |               |               |              |
| Server             |               |                    |                    |                 |                |           |                |               |               |              |
| Server Artifact    |               |                    |                    |                 |                |           |                |               |               |              |
| Server Config      |               |                    |                    |                 |                |           |                |               |               |              |
| Server App Service |               |                    |                    |                 |                |           |                |               |               |              |
| Domain Allocation  |               |                    |                    |                 |                |           |                |               |               |              |
| Domain Attachment  |               |                    |                    |                 |                |           |                |               |               |              |
