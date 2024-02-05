# Nginx Log Analyzer

## Overview

Analyzes Nginx access logs to provide insights into traffic. It parses nginx logs to determine the number of unique visitors per day and provides a summary of total visitors over the analyzed period.

**Defaults**

- No side effects, no temp files, no db writes
- Parses pattern `/var/log/nginx/access.log*`, including .gz files

**Options**

- Pass `--write-db` to create or update a simple sqlite database, containing date and visitor count.
- Specify db path with `--database <path>` to override default

## Build

```
cargo build --release
```

Note: if you get `/usr/bin/ld: cannot find -lsqlite3 collect2: error: ld returned 1 exit status`, you'll need the sqlite3 development package:

Debian/Ubuntu:

```
sudo apt-get install libsqlite3-dev
```

Red Hat/Fedora/CentOS-based systems:

```
sudo yum install sqlite-devel
```

Arch linux:

```
sudo pacman -S sqlite
```

macOS:

```
brew install sqlite
```

## Run

```
> sudo ./target/release/nginx_log_analyzer --write-db
2024-01-22: 3426 unique visitors
2024-01-23: 2385 unique visitors
2024-01-24: 2386 unique visitors
2024-01-25: 2426 unique visitors
2024-01-26: 2353 unique visitors
2024-01-27: 1979 unique visitors
2024-01-28: 1581 unique visitors
2024-01-29: 1529 unique visitors
2024-01-30: 1553 unique visitors
2024-01-31: 2076 unique visitors
2024-02-01: 4380 unique visitors
2024-02-02: 5785 unique visitors
2024-02-03: 7820 unique visitors
2024-02-04: 7512 unique visitors
2024-02-05: 2569 unique visitors
Total unique visitors across all days: 49760, over 15 days
```

## Test

```
 cargo run -- --pattern test/example.log
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/nginx_log_analyzer --pattern test/example.log`
2024-01-29: 1 unique visitors
2024-01-30: 1 unique visitors
2024-02-01: 1 unique visitors
2024-02-02: 2 unique visitors
2024-02-03: 1 unique visitors
2024-02-04: 1 unique visitors
Total unique visitors across all days: 7, over 6 days
```

## API

```
target/debug/nginx_log_analyzer --help
Nginx Log Analyzer 1.0
Chad Smith
Analyzes Nginx logs to count unique visitors per day and provide a summary.

USAGE:
    nginx_log_analyzer [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -V, --version     Prints version information
        --write-db    Enables writing the summary to the SQLite database

OPTIONS:
    -d, --database <DATABASE>    Sets the path to the SQLite database file [default: nginxaccess.db]
    -p, --pattern <PATTERN>      Sets the pattern to search for log files, including .gz files [default:
                                 /var/log/nginx/access.log*]
```
