# ZPL to PDF

This project will hopefully eventually turn into a ZPL drawer into PDFs.  
It is a mono repo with multiple packages, some of these might be useful for when
you want to do anything with ZPLs.

## Structure

- ./zpl-tokenizer - A tokenizer built for lossy tokenization of the ZPL format

## Todo

- [x] ZPL tokenizer - A tokenizer for ZPL
- [x] ZPL Parser - A parser for ZPL which will take the tokens from the tokenizer
and turn them into command specific structures.
- [x] ZPL State Machine - Stateful processor that tracks ZPL settings and defaults,
converting parsed commands into renderer-agnostic draw operations.
- [ ] PDF ZPL drawer - Draw ZPLs into a pdf file based on the commands from the parser.
- [ ] REST api - REST api that uses above libraries to take in a ZPL and respond
a PDF (goal: sub 15ms processing time). Could also expose draw operations as JSON
for custom renderers.
- [ ] CLI tool - Command Line Interface for turning ZPLs into PDFs.

## Design choices

To keep a project like this maintainable we must have a clear seperation of concern.  
To achieve this we split this project into multiple semi independent libraries.  
Every library of the core pdf renderer is part of a pipeline which is the following:

- tokenizer
- parser
- State manager
- PDF renderer

Each step in this pipeline may depend on the one before it. But must not expose anything of the previous step to the next step.
