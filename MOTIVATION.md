# Motivation

## Preamble

In any hosted software system, the process to build the application, package configuration, provision the environment, and install and configure the software is a long running process.

Often there are issues requiring this process to be repeated:

* Last minute code changes.
* Configuration changes.
* Intermittent deployment issues.

Clean up and redeployment is costly in terms of time, effort, and morale.

We can apply the concepts of a programming language compiler -- if the desired artifact (or state) is already existent, then no work is done to produce the artifact.

Instead of restarting the build and deployment process from scratch, we can reuse most of the previous deployment, and replace only the parts that need replacing.

`peace` is an attempt to bring the simplicity of a modern build tool to all parts of the software lifecycle.

## Perspectives

### Development

* I want to set up my laptop with these applications and configuration that I normally use.
* A realistic CI environment takes a long time to build and deploy, so when I make a change, the rebuilt application should be able to be deployed into an existing integration environment.

### Operations

* When alerted, I want to know any differences between the deployed environment and its intended state.
* When I identify and solve the issue, I want to know what would change when I run a command before I execute it for real.
* I want to be able to backup these environments before I make a bulk change.

### Solution Implementation

* When there is an issue with a deployment, I want to understand what is wrong without needing to ask.
* When I change configuration, I want the deployment automation to only affect applications that were changed.

### Support

* I want to be able to deploy a replica customer environment to reproduce an issue.
* I want to know the deployment progress, so I know how long it takes before I can begin to investigate and resolve a customer issue.
