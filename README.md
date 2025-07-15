A small rust program to merge calendars

# History

At $work, one of my coworker spent time copying public holiday from
Google calendar to the team calendar. Seeing that, I decided this could be
the job of a script and so I wrote a python script for that. 

later, I decided that python was not cool enough and wouldn't give me street cred
among my peers, so I rewrote the software in rust.

While searching for a name, I had the idea to combine "bear", the mascot of our
departement, and "David Cronenberg", because there was a restrospective of his movies
in Paris which reminded me of various body horror themes in his filmography (like, The Fly),
hence cronenbear.

# Adding a new alias

The aliases are hardcoded in the file `data/aliases.toml`. To add a new one, please
submit a pull request.

# Adding a new country

As Google somehow decided to not use iso 3166 code for naming the calendar, we have
to do a lookup table manually. I just used a few countries before I got bored, so
to add a new one, the `get_google_id` in `src/country_calendar.rs` need to be completed.

# Missing features
* Religious holidays are not yet supported (code is almost here)
* The documentation on the index page is sparse
* The design could be improved (see `templates/index.html`)
* Startup is slow as every calendar is fetched one by one
* There is no resilience against Google Calendar bugs, some caching could be added
* There is no refresh coded in the server except restarting, which is automatic on Openshift
* There should be more information about each holiday, starting by the country
