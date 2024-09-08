
## Usage
Get usage from command line
```shell
remind --help
remind -h
```
Create a new list of events
```shell
remind --new "daily" "1234" # private list
remind -n "daily" # public list 
remind -n "daily" "1234"
```
Check all available lists
```shell
remind --lists
remind -ls
```
Load a list
```shell
remind --load "daily"
remind -ld "daily"
```
Add new event
```shell
remind --add "title" "details" "yyyy-mm-dd" "hh:mm"
remind --with-level 1 "title" "details" "yyyy-mm-dd" "hh:mm"
remind --wl 1 "title" "details" "yyyy-mm-dd" "hh:mm"
remind -a "title" "extra content" "yyyy-mm-dd" "hh:mm"
remind -a "title" "extra content" "yyyy-mm-dd" 
remind -a "title" "extra content" 
remind -a "title"
```
Check all available events
```shell
remind --events 
remind -es
```
Check out the finished events. You need to list all events first to get the ID of them.
```shell
remind --finish 1 3 4 6
remind -f 1 3 4 6
```

