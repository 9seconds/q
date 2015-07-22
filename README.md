# q

Your gentle way to search through the pipes with a predefined sets of regular
expressions.

## Installation

Installation can be done with sources install or HomeBrew if you like.

### Sources

q uses PCRE bindings so before build please be sure that they are installed:

For Debian / Ubuntu install `libpcre3-dev`:

```bash
$ sudo apt-get install libpcre3-dev
```

For Fedora / CentOS / RHEL install `pcre-devel`:

```bash
$ yum install pcre-devel
```

After that just do the following

```bash
$ git clone https://github.com/9seconds/q.git
$ cd q
$ cargo build --release
```

and your binary safely sets into `target/release/q`. You may copy that or symlink
to the place you want (e.g `/usr/local/bin/q`). If you want to install man pages,
install them (`man/q.1` as man 1 page).

### HomeBrew

To install just do the following command in your terminal:

```bash
$ brew tap brew tap 9seconds/homebrew-q
$ brew install 9seconds/q/q
```
