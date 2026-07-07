# woutline

Read and **w**rite document **outline**s from text files.

```
$ woutline r test.pdf
wrote 4 outline entries to test~OUTLINE.txt
$ cat test~OUTLINE.txt
1 Test document
    1 Introduction
        3 Examples
    5 Chapter 2
$ echo "    10 Chapter 3" >> test~OUTLINE.txt
$ woutline w test.pdf
```
