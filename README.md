# McGet

Automated minecraft package manager that will download latest mods from CurseForge.

# Usage

type `$ mcget --help`, you'll get something like this:

```
Usage: mcget [-s <search>] [-v <version>] [-a <add>] [-l <mod-loader>] [--create-modpack <create-modpack>] [-d <download>] [-s <switch>] [-r <remove>]

CurseForge package manager for Minecarft mods

Options:
-s, --search      search mods by query
-v, --version     minecraft version
-a, --add         add first match on search to modpack
-l, --mod-loader  mod loader(e.g. forge)
--create-modpack  create modpack
-d, --download    download and switch modpack
-s, --switch      switch modpack
-r, --remove      remove modpack
--help            display usage information
```

`-s, --search [query]` - search mod that you need in CurseForge repository.

Search command can directly add mod to your modpack configuration, just pass `-a, --add [file]`, this will add first match to pack

`-v, --version [minecraft version]` - specify minecraft version


`-l, --mod-loader [loader]` - specify mod loader(Forge, Fabric, LiteLoader)

`-d, --downloader [modpack file]` - download mods from pack configuration

`-s, --switch [modpack name]` - switch current modpack

`-r, --remove [modpack name]` - remove modpack

`--create-modpack [modpack name]` - create modpack; This argument will create [modpack name].yaml file, see information below


# Modpack configuration

McGet has amazing feature: modpack configuration.

ModPack configuration is a single YAML file that contains information about needed mods:

- Mod Loader(forge by default)
- game version
- Mods

Let's take a closer look on blank configuration file:

```yaml
Minecraft:
  Name: mcpack
  ModLoader: forge
  Version: 1.12.2
  Mods: []
```

- `Name` - ModPack name (will be saved in mcget directory)
- `ModLoader` - explicit specification of minecraft mod loader
- `Version` - minecraft version
- `Mods` - list of mods

### Mods structure

If we search & add some mod to config it will look like this:

```yaml
Minecraft:
  Name: mcpack
  ModLoader: forge
  Version: 1.12.2
  Mods:
    - Id: 363543
```

So, `Mods` is a list of objects with a single `Id` key


# McGet modpack switching

Another McGet amazing feature is ModPack switching.

McGet can help you with saving up your storage space by simply downloading all modpacks to McGet's configuration directory.

If you call `mcget --switch [modpack]` mcget will make a symbolic link of modpack mods to your .minecraft/mods directory


# McGet configuration folder

McGet configuration folder location is dependent on running OS

- Windows: C:\\Users\\username\\AppData\\mcget
- Unix: /home/username/.local/mcget
- MacOS: /Users/username/mcget

McGet.yaml has only one field: MinecraftPath
