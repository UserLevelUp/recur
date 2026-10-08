# Website test baseline

defines: demo.blackjack.web.baseline recorded contract-first failure observations
consumer: demo.blackjack.web.requirements bounded single-player website

Observed 2026-10-02 before the corresponding implementation stages:

- Engine baseline: test suite could not load the not-yet-created engine file.
- HTTP baseline: 8,601 engine assertions passed; server file did not yet exist.
- Browser assets baseline: 3 passed, 9 failed, 1 errored because HTML/CSS/JS
  assets were absent. These are scaffold failures, not pre-existing game bugs.
- Correspondence baseline: 21 passed, 2 failed, 1 errored. The failures exposed
  an incomplete public-output field bundle and wrong handler namespace; the
  error was the intentionally not-yet-recorded source review observation.

Raw baseline logs are in `observations`. The references and tests came first for
the engine, HTTP server and browser assets. The CIR dependency reference is a
retrospective review model added once the source existed. Do not infer that the
entire website was automatically generated or proved correct by the language.
