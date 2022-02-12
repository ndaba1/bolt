# Bolt
Bolt is a command line tool built for managing multilingual projects. It was built out of necessity. I was managing a project organized into a monorepo such that each project was a different language: i.e Javascript, Dart, Elixir, Python and some more. It was getting hard to manage all the individual projects since you have to first `cd` into the said project directory and run your commands from there.

Bolt simplifies all this by acting as a "proxy" for all your commands. You perform all the actions at the root of the workspace using bolt and specify which app the action is meant for and bolt resolves the action and creates a child process which executes the said command. The following is a short primer on bolt and its usage:

```bash
    bolt up <app-name>?
    #the app-name is optional, if you dont provide it, you will be prompted for which app to start
    #the app command simply starts-up the specified application
    #each project contains a boltconfig.json file which has a priority field, it can be used as shown below:

    bolt up -p 1
    #if you specifiy the -p(priority) flag, bolt starts all apps with a priority that is less than or equal to the provided value
    #Other commands utilized in the workspace include:

    bolt test <app-name>? (-p ?)
    #same rules for priority apply, runs all configured tests
 
```

Bolt is written in Rust and as you can see, it's more of a task runner than a build tool. It doesnt override the native build tools (yet) but rather wraps around them providing a unified interface for running commands and thus managing multi-lingual projects in a monorepo. 


## Getting started with bolt
Setting up bolt on your machine is actually pretty easy. All you have to do is get the executable, save it to anywhere in your disk(preferably somewhere that doesn't require elevated privileges) and add the location of the bolt application/executable to your path.

- For windows, you can get the executable, [here](https://drive.google.com)
- For MacOs, you can install bolt from homebrew, by running 
  ```bash
  brew update

  brew install bolt
  ```
- For Linux systems, you can use your specific package manager to install bolt

## Bolt internals
Bolt requires two critical files for it to run correctly:
- A `bolt.json` file in the root of the workspace
  This file contains general info about the workspace. Such as the workspace name and where to find the projects. It may also contain a `registered` field which is required in the case that you want to use bolt to manage some and not all the projects in a given workspace. If the `registered` field is not passed, bolt scans all the projects in the workspace, checking for their config files.
- A `boltconfig.json` file in each of the projects individual directories. 
  The following is an example of a `boltconfig.json` found in an angular app:
  ```json
  {
  "info": {
    "name": "angular-app",
    "alias": "app",
    "priority": 0,
    "env": {
      "from": "angular",
      "cli": "ng",
      "pkg_mgr": {
        "value": "npm",
        "cmds": {
          "add": "install",
        }
      }
    }
  },
  "policies": [{
      "name": "up",
      "depends_on": "^",
      "map_to": {
        "value": "cli",
        "cmd": "serve"
      }
    },
    {
      "name": "install",
      "depends_on": "",
      "map_to": {
        "value": "pkg_mgr"
      }
    },
   ]
  }
  ```
  - `info > `
    - `name`: this corresponds to the name of the directory the project is in.
    - `alias`: an alias can be used to refer to the project without typing out the actual directory name.
    - `priority`: this field tells bolt how to treat the said project. For instance, if you run `bolt up -p 1`, bolt runs every app with a priority of 1. You can also run `bolt up -p <1` which will start all apps with a priority less than or equal to one.
    - `env > from`: refers to the highest level environment of the project, i.e In this example, the from is set to angular, which itself depends on a js environment, which requires node etc.
    - `env > cli`: some apps have a cli for their use and mgmt, such app @ng/cli or @vue/cli. You can specify the value here or it can be empty.
    - `env > pkg_mgr > value`: the package manager of the app, such as npm, yarn, cargo, flutter pub, mix etc.
    - `env > pkg_mgr > cmds`: This helps bolt know the available commands of the package manager 
  - `policies`: policies are the core of bolt and tell bolt how to map various commands, for instance, in the above example, running `bolt up app` will be mapped to `ng serve`
    - `name`: the name of the policy which corresponds to a bolt command, i.e bolt up
    - `depends_on`: this value tells bolt whether there are any commands that should be run before the current one. If the value is `^` or `install`, bolt will check dependencies before running the command.
    - `map_to`: this is where the command is routed to, either the cli or the pkg_mgr can be used

## Building bolt from scratch
To build bolt from scratch, first clone this repo to your local environment. Do note that you have to have an existing development environment for running rust code. The official guide for this can be found [here](https://rust-lang.org/guides). 

Assuming you have the necessary environment, all you have to do now is run:
```bash
cargo build
```

## Bolt features implementation: 

### Completed: 

- [x] Running bolt up `app-name` starts up the given app
- [x] Bolt can successfully redirect your commands from the root directory. i.e running `bolt angular-app ng g c navbar` will run the command in the project named `angular-app`
- [x] Bolt can resolve project aliases i.e, if your project has a long name such as `my_super_awesome_python_app`, you can add an alias for bolt to use by adding the field alias to your boltconfig.json 

### To be implented:

- [ ] Enable creation of a new workspace by running bolt init 
- [ ] Adding bolt to an existing monorepo by running bolt bootstrap
- [ ] Running configured tests by running bolt test
- [ ] Check and apply bolt updates by running bolt update