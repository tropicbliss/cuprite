# cuprite

A fast and simple Minecraft server backup daemon.

## Install

### Pre-compiled binaries (only available for x86-64 Linux systems)

- Download the [latest release](https://github.com/tropicbliss/cuprite/releases/latest)
- Make the downloaded binary executable

```shell
sudo chmod +x cuprite
```

### Compiling from source

If you are on another platform, compile the binary yourself to try it out:

```sh
git clone https://github.com/tropicbliss/cuprite
cd cuprite
cargo build --release
```

Compiling from source requires the latest stable version of Rust. Older Rust versions may be able to compile `cuprite`, but they are not guaranteed to keep working.

The binary will be located in `target/release`.

## Usage

```shell
./cuprite -i world -m 128 -o backups -p password -P 25575
```

### Enable RCON

```
# server.properties

enable-rcon=true
rcon.password=<your password>
rcon.port=<1-65535>
broadcast-rcon-to-ops=false
```

### Command line arguments

```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --compression-level <compression-level>    Compression level [default: 3]
    -i, --input-dir <input-dir>                    Input directory (directory to backup) [default: world]
    -m, --max-backups <max-backups>                Maximum number of backups to keep [default: 128]
    -o, --output-dir <output-dir>                  Output directory [default: backups]
    -P, --password <rcon-password>                 RCON password
    -p, --port <rcon-port>                         RCON port [default: 25575]
```

### Automating backups with CRON

It's a good idea to run `cuprite` on the terminal first to verify that you have inputted your arguments correctly before attempting to automate your backups with CRON.

- Edit the crontab with `crontab -e`

```
00 * * * * /path/to/cuprite
```

## Disclaimer

This software does not provide any warranty with regard to any loss of data.
