#! /usr/bin/bash

pkg install dart python
echo '>>>>>>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<<<<'
mkdir $PREFIX/bin/dart-repl-file
cp dart-cli.py $PREFIX/bin/dart-repl-file
cp dart-repl $PREFIX/bin
cd $PREFIX/bin/
chmod +x dart-repl
echo '>>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<<<<<<<'
echo ok
echo '>>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<<<<<<<'
echo type dart-repl to run
echo '>>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<<<<<<<'