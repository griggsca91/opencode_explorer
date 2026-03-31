# opencode_explorer

A Rust TUI application for viewing opencode session request counts directly from the SQLite database.

## Why creep on DB

Because I don't want to do a proxy due to the annoying way to get that work, especially in an environment where you may not have easy access to certs or managing the proxy value. I'm not looking for pure accuracy, just a general idea.

Yes I know this will be fragile, oh well.

## What it does

This application connects to opencode's SQLite database and displays the top 10 most recent sessions with their user request counts in an interactive terminal table.

## Number of requests sent (excluding the initial title request that opencode sends)

The role `assistant` is the response, the role `user` is the person.

```sql
select s.title, m.session_id, count(1)
from message m
join session s on m.session_id = s.id
where m.data ->> 'role' == 'user'
group by m.session_id
order by max(m.time_updated) desc limit 10;
```

## Model Price

<https://docs.github.com/en/copilot/reference/ai-models/supported-models#model-multipliers>

## Usage

Build and run with:

```bash
cargo run --release
```

## Controls

- `j`/`↓` - Move down
- `k`/`↑` - Move up
- `h`/`←` - Move left (column)
- `l`/`→` - Move right (column)
- `g` - Go to first row
- `G` - Go to last row
- `q`/`Esc` - Quit
