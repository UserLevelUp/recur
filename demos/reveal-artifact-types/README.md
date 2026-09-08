# Typed reveal artifacts

This fixture demonstrates shared discovery without activating anything:

```powershell
recur reveal -d demos/reveal-artifact-types
recur reveal --type skill -d demos/reveal-artifact-types
recur reveal persona.skippy -d demos/reveal-artifact-types
recur tree skill -d demos/reveal-artifact-types --sep .
```

All three capsules explicitly declare their type. No configuration or SKILL.md
file is required. For optional prefix hints see the reveal command documentation.
The skill has a separate current-work artifact to illustrate Eventness around
the same subject. Queries only read and report these files.
