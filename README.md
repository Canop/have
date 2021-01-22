

Allow inserting a function on any type in a call chain:

```
any_type
	.fun(|o| do_something_with(o))
	.other_things();
```

More detailled example:

```
use {
    have::Fun,
    itertools::*,
};

fn analyze(log_lines: Vec<LogLine>) {
    log_lines
    	.iter()
        .into_group_map_by(|ll| ll.remote_addr)
        .fun(|g| println!("{} distinct remote_addresses", g.len()))
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(3)
        .for_each(|e| println!("{} hits by remote addresse {}", e.1.len(), e.0));
}
```

## Usage:

```TOML
[dependencies]
have = "0.1"
```
