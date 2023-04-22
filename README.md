# Datcord-rs

This documentation follows the [official Discord API docs](https://discord.com/developers/docs/)

### Datcord-rs is a high-level Discord API wrapper

Current features:
- Getting all existing global commands[^1]
- Registering new commands[^2]
- Deleting existing commands[^3]
- Ed25519 encryption headers verification[^4]
- Responding to PING[^6] interactions[^5]

Currently working on:
- Responding to APPLICATION_COMMAND[^6] interactions

[^1]: [Getting existing commands](https://discord.com/developers/docs/interactions/application-commands#get-global-application-commands)
[^2]: [Creating new commands](https://discord.com/developers/docs/interactions/application-commands#create-global-application-command)
[^3]: [Deleting existing commands](https://discord.com/developers/docs/interactions/application-commands#delete-global-application-command)

[^4]: [How discord uses Ed25519](https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization)
[^5]: [Interactions](https://discord.com/developers/docs/interactions/receiving-and-responding)
[^6]: [Types of interactions](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type)