# git-manage

Switch between GitHub accounts without the pain. Never commit as the wrong account again.

## Features

- **Multi-Account Support**: Manage as many GitHub profiles as you need.
- **Repository-Aware Logging**: Tracks which account you applied to which repository.
- **Identity Guard Hook**: Optional pre-commit hook that blocks commits if your Git identity doesn't match your active profile.
- **Path Bindings**: Automatically associate specific folders with specific accounts.
- **SSH Integration**: Advisory checks for SSH host aliases in `~/.ssh/config`.
- **Zero Secrets**: Only manages public identity (username/email). No tokens, no keys.

## Install

```bash
cargo install --path .
```

## Commands

| command | description |
|---|---|
| `init` | Interactive setup wizard |
| `switch [alias]` | Switch active account (or cycle through all) |
| `apply [--global]` | Apply active identity to repo (or globally) |
| `status` | View active account & current Git configuration |
| `whoami` | Verify repo identity with mismatch warnings |
| `list` | Show all configured accounts |
| `add <alias> <user> <email> [--primary] [--ssh-host <host>]` | Add/update an account |
| `remove <alias>` | Remove an account |
| `bind <alias> [path]` | Bind a repository path to an account |
| `hook <install\|uninstall\|status>` | Manage the identity guard pre-commit hook |
| `log [--count N]` | Show switch history (with repo paths) |
| `prune` | Remove bindings for paths that no longer exist |
| `clear-log` | Clear the switch history |
| `completions <shell>` | Generate shell completions (zsh/bash/fish) |

## Typical Workflow

### 1. The "Set and Forget" Method (Recommended)
Bind a project folder to an account and install the hook. You'll never commit as the wrong person again.

```bash
cd ~/work/client-project
git-manage bind work-profile
git-manage hook install
```

### 2. The Manual Switch
If you don't want hooks, just switch and apply manually:

```bash
git-manage switch oss-profile
git-manage apply
```

## Switch History

`git-manage` keeps a record of your switches. Since v0.2.0, it automatically captures the repository path when you run `apply`:

```bash
git-manage log
# 2026-05-04 12:08:18  oss-profile [alt]  /home/user/dev/cool-app
```

## SSH Key Switching

`git-manage` handles identity (name/email). For SSH key switching per-account, use host aliases in `~/.ssh/config`:

```ssh
Host github-alt
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519_alt // your alt key

Host github.com
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519  # your primary key
```

Then add your account with the host:
`git-manage add alt-user alt-user alt@email.com --ssh-host github-alt`

## Shell Completions

```bash
# zsh
git-manage completions zsh > /usr/local/share/zsh/site-functions/_git-manage

# bash
git-manage completions bash >> ~/.bash_completion

# fish
git-manage completions fish > ~/.config/fish/completions/git-manage.fish
```

## Config Location

`~/.config/git-manage/config.json` — simple, human-readable JSON.
