# asdfg

Installing global asdf packages from a YAML config.

asdfg requires [asdf](https://asdf-vm.com) to be installed and allows to configure and install/update multiple versions of [asdf-plugins](https://github.com/asdf-vm/asdf-plugins) by a single cli command.

## Installation

TBD

## Getting Started

You can open the asdfg YAML configuration file with your default editor by running:

```bash
$ asdfg config open
```

The YAML config is by default stored in `~/.asdfg/config.yaml`.

Add the asdf packages you want to install with their respective versions here, e.g.:

```yml
# ~/.asdfg/config.yaml

rust:
  - 1.39.0
  - latest
python: latest
erlang: latest
elixir:
  - 1.9.3
  - latest
golang: latest
```

The keys must match an asdf plugin name, the version(s) can either be a single value or a list of versions. All version must be available via the asdf plugin.
The last version for each asdf plugin is set to be the globally used version, when running:

```bash
$ asdfg install
```

In order to install all configured version of a specific asdf plugin, run:

```bash
$ asdfg install <plugin-name>
```

For further commands, please have a look at the cli documentation:

```bash
$ asdfg --help
```

## License

asdfg – Installing global asdf packages from a YAML config.  
Copyright (C) 2020, Paul Götze

This program is free software: you can redistribute it and/or modify  
it under the terms of the GNU General Public License as published by  
the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful,  
but WITHOUT ANY WARRANTY; without even the implied warranty of  
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the  
GNU General Public License for more details.

You should have received a copy of the GNU General Public License  
along with this program.  If not, see <https://www.gnu.org/licenses/>.
