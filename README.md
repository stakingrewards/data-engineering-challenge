# The Journey

The environment we have at Staking Rewards and the engineers you'll work with dictate these three expectations:

1. You're a well-rounded engineer
2. Comfortable in unfamiliar situations
3. And have an unquenchable thirst for knowledge

This is why we're calling this a journey not a challenge. We'd like you to venture
into an uncharted territory, have fun while doing it and treat yourself to a difficult but
interesting engineering task.

This means we don't expect you to compete the task, unless you're absolutely intrigued by it, have or can
develop the skills in the alloted time and don't like to leave things unfinished which
gives you more plus points obviously.

However, the purpose is to see what you could accomplish on a tight time budget, working in a technology you're unfamiliar with, doing a task that on the surface, looks like a hellish uphill climb. It isn't. 

Did I lose you? No, wonderful! Grab yourself some coffee, or tea for that matter and dive right in.

## Spreadsheets

> Trivia: Did you know that the concept of electronic spreadsheets dates back to 1961
> when it was first outlined in a paper titled "Budgeting Models and System Simulation" by Richard Mattessich

Pretty neat huh? Yeah, that's straight from Wikipedia. It's a fun read if you're into that kind of thing but in any case, back to the task.

### Task

You're given a CSV file, titled `transactions.csv`. If you have a keen eye and inspect the contents of the file 
you'll realize that this isn't a traditional comma-separated values file. 

- The delimiter is the pipe operator `|`
- Named columns have an exclamation mark prefix `!`
- Named columns appear anywhere in the file as long as they maintain the same column count
- Cells can have equations prefixed with `=`

The goal is to take `transactions.csv` and compute what needs to be computed producing a file
that contains all the static values + all the equations resolved.

We ask you to do this in a language you are not familiar with.

### Operations & Equations

Any computable expression in the CSV must be prefixed with `=`. The expression
language is very similar to excel formulas, it supports basic arithmetic expressions 
as well as function calls that provide additional features like comparisons,
string concatenations and other useful utility functions.

#### Operations

- `^^` Copies the formula from the cell above in the same column, with some special evaluation rules
- `(A..Z)n` references a cell by a combination of a column-letter+row-number. Ex: A2 B3
- `A^` copies the evaluated result of the cell above in the same column
- `!label` Columns can have labels, which allows this ability to have different column groups in the same file as long as the number of columns stays consistent
- `A^v` copies the evaluated result of the last cell in the specified column from the most recently available column group that has data in that specified column
- `@label<n>` References a specific labeled column and a specific row `n` under that column relative to where the column was labeled. This is a reference operator with relative row traversal
