#!/bin/bash

git checkout main
git pull
trunk build --release
rm /var/www/minesweeper/*
\cp -R dist/. /var/www/minesweeper/
