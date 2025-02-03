#!/bin/bash

echo "Enter directory name"
read dir_name

if [ -d "$dir_name" ]; then
  echo "Directory exist"
else
  mkdir "$dir_name"
  echo "Directory created"
fi