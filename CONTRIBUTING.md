# Contributing to Loid

## How to

We use Github Issues to keep track of our bug reports.
Please write in detail what the issue you are facing is and how to reproduce it when submitting a bug report.

When creating an issue or PR, make sure they have each of these labels:
- priority:
    - critical
    - high
    - medium
    - low

- status:
    - blocked
    - confirmed
    - in-progress
    - needs-revision
    - on-hold
    - review
    - triage
    - abandoned
    - duplicate
    - wontfix
    - unfixable

- type:
    - bug
    - chore
    - dependencies
    - documentation
    - enhancement
    - feature
    - performance
    - refactor
    - security

- area:
    - multi
    - lexer
    - parser
    - javascript transpiler
    - dev environment
    - documentation
    - standard library

- difficulty:
    - easy
    - medium
    - hard
    - needs a team

And if needed:
- good first issue
- help wanted

Your PR will not be reviewed until the checks in Github Actions pass.
But do let us know if any issues arise, we will provide help depending on what is happening.

## Policies

Most policies here are self explanatory and can be summarized as this:
```
- Follow clean code procedures
- Have good documentation, with test cases
- Don't use AI to generate new code, and disclose where you've used it
```

In general we expect all contributors to follow a code of conduct:
```
We are committed to providing a welcoming and inclusive environment for all contributors. All Contributors are expected to behave in a professional, respectful, and constructive ways in all issues, pr requests, and anywhere else throughout the project. Failure to
```


### Formatting and Good Code Practices

Use `rustfmt` as described to format your code to Loid's specification

Follow Loid's style guide for writing new code





### Testing

Changes should always include tests for new behavior

All tests must pass before contributing


### Documentation

Changes should always be documented, with examples included where applicable

### Commit Messages

You should have a clear commit history

### AI

You can use LLMs for:

    - answering questions
    - analyzing
    - distilling
    - refining
    - checking
    - suggesting
    - reviewing

But **NOT** for creation

AI disclosure is necessary

Contributors are responsible for understanding and verifying everything they submit

## Building Loid

Loid uses <a href="https://github.com/casey/just">just</a> as its build tool.
Learn more about `just` at: <a href="https://just.systems/">just.systems</a>

## Breaking Changes

## Getting Dev Environment Set Up

### Get Started

To get started always just clone this repository.

```
git clone https://github.com/Loid-Project/loid.git
cd loid
```

Loid uses <a href="https://wiki.nixos.org/wiki/Flakes">nix flakes</a> with <a href="https://wiki.nixos.org/wiki/Direnv">direnv</a> for its dev environments.

After installing `direnv` you can do:
Then go into this project's root directory on your local machine.
And run:
```
direnv allow
```

This'll install all the dependencies and they'll only be accessible within this folder, not cluttering your actual computer.

### On NixOS

Get <a href="https://wiki.nixos.org/wiki/Direnv">direnv</a>.

You can enable it with:
```Nix
{
    programs.direnv.enable = true;
}

```

Make sure nix flakes are enabled:
```Nix
{
    nix.settings.experimental-features = [
        "nix-command"
        "flakes"
    ];
}
```

And rebuild with `$ nixos-rebuild switch --sudo`.

### On other Linux Distributions

Every most distributions provide the `direnv` package, however, for `direnv` to work correctly with `nix flakes`, one must still enable `nix`.

You can see how to do so here:

#### On any Distribution through Nix:

To use nix flakes on other distributions, one can set up the <a href="https://nixos.org/download/">nix</a> package manager and get direnv through it.

For a simpler approach you can just run:
```bash
curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install | sh -s -- --daemon
```

Then you can get flakes and commands enabled through:
```bash
echo "experimental-features = nix-command flakes" | sudo tee -a /etc/nix/nix.conf
```

And then:
```bash
nix profile add nixpkgs#nix-direnv
```

Then make a config file:
```bash
mkdir -p ~/.config/direnv
echo 'source $HOME/.nix-profile/share/nix-direnv/direnvrc' >> ~/.config/direnv/direnvrc
```

A similar result can be achieved by using the standard <a href="https://github.com/direnv/direnv">direnv</a> package. Instructions on their <a href="https://github.com/direnv/direnv">GitHub repository</a>.

After doing this, the `direnv` package should be downloaded through your distribution's package manager and you must use a shell hook.

This'll all be explained below:

#### On Arch Linux

This section applies to <a href="https://archlinux.org/">Arch Linux</a> and its derivatives such as <a href="https://cachyos.org/">CachyOS</a> or <a href="https://manjaro.org/">Manjaro</a>

`direnv` can be installed natively through `pacman`:
```
sudo pacman -S direnv
```

#### Ubuntu

This section applies to <a href="https://ubuntu.com/">Ubuntu</a> and its many derivatives.

`direnv` can be installed via `apt`:
```
sudo apt install direnv
```

#### Fedora

This section applies to <a href="https://fedoraproject.org/">Fedora Linux</a> and its various derivatives.

`direnv` can be installed via `dnf`:
```
sudo dnf install direnv
```

#### Shells:

Do the following in the root directory of your computer:

##### BASH
```
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
source ~/.bashrc
```

##### zsh
```
echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc
source ~/.zshrc
```

##### Fish
```
mkdir -p ~/.config/fish
echo 'direnv hook fish | source' >> ~/.config/fish/config.fish
source ~/.config/fish/config.fish
```

### MacOS

You must first install `nix` on MacOS:
```
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

You can install direnv via homebrew:
```
brew install direnv
```

Then add the shell hook to your zsh:
```
echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc
source ~/.zshrc
```

Install `nix-direnv` to allow for flake evaluation:
```
nix profile add nixpkgs#nix-direnv
```

Configuring the caching hook:
```
mkdir -p ~/.config/direnv
echo 'source $HOME/.nix-profile/share/nix-direnv/direnvrc' >> ~/.config/direnv/direnvrc
```

Then, in the root of the directory run:
```
direnv allow
```

### BSD

We do not yet support this platform.
Feel free to help bring Loid to BSD.

### Windows

We do not yet support Windows.
Feel free to help bring Loid to Windows.

## Dev Documentation

Check ./docs
