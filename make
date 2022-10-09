#!/bin/bash

trunk build --release
cp -R dist/ /var/www/minesweeper/
