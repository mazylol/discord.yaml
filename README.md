# Discord.yaml
A very simple discord (library?)

## Example:
```yaml
token: your-token-here
guild_id: your-guild-id-here

responses:
  your: mom
  rick: roll
  never gonna: give you up

presence:
  status: online
  activity: playing
  description: bruh

commands:
  text:
    prefix: "!"
    commands:
    - name: ping
      response: pong
  slash:
    commands:
    - name: ping
      description: ping pong
      response: pong
```
