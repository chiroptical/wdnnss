wdnnss
---

"We don't need no stinkin' spaces!" - me, sometimes

```shell
$ tree
.
├── a directory with spaces
│   ├── a file with spaces.txt
│   └── another file with spaces.txt
```

First of all, oof. I think we can all agree that spaces in filenames is a
mistake. If we can't, stop reading and find some other library. This
command line utility will convert the above structure to:

```
$ wdnnss --seperator=_ .
$ tree
.
├── a_directory_with_spaces
│   ├── a_file_with_spaces.txt
│   └── another_file_with_spaces.txt
```
