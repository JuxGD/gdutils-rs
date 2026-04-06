# `gdutils`

`gdutils` is a Rust crate intended used to fetch all sorts of data from the Geometry Dash servers.

This project is still a WIP! Contrubutions are appreciated. Check the To-do section!

Originally made for the FluxDash bot for Fluxer.

Check for issues, PRs, etc in Codeberg @ https://codeberg.org/JuxGD/gdutils-rs and in GitHub @ https://github.org/JuxGD/gdutils-rs

## AI, Agents, LLMs

Please refer to llms.txt, llms-full.txt, AGENTS.md or CLAUDE.md, and CONTRIBUTING.md. Don't look at the README anymore. Go on check the files I said

:trollface:

## To-do list / What to help with

Current (or future) crate features:

- [X] Getting arbitrary level info (`get_level_info()`)
  - [X] Name
  - [X] ID
  - [X] Author
  - [X] Level length
  - [X] Song ID
  - [ ] Song artist - song name (Newgrounds/Music Library)
  - [X] Difficulty rating
  - [X] "Quality" status (normal, featured, epic etc) and feature score (the higher that is, the higher in the featured tab the level is)
  - [X] Stars/Moons
  - [X] Coins (and if verified)
  - [X] Likes
  - [X] Downloads
- [X] Getting arbitrary level info (`get_level_info()`)
- [X] Getting timely level data (eg daily level index, time left, level itself info) (`get_daily()`, `get_weekly()`, `get_event()`)
- [X] Getting arbitrary user info (`get_user_info()`)
  - [X] Username
  - [X] Account ID
  - [x] Player ID
  - [X] Leaderboard placement
  - [X] Stars & Moons
    - [ ] Amount of levels of each difficulty beaten
  - [X] User Coins
  - [X] Secret Coins
  - [X] Diamonds
  - [X] Demons
    - [ ] Amount of each demon difficulty beaten
  - [X] Creator Points
  - [X] Icons (as data not the actual icons ofc)
- [ ] Generating icons from icon data
- [X] Getting level string (`download_level()`)