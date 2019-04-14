# `dep`: A dependency management library

This is an attempt to simplify the job of every single application or library
that does its own dependency management. No matter whether it is for packages,
services, extensions, modules, _whatever_ - this library aims to do that job for
it.

### How?
All common (and some uncommon) features have been implemented (hopefully!) over
the generic concept of a _target_ - a single entity with respect to dependency
management.

### Features
Some features (to be) implemented are:
* Features ("flags" for modifying the installation and building process of the
  target)
* Group identifiers (IDs matching multiple targets, e.g to allow choosing one of
  multiple dependencies)
* Build-time vs. run-time dependencies (a chain of run-time dependencies are
  allowed to have cyclic dependencies)
* Conflicts
* Safe failure in installation or removal (installations and removals are
  ordered in such a way that a failure does not break anything)

### Missing something
If some feature that could be considered generally relevant is missing, then
please go to the issues page! Feature requests are always welcome and will be
discussed thoroughly, and then hopefully added to `dep`.

### License: MIT
```
Copyright (c) 2019 ARaspiK

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
