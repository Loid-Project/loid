# Loid

Loid is an in-works programming language integrating Martin-Lof's Type Theory into an OOP approach.

Loid compiles to JavaScript.

## Book

Loid is currently writing a full documentation to describe every aspect of the language before proper development is begun.
This is as to avoid any issues after the language is developed.

You can see the progress in the <a href="docs/">docs</a> folder.

### Chapters

- <a href="docs/book/C0-Syntax.md">Chapter 0 - Syntax</a>: Explains the basics of Loid's syntax, functionality, tricks, and tips.
- <a href="docs/book/C1-Types.md">Chapter 1 - Types</a>: Describes Loid's type system, how it integrates OOP, and how it works with dependent types.
- <a href="docs/book/C2-Functions.md">Chapter 2 - Functions</a>: Goes into depth about Loid's functions, how they're typed, language caveats, tags, and compilation caveats.
- <a href="docs/book/C3-PatternMatching.md">Chapter 3 - Pattern Matching</a>: Explains how Loid's pattern matching on structs, enums, functions, and other objects works.
- <a href="docs/book/C4-Strings.md">Chapter 4 - Strings</a>: Strings in Loid have multiple benefits over JavaScript strings, and are rather complex. This chapter goes into detail.
- <a href="docs/book/C5-Underscore.md">Chapter 5 - Underscore</a>: The underscore `_` keyword in Loid and its many use cases.
- <a href="docs/book/C6-StandardLibrary.md">Chapter 6 - Standard Library</a>: Explains the standard library (in some depth), how it works, how to swap it out, its modules, and so on.
- <a href="docs/book/C7-ErrorHandling.md">Chapter 7 - Error Handling</a>: This chapter focuses on Loid's monadic error handling system.
- <a href="docs/book/C8-FileStructure.md">Chapter 8 - File Structure</a>: Explains Loid projects' file structure and management.
- <a href="docs/book/C9-Namespaces.md">Chapter 9 - Namespaces</a>: Explains how namespaces work in Loid.
- <a href="docs/book/C10-Async.md">Chapter 10 - Async</a>: Explains the `async` `await` system in Loid, compared to JavaScript.
- <a href="docs/book/C11-Testing.md">Chapter 11 - Testing</a>: Regarding Loid's unit/integration testing system that is built into the language.
- <a href="docs/book/C12-ForeignFunctions.md">Chapter 12 - Foreign Functions</a>: Explains headers and how to use them to import foreign `C`, `TypeScript`, and `JavaScript` functions.
- <a href="docs/book/C13-Packages.md">Chapter 13 - Packages</a>: Explains the Loid package manager, how to create Loid packages, change package repositories, add package repositories, and so on.
- <a href="docs/book/C14-Comptime.md">Chapter 14 - Comptime</a>: This chapter talks about the compile-time computations that happen in Loid. This will be elaborated upon greatly in every following chapter.
- <a href="docs/book/C15-TypeRepAndKernel.md">Chapter 15 - `TypeRep` and the Kernel</a>: Explains the Loid type kernel and the `TypeRep` type.
- <a href="docs/book/C16-UtilityTypes.md">Chapter 16 - Utility Types</a>: Explains the standard library utility types in Loid in detail (with examples), and explains how to write them.
- <a href="docs/book/C17-VarianceAnnotations.md">Chapter 17 - Variance Annotations</a>: Explains in detail Loid's type variance system "for dummies" (simply, not assuming prerequisites).
- <a href="docs/book/C18-Typestating.md">Chapter 18 - Typestating</a>: Elaborates on how typestating can work in Loid, for OOP, structs, and tagged union types.
- <a href="docs/book/C19-Concurrency.md">Chapter 19 - Concurrency</a>: Goes into depth about concurrent programming in Loid beyond just `async` and `await`.
- <a href="docs/book/C20-JavaScriptInterop.md">Chapter 20 - JavaScript Interop</a>: Explains how to interoperate with JavaScript codebases.
- <a href="docs/book/C21-TypeScriptInterop.md">Chapter 21 - TypeScript Interop</a>: Explains how to interoperate with TypeScript codebases.
- <a href="docs/book/C22-CompilationModel.md">Chapter 22 - Compilation Model</a>: Discusses Loid's compilation model (advanced, not meant for people just looking to use Loid for simply development).
- <a href="docs/book/C23-Metaprogramming.md">Chapter 23 - Metaprogramming</a>: Explains metaprogramming in Loid, the `block` type, macros, comptime computations and interpreter injections.
- <a href="docs/book/C24-QuantumComputing.md">Chapter 24 - Quantum Computing</a>: Loid supports emulating a quantum computer with its standard library `q` module and the `qbit` type. This chapter explains in depth how that works.
- <a href="docs/book/C25-Migration.md">Chapter 25 - Migration</a>: Explains how to migrate a JavaScript codebase to Loid.
- <a href="docs/book/C26-StyleGuide.md">Chapter 26 - Style Guide</a>: How the language should generally be styled, organized, and written. Guidelines, rather than rules.

## Contributing

Any contributions are welcome, whether they be for documentation, ideas, any bug-fixes, issues found, new features, or anything else.
Your ideas are always appreciated.

Feel free to open any issues asking any questions or suggesting ideas.

Please read <a href="CONTRIBUTING.md">CONTRIBUTING.md</a> before beginning to work on the codebase and/or making a pull request.

## Team

The Loid core team is currently made up of:
<a href="https://github.com/KrakenTheJaken">logan</a>, <a href="https://github.com/dagworth">larry</a>, and <a href="https://github.com/givikuna">givi</a>

## Sponsors

Loid is currently sponsored and maintained, in-part by the <a href="http://amphiseum.dev/">Amphiseum</a> corporation.
For any legal issues, contact the Amphiseum team, per their website.

If any organization, team, or company wants to contribute, sponsor, or help in any form maintain, please contact <a href="https://github.com/givikuna">Givi Tsvariani</a>.

## AI Policy

Loid does allow for AI-assisted code, but any issue, pull request, or code published or pushed to this codebase must be verified by a human and sent by a human.

AI-assisted code will still be expected to hold up to our standards, be well written, and tested.
We ask that you cite your AI sources when used and how it was used.

Code that is clearly fully AI generated, or written and then pushed to the codebase by an autonomous AI agent will not be accepted.

## Issues

...

## Security

Follow the guidelines in <a href="SECURITY.md">SECURITY.md</a>.

## LICENSE

The Loid programming language is licensed under the GNU General Public License version 3.0.

## COPYRIGHT

THe Loid language's copyright is owned by the Loid Project, and its creators.
