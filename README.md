# containers.nvim

![Neovim](https://img.shields.io/badge/NeoVim-%2357A143.svg?&style=for-the-badge&logo=neovim&logoColor=white)
![Lua](https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

a Neovim plugin designed to seamlessly integrate container management into your editor. it allows you to view and manage containers across various providers.

**🚧 this plugin is in its infancy stage and is currently not/barely useable 🚧**

## features

- **container inspection:** view container status, logs, and configuration details
- **management commands:** start, stop and remove containers directly from Neovim
- **multi-provider support (planned):** future updates will include integration with multiple containerization tools like Podman and containerd

## supported providers

- Docker

## installation and configuration

to install `containers.nvim`, simply add it to your Neovim plugin list by specifying the repository `julianollivieira/containers.nvim`. after installation, configure the plugin in your Neovim configuration file:

```lua
require('containers').setup({
  providers = { "docker" },
})
```

## progress

- [ ] view container list
- [ ] view logs of hovered container
- [ ] get shell in hovered container
- [ ] view more details of hovered container
- [ ] start/stop containers
- [ ] add option for using floating window instead
- [ ] telescope pickers
- [ ] rich customization
