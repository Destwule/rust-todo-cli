# CLI TODO APP

A simple command-line todo list app written in Rust, with JSON file persistence.

## FEATURES

- Add, Remove, and list tasks
- Mark tasks as completed or pending
- Tasks are automatically saved to a JSON file ('tasks.json') and reloaded on startup
- Prompts to create the storage file if it does not exist yet

## RUNNING IT
```bash
cargo run
```

You'll be dropped into a simple menu:

```
1. Add item
2. Remove item
3. List items
4. Mark as Completed
5. Mark as Pending
```

Type 'q' at any time to quit.

## NOTES

This was built as a practice project while learning Rust
