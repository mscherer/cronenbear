A small rust program to merge holidays calendars

# History

At $work, one of my coworker spent time copying public holidays from
Google Calendar to our shared team calendar. Seeing that, I decided this could be
the job of a script and so I initially wrote a small CLI tool for that. 

Later, I decided that Python was not cool enough and wouldn't give me street cred
among my peers, so I rewrote the software in Rust with Axum and Askama.

While searching for a name, I had the idea to combine "bear", the mascot of our
departement, and "David Cronenberg", because there was a restrospective of his movies
in Paris which reminded me of the theme of merging in his filmography (The Fly, Dead Ringers,
eXistenZ to name a few), hence the punny name "Cronenbear".

# Installation

Just install a rust tool chain and build with `cargo run --release`. There is no support
for https, the tool listen in http on port 1107 and assume it is behind a reverse proxy dealing
with TLS.

You need to restart it from time to time to refresh the calendars from Google.

A Dockerfile is provided to let everything be built on Openshift.

# Adding a new alias

For now, the aliases are hardcoded in the file `data/aliases.toml`. To add a new one, please
submit a pull request. I may add support for a config file later.

# Adding a new country

As Google somehow decided to not use iso 3166 code for naming the calendars, the code
use a manually built lookup table. I added a few countries before I got bored, so
if a country is missing, the `get_google_id` function in `src/country_calendar.rs` need to be completed.

# Adding a new religion

If Google decide to add a new religious calendar, it need to be added in `src/religion_calendar.rs`, and
a corresponding emoji is likely needed in the same file.

# Missing features
* The documentation on the index page is sparse
* The design could be improved (see `templates/index.html`)
* Startup is slow as every calendar is fetched one by one
* There is no resilience against Google Calendar bugs, some caching should be added
* There is no refresh coded in the server except restarting, which is automatic on Openshift
* There should be a config file and not use a hardcoded toml file. Or even better, a configuration directory.
* There should be a way to have "private" calendar with a setting (requires separate config file), calendar that do not appear on index and can't be guessed easily.
