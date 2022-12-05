# Task

Build a simple spreadsheet application. You wont be recreating excel don't worry. We expect
a reactive grid of inputs that auto-saves user inputs over time.

You're provided with

- A design in Figma, [located here](https://www.figma.com/file/ID54ipGCsghCKyAHBKgshE/Challenges?node-id=0%3A1&t=tLMYNico0EBK9bHq-1) You will need a figma account to view layers
- A server as a docker image, [located here](https://hub.docker.com/r/stakingrewards/engineering-frontend-challenge)

## What we're looking for

- UI implementation as close to the design as possible

Feel free to improve things _you think_ should be better

- Auto-saving user input

- Reactive UI

- Performant UI with emphasis on a friendly UX

Don't confuse the user, if you feel there's a missing design element that aids in this, add it.

We also don't expect you to stress test the UI, but your solutions
will give hints to potential performance issues. So be mindful of that.

- Build around limitations

The server is buggy and slow, you still need to aim for an exceptional experience

- Make good decisions

You'll be evaluated based on your design implementation choices, code implementation choices and
the solutions you bring to the table

> We can only evaluate what you put forth, our evaluation will be incomplete if you provide an incomplete solution. If time is a bottleneck, feel free to ask for more time and we're happy to negotiate... Just like in a real, working environment ;)

## Time

Speaking of time, we feel the task should not take you more than 3 working days.
We could be wrong.

### The Server

It's slow, buggy and it's all by design. Your UI implementation has to account for these
facts, the server will randomly throw errors and will randomly introduce latencies.

The server only allows you to "save" a spreadsheet by sending the values as a CSV blob. This
will help you implement auto-saving.

The API is [documented here](./api.md)

### The UI

We expect you to compute the spreadsheet formulas on the client side. The simple expression
language is defined below.

The UI needs to be reactive, as you would expect in any spreadsheet software. Expressions in
a cell are computed in `onblur` events. Expressions that reference cells for values whose values
change must result in a recomputation of the expression in the relevant places.

**Expressions**

Any computable expression in the spreadsheet must be prefixed with `=`. The expression
language supports basic arithmetic expressions as well as value references.

Arithmetics look like this (If you want to go the extra mile, add support for floats)

```
=1 + 2 * (42.42 / 1)
```

Value references look like this

```
=(A..Z)n+
```

where A to Z are column labels and `n+` is row number. The notation of `n+` simply
implies that row numbers in value references can have multiple digits. Both `A0`
and `A10` are valid value references. However, `A00` and `A010` means the same as `A0` and `A10` respectively.

Value references can be used in place of numbers, the letter (column-label) should have no
space between it and the row-number.

## Technology

We work in React and Nextjs using both Javascript and Typescript. We expect the solution to be React based even if you're
not exceptionally familiar with React. React, in the end of the day, is a Javascript library. If you know Javascript, then
we trust you'll find React's documentation easy to follow.

The choice between Javasript or Typescript is up to you.

## Submission

Please submit your working code as a Github repo link with instructions on how to run the project.
