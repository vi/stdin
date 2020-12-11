# stdinbot

```
$ stdinbot --help
Usage: stdinbot <token_file> -g <group> [-n]

Simple Telegram bot for posting each line read from stdin to a Telegram group

Options:
  -g, --group       telegram group ID to send all stdin lines to
  -n, --inhibit-updates
                    do not poll for updates for the bot, start only the message
                    sending task
  --help            display usage information
```

1. Create a Telegram bot
2. save the token to file
3. start this program with arbitrary numeric group id, e.g. `stdinbot mytokenfile -g 0`
4. create a Telegram group
5. invite the bot to the group
6. Observe GroupID somewhere in the debugging output
7. Interrupt stdinbot, then start it again, now with correct group ID, like `stdinbot mytokenfile -g -452343` 
8. Write some lines to program's stdin
9. Observe new messages in Telegram group

There is one pre-built non-static Linux release file available on Github releases.
