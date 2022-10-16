#!/bin/bash

trunk build --release
rm /var/www/minesweeper/*
cp -R dist/. /var/www/minesweeper/
