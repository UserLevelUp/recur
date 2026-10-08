artifact.type = lane
publish: main.command.watch.eventness.tests observed expected-red native baseline
trigger: main.command.watch.eventness.implementation implement frozen native interfaces

Eleven explicitly selected native conformance tests ran and failed at proposed topic creation because the current CLI reports unrecognized subcommand topic. No downstream native semantic assertion was reached. Four setup tests independently passed. See observations/baseline.json, native-red.txt and scaffold.txt for exact runs, exit codes, source and binary fingerprints. This accepts an observed unimplemented-interface baseline only; implementation, failure-injection/recovery, affected Rust/Julia regressions and source-bound final review remain pending. The red suite is not added to ordinary regressions or any demo selection.
