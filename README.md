# Overview

There is no time limit to completing this exercise, however, we recommend you aim to spend in the region of 2 hours on it. Once you have finished - and are happy with your solution - give us a shout.

## Description

Given we want to store historic data series for:

- an unlimited timeframe
- a potential growth in size of 100x
- a frequency of one hour
- a metric defined by a unique identifier (blockchain address), a key (label) and a value (numeric)

And given, we want to read those metrics for charts with the following requirements:

- a chart for a single metric
- a certain timeframe (e.g. last 90 days, last year, etc.)
- a maximum of 1000 points per chart (aggregate)
- response time of max 200ms

For the above specs, draw an architecture diagram with your technology and design choices.

Write a service/pipeline to output a data series for an aggregated data set from the database and a CSV file in this format (one value per day, random price change increasing to 1.000):

    date, price
    2019-01-01, 0.001
    2019-01-02, 0.002
    2019-01-03, 0.003
    ...
    2022-01-01, 1.000

The aggregate should be the value (each hour) for the address `0x123` and the key `balance` multiplied by the price from the CSV.

## Task

- Fork the repository in your private Github account
- Draw the diagram and write teh service
- Push and send us the link (optional raise a PR)

## Bonus

If the task is too simple or your're bored, you can do the following bonuses:

- Test your code or design a testing strategy
- Benchmark different storage engines
- Set up infrastructure as code
