# Khnum

[![Build Status](https://travis-ci.org/mmai/activue.svg?branch=master)](https://travis-ci.org/mmai/activue) [![Coverage Status](https://coveralls.io/repos/github/mmai/activue/badge.svg?branch=master)](https://coveralls.io/github/mmai/activue?branch=master)

Experiments with Actix and Vuejs (WIP)

## Start a new project based on Khnum

Install framework
```sh
curl https://raw.githubusercontent.com/mmai/khnum/master/bin/khnum-new.sh | bash -s myproject
```
Init postgresql database (with docker)

```sh
cd myproject
make initdb
make migrate
```

Start backend server
```sh
make run
```
Start frontend developpement server

```sh
make frontrun
```

## TODO

* users management
  * [x] registration
  * [ ] personal pages
* [ ] i18n
* [ ] basic crud admin
* [ ] activitypub
* [ ] CQRS
