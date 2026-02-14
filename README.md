# OEIS bot

➡️ [@OEISbot@mathstodon.xyz](https://mathstodon.xyz/@OEISbot) ⬅️

Fetch a random sequence from the [OEIS](https://oeis.org/), and post
it to Mastodon.

## Random sequence selection

A random sequence ID is picked in the range 1-380,000. Sequences that
are probably uninteresting are ignored. Currently this includes
sequences tagged with one of the keywords `dead`, `dumb`, `dupe`,
`less`, `obsc`, `probation`, and `uned`. See the
[documentation](https://oeis.org/eishelp2.html#RK) for details.

The sequence is then retrieved in the [JSON
format](https://oeis.org/wiki/JSON_Format).

## Mastodon configuration

- `MASTODON_INSTANCE_URL`: URL of the Mastodon instance.
- `MASTODON_ACCESS_TOKEN`: access token of the account on the Mastodon
  instance. It can be generated in Preferences > Development > New
  application. The token only needs the `write:statuses` permission.

## Deployment

1. Build with `cargo build --release`.
2. Copy the executable to the server:
   ```sh
   scp target/release/oeis_bot my-server:~/.local/bin
   ```
3. Copy the systemd service and timer configuration to the server:
   ```sh
   scp oeis-bot.{service,timer} my-server:~/.config/systemd/user
   ```
3. Edit `oeis-bot.service` with the correct value for the environment variables.
4. Activate the timer:
   ```sh
   systemctl --user daemon-reload
   systemctl --user start oeis-bot.timer
   ```
