# C Package Index

> Just a simple cli tool to manage packages in c
> (kinda like pip)

## Todo
- Use TOML to store metadata
- Make code rustier
- Implement version tracking (ooo scary)
- Maybe upload (instead of me manually moving files)
- Get it on a server (currently it works locally)

## Motivation
- im bored af
- why not?


## why in rust??
*the memes* <br>
(to lazy to implement HashSet) <br>

```
/module-name
    /0.1.0 # old version number
        module-name.c # old files
        module-name.h
        module-name.so
        metadata.toml
    # Current Version
    module-name.c # C source file
    module-name.h # Header source file
    module-name.so # Shared Object file
    metadata.toml
```
    