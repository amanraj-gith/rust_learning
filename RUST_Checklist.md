# Rust → Backend Job Switch Tracker

**Starting date:** *fill this in* **Target application month:** *fill this in (start date + ~4 months)* **Weekly hour budget:** 15 hrs/week (2 hrs × 4 weekday mornings + 6-7 hrs across weekend)

---

## How to use this tracker

- Check things off as you go — `- [ ]` becomes `- [x]`
- Do a 15-min retro every Sunday using the template at the bottom
- Missed a day? Don't try to "catch up" — just resume. Consistency > cramming
- If a week takes longer, slide the schedule. Don't skip depth to stay on calendar
- Every artifact (repo, blog post, PR) gets added to the **Artifact Log** at the bottom

---

## Ground rules (read before starting)

1. **No copy-paste of code.** Type every example from the book yourself. Muscle memory matters for interviews.
2. **The borrow checker is not your enemy.** When it yells, read the error slowly. Rust errors are famously helpful.
3. **Publish everything.** Every project goes on GitHub the day you start it, not when it's "done." Public commits build the story.
4. **Morning brain > evening brain.** Do the hard focused work (book chapters 4, 10, 15, 16; new project features) in morning slots. Evenings for videos, blog drafts, LeetCode.
5. **Ship > perfect.** A rough README beats no README. A deployed project beats a "polished" localhost demo.

---

# PHASE 1 — Language Foundation (Weeks 1-4)

**Goal:** Understand Rust deeply enough that ownership, borrowing, lifetimes, traits, and async don't scare you.

**End-of-phase artifact:** Rustlings 100% complete + 15 LeetCode problems solved in Rust, all pushed to a public GitHub repo called `rust-learning`.

---

## Week 1 — Setup + Book Ch 1-3

**Learning:**

- Install Rust via rustup: [https://rustup.rs](https://rustup.rs)
- Install VS Code + rust-analyzer extension (or your editor of choice)
- Set up GitHub repo `rust-learning` — push daily
- Book: Chapter 1 (Getting Started) — do the guessing game *from scratch* without looking
- Book: Chapter 2 (Guessing Game) — you already have notes; re-do it typing every line
- Book: Chapter 3 (Common Programming Concepts) — variables, data types, functions, control flow
- Rustlings: `intro`, `variables`, `functions`, `if` sections

**Milestone:** Repo has 7 daily commits. Can explain shadowing vs mut without notes.

**Time budget:** 15 hrs

---

## Week 2 — Book Ch 4 (Ownership) — SLOW DOWN HERE

This is the most important week of the whole plan. Do not rush it.

**Learning:**

- Book: Chapter 4 — Ownership, References & Borrowing, Slices
- Watch: Jon Gjengset "Crust of Rust: Smart Pointers and Interior Mutability" (YouTube) — even if you don't fully get it, first exposure helps
- Rustlings: `move_semantics`, `primitive_types`, `strings`, `vecs`
- Read the same chapter twice. Yes, twice.
- Write 5 small programs that intentionally cause borrow checker errors, then fix them. Write comments explaining *why* each error happened.

**Milestone:** Can rubber-duck-explain: why `String` moves but `i32` copies, why you can't have `&mut` and `&` at the same time, what a slice actually is.

**Time budget:** 15-18 hrs (worth going over)

---

## Week 3 — Book Ch 5-7 + Ch 8

**Learning:**

- Book: Chapter 5 (Structs)
- Book: Chapter 6 (Enums + `match`)
- Book: Chapter 7 (Packages, Crates, Modules)
- Book: Chapter 8 (Common Collections: Vec, String, HashMap)
- Rustlings: `structs`, `enums`, `modules`, `hashmaps`
- LeetCode: 3 easy problems in Rust (Two Sum, Valid Parentheses, Reverse Linked List)

**Milestone:** You can model a small domain (a library book system, a chess board) in structs + enums without help.

**Time budget:** 15 hrs

---

## Week 4 — Book Ch 9-10 + Error handling deep dive

**Learning:**

- Book: Chapter 9 (Error Handling — `Result`, `?` operator, panics)
- Book: Chapter 10 (Generics, Traits, Lifetimes) — SLOW DOWN, this is #2 hardest chapter
- Rustlings: `error_handling`, `generics`, `traits`, `lifetimes`
- Watch: Jon Gjengset "Crust of Rust: Lifetime Annotations" (YouTube)
- LeetCode: 5 easy problems in Rust
- Read: `anyhow` and `thiserror` crate docs on docs.rs

**Milestone:** Can explain when to use `anyhow` vs `thiserror` and why. Can write a function with a generic + trait bound + lifetime without googling.

**Time budget:** 15-18 hrs

---

## Weekend of Week 4 — Consolidation

- Finish remaining Rustlings sections you skipped
- Push everything to `rust-learning` repo with a proper README
- Write a Google Doc or Notion note: "What I still don't understand about Rust" — you'll revisit this in Week 8

---

# PHASE 2 — The Three Projects (Weeks 5-12)

**Goal:** Build the portfolio that gets you interviews. Each project = 1 GitHub repo + 1 blog post + 1 LinkedIn post.

**End-of-phase artifacts:**

- Published crate on crates.io
- Deployed REST API accessible via public URL
- Systems project with performance numbers in the README

---

## Project 1: CLI Tool (Weeks 5-6)

### Scope

Pick ONE (don't do all three):

- **`git-cleanup`** — CLI that lists local git branches merged into main and offers to delete them. Bonus: detects branches not pushed for N days.
- **`logfilter`** — CLI that tails a log file and filters lines by regex, log level, or time window. Bonus: colored output.
- **`dirstat`** — Analyze a directory tree: total size, largest files, file type breakdown, empty folders. Bonus: JSON output mode.

### Tech stack

- `clap` for argument parsing
- `anyhow` for error handling
- `serde` + `serde_json` if you add JSON output
- `regex` if needed
- `colored` for terminal colors

### Week 5

- Pick project. Write a 1-page README FIRST — problem, features, example commands
- `cargo new` the project. Add dependencies
- Implement core feature (happy path only)
- Push to GitHub daily

### Week 6

- Add error handling for edge cases (missing files, permissions, invalid input)
- Write at least 5 unit tests + 2 integration tests
- Polish README with GIF or asciinema of it running
- **Publish to crates.io** — follow [https://doc.rust-lang.org/cargo/reference/publishing.html](https://doc.rust-lang.org/cargo/reference/publishing.html)
- Write blog post: "Building my first Rust CLI: what I learned"
- LinkedIn post announcing the crate

**Milestone:** Someone can run `cargo install <your-crate>` and use your tool.

---

## Project 2: REST API with Axum (Weeks 7-9)

### Scope

Pick ONE — pick something with real domain logic, not a todo app:

- **Habit tracker API** — users, habits, daily check-ins, streak calculations, weekly stats
- **Expense splitter** (mini Splitwise) — users, groups, expenses, debt simplification algorithm
- **Bookmark manager** — users, bookmarks with tags, full-text search, browser extension API

### Tech stack (non-negotiable — this is what employers see)

- `axum` — web framework
- `tokio` — async runtime
- `sqlx` — Postgres with compile-time checked queries
- `serde` — JSON serialization
- `tracing` + `tracing-subscriber` — structured logging
- `thiserror` — typed errors
- `jsonwebtoken` — JWT auth
- `argon2` — password hashing
- `tower-http` — middleware (CORS, tracing)
- `testcontainers` — integration test infra
- Docker + docker-compose for local Postgres

### Week 7

- Design API — write out endpoints in the README before coding. Use REST conventions.
- Set up project skeleton: `axum` handlers, `sqlx` pool, error type, config from env
- Docker-compose with Postgres running locally
- Implement auth: register, login, JWT middleware
- First 2-3 CRUD endpoints
- Push daily

### Week 8

- Rest of endpoints — the actual domain logic
- Structured logging with `tracing` (request IDs, spans)
- Migrations via `sqlx migrate`
- Integration tests with `testcontainers` — real Postgres, not mocks
- Revisit "What I still don't understand" note from Week 4. Cross out things you now get.

### Week 9

- Rate limiting middleware (use `tower-governor` or write your own)
- OpenAPI spec via `utoipa` (optional but impressive)
- Deploy to Fly.io or Railway with a public URL
- README: architecture diagram (use [https://excalidraw.com](https://excalidraw.com)), API docs, run instructions, deployment steps
- Blog post: "Designing errors in a Rust web service" or "sqlx compile-time queries: what surprised me"
- LinkedIn post with the live URL

**Milestone:** You can share `https://your-project.fly.dev` in interviews. Someone can hit your endpoints from Postman.

---

## Project 3: Systems / Concurrent Project (Weeks 10-12)

### Scope

This is the project that separates you from the pack. Pick ONE:

- **Mini Redis clone** — In-memory KV store with TTL, SET/GET/DEL/EXPIRE/INCR commands, simple text wire protocol over TCP. Multiple concurrent clients via tokio.
- **Rate limiter service** — Standalone service exposing token bucket + sliding window algorithms via HTTP. Backed by an in-memory store with proper synchronization.
- **Job queue** — Postgres LISTEN/NOTIFY-based job queue: enqueue jobs via HTTP, worker processes pick them up, retries with exponential backoff, dead-letter queue.

### Week 10

- Design doc in the README: what problem, what protocol/API, what data structures, what concurrency model
- Read: `tokio` tutorial ([https://tokio.rs/tokio/tutorial](https://tokio.rs/tokio/tutorial)) — all sections
- Implement core happy path
- Push daily

### Week 11

- Concurrency: multiple clients / workers running simultaneously
- Proper shutdown handling (Ctrl+C should not corrupt state)
- Benchmarking — use `criterion` for microbenchmarks or a simple loop for throughput. Get real numbers.
- Handle edge cases: what if a client disconnects mid-request? What if the DB is down?

### Week 12

- Write a stress test — spawn 1000 concurrent clients and see what breaks
- README: architecture, protocol spec, benchmark numbers (ops/sec, p50/p99 latency), what you'd do differently at scale
- Blog post: the deepest one yet — "What I learned building a mini Redis in Rust" with actual insights
- LinkedIn post + share in r/rust

**Milestone:** Your README has real numbers. "Handles 50k ops/sec with p99 of 2ms on my laptop" is a sentence a hiring manager remembers.

---

# PHASE 3 — Visibility + Interview Prep (Weeks 13-16)

**Goal:** Be visible in the Rust community + interview-ready.

## Ongoing every week in this phase

- 1 blog post per week (7-8 total by end)
- 3-4 LeetCode problems (any language)
- 1 hour of SQL practice (selectstarsql.com, then [https://pgexercises.com](https://pgexercises.com))
- 1 ByteByteGo system design video + take notes
- Engage in r/rust and Rust Discord — help beginners, ask smart questions

## Week 13 — Blog series ramp up

Blog post ideas (pick from these across weeks 13-16):

- "Why my Axum handler wouldn't compile: understanding Send + Sync"
- "sqlx vs diesel: choosing an ORM for a small project"
- "anyhow vs thiserror: when to use which"
- "async in Rust for people who know async in JavaScript"
- "Understanding Pin and why async traits were hard"
- "Deploying a Rust web service on Fly.io: a walkthrough"
- "Fighting the borrow checker: 5 patterns I use daily"
- "What I wish I knew before starting Rust"

Cross-post on dev.to and personal blog (Hashnode/Substack/self-hosted).

## Week 14 — Open source contribution

- Pick a crate you use: axum, sqlx, tokio, clap, tracing, serde, reqwest
- Read their CONTRIBUTING.md
- Find issues tagged `good first issue` or `help wanted`
- Comment on one saying you'll take it
- Submit PR by end of week (even if it's a doc fix — merged is merged)

## Week 15 — Interview drills

- Re-read Book Chapter 4 (ownership) and Chapter 10 (traits/lifetimes)
- Re-read Book Chapter 15 (smart pointers) and Chapter 16 (concurrency)
- Read the async chapter (Ch 17 in newer editions) if you skipped
- Practice Rust interview questions — search "Rust interview questions site:reddit.com/r/rust"
- Do 2 mock interviews on Pramp or with someone from Rust Discord

## Week 16 — System design + SQL heavy week

- Watch ByteByteGo: URL shortener, rate limiter, chat system, news feed, top-K
- Practice: design Twitter/Uber/Instagram at whiteboard level
- SQL: window functions, CTEs, indexes, EXPLAIN plans
- Read: [https://use-the-index-luke.com](https://use-the-index-luke.com) (skim, don't memorize)

**End-of-phase milestone:** 7-8 published blog posts, 1 merged OSS PR, interview-ready fundamentals.

---

# PHASE 4 — Applications (Weeks 17-20)

**Goal:** 20-30 high-quality applications → 5+ interviews → 1-2 offers.

## Week 17 — Resume + LinkedIn overhaul

- Rewrite resume: lead with projects, not current job. Include GitHub links.
- LinkedIn headline: "Backend Engineer — Rust, Axum, Postgres | Building [project 3 name]"
- LinkedIn About section: your Rust journey in 3 paragraphs. Link to blog + GitHub.
- Get resume reviewed by 2 people — one Rust dev, one recruiter friend
- Update GitHub profile README with pinned projects

## Week 18 — Target list

**India-based (Bangalore-friendly):**

- Cloudflare India
- CRED (specific teams)
- InMobi
- Fyllo
- Rippling India
- Turing (as a placement)

**Remote-open to India:**

- Fermyon
- Deepgram
- 1Password
- Astral (uv, ruff)
- Discord
- Sentry
- Vercel
- Deel

**Ongoing sources — check weekly:**

- wellfound.com filtered "Rust" + "Remote"
- this-week-in-rust.org Jobs section (published every Wednesday)
- r/rust monthly hiring thread
- "Who's hiring" on Hacker News (first day of each month)
- filtra.io (Rust-specific job board)

Add to spreadsheet: Company | Role | Source | Date applied | Status | Notes

## Week 19 — Applications wave 1

- 10-12 applications, each with a personalized note
- For each: read their engineering blog, mention something specific, link to your most relevant project
- Reach out to 3 engineers at target companies on LinkedIn (not asking for referral — asking a smart question about their team)
- Follow up on any radio silence after 7 days

## Week 20 — Applications wave 2 + interview loops start

- 10-12 more applications
- Interview prep: for each company, read glassdoor + team blog, prep 3 questions to ask them
- Track every interview: what was asked, what you fumbled, what to study next

**Beyond week 20:** keep applying and interviewing. Most people who do this get their offer between weeks 22-30. Do not panic if week 20 doesn't produce one.

---

# Resources (bookmark these)

## Learning

- The Rust Book: [https://doc.rust-lang.org/book](https://doc.rust-lang.org/book)
- Rustlings: [https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
- Rust by Example: [https://doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example)
- Jon Gjengset's Crust of Rust (YouTube): deep-dive videos
- Zero To Production In Rust by Luca Palmieri (paid book, worth every rupee for Project 2)

## Async / Backend

- Tokio Tutorial: [https://tokio.rs/tokio/tutorial](https://tokio.rs/tokio/tutorial)
- Axum docs + examples: [https://github.com/tokio-rs/axum/tree/main/examples](https://github.com/tokio-rs/axum/tree/main/examples)
- sqlx README + docs

## Community

- r/rust (Reddit)
- Rust Users Forum: [https://users.rust-lang.org](https://users.rust-lang.org)
- Tokio Discord
- This Week in Rust: [https://this-week-in-rust.org](https://this-week-in-rust.org) (subscribe by email)

## Interview prep

- LeetCode (any language)
- selectstarsql.com — SQL
- pgexercises.com — SQL
- ByteByteGo YouTube — system design
- [https://github.com/donnemartin/system-design-primer](https://github.com/donnemartin/system-design-primer)

## Deployment

- Fly.io — free tier, great for Rust
- Railway — also good
- Shuttle.rs — Rust-native deployments (worth trying)

---

# Weekly retrospective template

Copy this into a new note every Sunday evening. Takes 15 minutes.

```
## Week N retro

### What I learned
- - - ### What I built / shipped
- - ### What I got stuck on
- - ### What I'll do differently next week
- - ### Confidence check (1-10) in:
- Ownership / borrowing:
- Traits / generics / lifetimes:
- Async / tokio:
- Axum / building a service:
- Talking about my projects in an interview:

### Blocker to flag:
```

If you rate below 6 on anything by week 12, add a "remediation" week before moving to Phase 3.

---

# Artifact log

Fill this in as you go. This becomes the spine of your resume + interviews.

## GitHub repos

- `rust-learning` — daily commits, book exercises, Rustlings
- Project 1: *name* — link:
- Project 2: *name* — link:
- Project 3: *name* — link:

## Published crates

- Crate name: _______ | crates.io link: _______

## Blog posts (aim for 7-8)

## Open source PRs

- Crate: _____ | PR link: _____ | Status: _____

## Deployed services

- Project 2 URL:
- Project 3 URL (if applicable):

## Interviews taken

- Company | Date | Round | Outcome | Notes

---

# Red flags to watch for (self-check monthly)

- **Passive learning trap:** Watching videos and reading the book but not typing code. If your `rust-learning` repo has fewer than 40 commits by end of week 4, you're in this trap.
- **Tutorial hell:** Starting a new tutorial before finishing the current project. If Project 1 isn't shipped by end of week 6, do not start Project 2. Finish first.
- **Perfectionism:** Not deploying because "it's not ready." Deploy at 70% quality. You can improve after.
- **Isolation:** Not engaging with any Rust community for 2+ weeks. Fix by posting one question or answer somewhere.
- **Burnout:** Sleeping < 6.5 hrs or skipping meals to code. Drop hours before you drop the whole plan.

---

*Print this. Actually print it. Tick boxes with a pen. Digital checkboxes are too easy to lie to yourself with.*

*See you on the other side.*