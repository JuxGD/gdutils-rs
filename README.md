# `gdutils`

`gdutils` is a Rust crate intended used to fetch all sorts of data from the Geometry Dash servers.

This project is still a WIP! Contrubutions are appreciated. Check the To-do section!

Originally made for the FluxDash bot for Fluxer.

## To-do list / What to help with

Current (or future) crate features:

- [ ] Getting arbitrary level info (`get_level_info()`)
  - [X] Name
  - [X] ID
  - [X] Author
  - [X] Song ID
  - [ ] Song artist - song name (Newgrounds/Music Library)
  - [X] Difficulty rating
  - [X] "Quality" status (normal, featured, epic etc) and feature score (the higher that is, the higher in the featured tab the level is)
  - [X] Stars/Moons
  - [X] Coins (and if verified)
  - [X] Likes
  - [X] Downloads
- [X] Getting arbitrary level (`get_level()`)
- [X] Getting daily, weekly level indes & time left (`get_daily()`, `get_weekly()`) 
- [X] Getting event level index (`get_event()`)
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
- [X] Generating icons from icon data

## AI, Agents, LLMs

Please refer to llms.txt, llms-full.txt, AGENTS.md or CLAUDE.md, and CONTRIBUTING.md. Don't look at the README anymore. Go on check the files I said

:trollface:
