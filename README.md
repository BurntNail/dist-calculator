# dist-calculator
Basic Calculator for distribution from a given input.

Place your input into `input.txt`, or pass in the file as a CLI argument, with each input on a newline.

This then calculates the distribution of the given items. 


## Example

Using the given `input.txt`, we get the following output:
```
╭─────────┬──────────────┬───────────╮
│ Val / 4 ┆ # / 17 / 4.2 ┆ % / 25.0% │
╞═════════╪══════════════╪═══════════╡
│ B       ┆ 6            ┆ 35.3%     │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ D       ┆ 5            ┆ 29.4%     │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ A       ┆ 5            ┆ 29.4%     │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ C       ┆ 1            ┆ 5.9%      │
╰─────────┴──────────────┴───────────╯
```

As we can see from the column headers, we have 4 different keys, 17 elements (average of 4.2 elements per keys), and an average of 25% per key.
