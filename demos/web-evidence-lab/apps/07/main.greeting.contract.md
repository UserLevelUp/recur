# main.greeting
            Defines: main.greeting.name trimmed nonempty name, at most 40 characters.
            Defines: main.greeting.message plain text, localized at level 05 onward.
            Input must render as text, never executable markup.
            API errors become visible at level 08; level 09 retries once and caches per name and locale.
