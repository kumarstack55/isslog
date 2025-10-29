# isslog - Interactive SSH Session Logger

This command primarily records interactive sessions of the ssh service for auditing purposes.

As stated in `man 2 execve`, the setgid bit on scripts is ignored.

> Linux (like most other modern UNIX systems) ignores the set-user-ID and set-group-ID bits on scripts.

Therefore, we implement a simple wrapper command to wrap the `script` command.

## Features

- Records logs of interactive sessions.
- Does not permit command specification like `ssh user@host "command arguments..."`.

## Requirements

- OpenSSH sshd service

## Usage

```bash
# bash

git clone https://github.com/kumarstack55/isslog.git

# Build and create the binary.
cd ./isslog
cargo build --release

# Deploy the binary with setgid enabled.
sudo useradd isslog
sudo install --owner isslog --group isslog -m 2755 ./target/release/isslog /usr/local/bin/isslog

# Configure the SSH service and apply the changes.
echo "ForceCommand /usr/local/bin/isslog" | sudo tee -a /etc/ssh/sshd_config.d/90-isslog.conf
sudo systemctl reload sshd.service
```

## License

MIT
