# C Package Index

> Just a simple cli tool to manage packages in c
> (kinda like pip)

## Todo
- Make code rustier
- Implement version tracking (ooo scary)
- Maybe upload (instead of me manually moving files)
- Get it on a server (currently it works locally)
- Maybe MAYBE move to JSON (i don't even know why I didn't do it in the first place but im way to tired for this right now so..........







idk)

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
        /dependencies # etc
            depends
            on
            this
    # Current Version
    module-name.c # C source file
    module-name.h # Header source file
    module-name.so # Shared Object file
    version # File that holds the current version
    /dependencies # Directory with modules that this depends on
        modules
        that
        this
        module
        depends
        on
```
    