# q

Your gentle way to search through the pipes with a predefined sets of
regular expressions.



## What is q

I wrote q because I was bored to grep through the logs or some text
files for information I want. For example, if I want to extract unique
IPs from the logs of my application I had to do following:

```bash
$ egrep -o "([0-9]{1,3}[\.]){3}[0-9]{1,3}" /var/log/myapp/*.log | sort -u
```

Well, it works but what I want to say is that I want to have an IPs. I
do not want to have `"([0-9]{1,3}[\.]){3}[0-9]{1,3}"`, right? Sometimes
regular expressions are not that simple as IP ones so I have to look
through my ZSH history to realize which regexp I have to use now because
I am too lazy or running out of time.

BTW, `"([0-9]{1,3}[\.]){3}[0-9]{1,3}"` is rather bad regexp for IPs,
guess why. And I have to repeatedly improve my regexp to improve the
results. Oh, and reading from local files are fast, but what if you pipe
from network?

Well, the only reason why q exists is an ability to give you predefined
presets of regular expressions. Every expression (or set, one expression
per file line) is stored in some files and you just have to say what are
you going to find.

```bash
$ q -o ipv4 /var/log/myapp/*.log | sort -u
```

It gives you the same results. If you will decide to search for URLs too, just do

```bash
$ q -o ipv4,url /var/log/myapp/*.log | sort -u
```

No need to remember or reconstruct regular expressions every time.
Just say what are you going to search. By default q searches for
these regular expressions presets in `$XDG_CONFIG_HOME/q/rules` (read
`$HOME/.config/q/rules`) but if you want you may always sets this dir
with `-r` flag.


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

and your binary safely sets into `target/release/q`. You may copy that
or symlink to the place you want (e.g `/usr/local/bin/q`). If you want
to install man pages, install them (`man/q.1` as man 1 page).

### HomeBrew

To install just do the following command in your terminal:

```bash
$ brew tap 9seconds/homebrew-q
$ brew install 9seconds/q/q
```


## Regexp files

By default q will search for regexp files in `$XDG_CONFIG_HOME/q/rules`. If you do not have `XDG_CONFIG_HOME` set, it would be `$HOME/.config/q/rules`. So if you do

```bash
$ q url,ipv4,ipv6 /var/log/httpd/*log
```

q will use following regexp files:

```bash
$XDG_CONFIG_HOME/q/rules/url
$XDG_CONFIG_HOME/q/rules/ipv4
$XDG_CONFIG_HOME/q/rules/ipv6
```

The structure of these files are rather simple: they just contain regular expressions as is. q uses PCRE for regular expressions, not RE2 or any other syntax so be sure that your regexps are PCRE compatible.

Just like an example of UUID4 file

```bash
$ cat $XDG_CONFIG_HOME/q/rules/uuid4
[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89aAbB][a-f0-9]{3}-[a-f0-9]{12}
```

So it is just a regexp inside. If you want to define several regular expressions inside one file it is possible, just put every regular expression on the new line. One line per regular expressions. And do not bother with pipes, q will assembly complete valid expression for you in a runtime.



## Examples

Let's imagine we have a test file like that

```bash
$ cat somefile.txt
Lorem ipsum http://google.com 127.0.0.1
e7f71ed0-77a3-42b2-b9f8-296816defd82 and the rest of the file
```

Let's run q with different options:

```bash
$ q url somefile.txt
Lorem ipsum http://google.com 127.0.0.1
```

```bash
$ q url,ipv4 somefile.txt
Lorem ipsum http://google.com 127.0.0.1
```

```bash
$ q -n url,ipv4 somefile.txt
somefile.txt:1  Lorem ipsum http://google.com 127.0.0.1
```

```bash
$ q -o url,ipv4 somefile.txt
http://google.com
127.0.0.1
```

```bash
$ q -no url,ipv4 somefile.txt
somefile.txt:1  http://google.com
somefile.txt:1  127.0.0.1
```

```bash
$ q -ol url,ipv4 somefile.txt
http://google.com 127.0.0.1
```


## Links

* [HomeBrew tap repository](https://github.com/9seconds/homebrew-q)
* [Some regexp sets](https://github.com/9seconds/q-regexps)

If you want to use my regexp set, just do

```bash
$ git clone https://github.com/9seconds/q-regexps $XDG_CONFIG_HOME/q/rules
```

or if you do not have `$XDG_CONFIG_HOME` set (if you have no idea what is this
then you do not have it set usually):

```bash
$ git clone https://github.com/9seconds/q-regexps $HOME/.config/q/rules
```
