# Chlaty-CLI

<div>
    <a href="https://github.com/chlaty/chlaty-cli/releases">
        <img src="https://img.shields.io/github/v/release/chlaty/chlaty-cli" />
    </a>
    <a href="https://github.com/chlaty/chlaty-cli/releases">
        <img src="https://img.shields.io/github/downloads/chlaty/chlaty-cli/total?color=green" />
    </a>
</div>

# What's Chlaty-CLI?
**Chlaty-CLI** is a powerful command-line utility that lets you seamlessly browse and stream your favorite shows.

# Preview
<div align="center" style="flex: auto">
  <img
    src="https://github.com/user-attachments/assets/3584a48b-f0a1-4c7f-b9d6-3119a4285281"
    style="width: 100%; object-fit: contain; border-radius: 6px" />
</div>

# Installation & Usage

<a href="https://github.com/chlaty/chlaty-cli/releases">
    <img src="https://img.shields.io/github/v/release/chlaty/chlaty-cli?style=for-the-badge&color=red" />
</a>

```bash
./chlaty-cli
```

### Environment Config
- If none of the directory is set it will default to current working directory.
- Set the directory to specfic path to avoid conflict and removal of existing directory.
```bash
# Directory for installing binaries.
CHLATY_BIN_DIRECTORY="bin"
# Directory for installing plugins.
CHLATY_PLUGIN_DIRECTORY="plugins"
# Directory for generated manifest.
CHLATY_STORAGE_DIRECTORY="storage"
```


# Used Dependencies 
- **[[chlaty-core]](https://github.com/chlaty/chlaty-core)**: A core package for managing plugins.
- **[[chlaty-player]](https://github.com/chlaty/chlaty-player)**: Lightweight media player designed for direct customization and feature extensibility.


