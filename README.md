### Task

You're given a simple design for a spreadsheet application, located [here](): link. We ask you to build a browser based spreadsheet application according to it.
The cells should allow you to insert constants or operators, which will be avaluated on the fly.
A list of supported operations and equations can be found below.

To spice up your journey, there was a little server prepared for you to implement an autosave feature.
The docker image for the server lives [here](https://hub.docker.com/r/stakingrewards/engineering-frontend-challenge).
It exposes two endpoints, to save the spreadsheet data and get the status of the save request. Unfortunately, the server is very slow and kinda buggy.
We need you to take into account that the saving happens asynchronously and might even result in an error.

### Operations & Equations

Any computable expression in the spreadsheet must be prefixed with `=`. The expression
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
