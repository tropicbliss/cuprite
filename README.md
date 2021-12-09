# cuprite

A fast and simple to use Minecraft server backup daemon.

## Install

Download the [latest release](https://github.com/tropicbliss/cuprite/releases/latest) for your operating system.

```shell
sudo chmod +x cuprite
```

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
    -p, --password <rcon-password>                 RCON password [default: ]
    -P, --port <rcon-port>                         RCON port [default: 25575]
```

### Automating backups with CRON

It's a good idea to run `cuprite` on the terminal first to verify that you have inputted your arguments correctly before attempting to automate your backups with CRON.

- Edit the crontab with `crontab -e`

```
00 * * * * /path/to/cuprite
```

## Disclaimer

This software does not provide any warranty with regard to any loss of data.
