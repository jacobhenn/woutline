# woutline

Read and **w**rite document **outline**s from text files. Currently, only PDFs are supported, and the PDF library used is kinda jank, so use at your own risk.

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
