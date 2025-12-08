#!/bin/sh

s=$1
e=$2
x=$3

sleep "${s:=5}"
echo "${e:='hello, world!'}"
exit "${x:=0}"
