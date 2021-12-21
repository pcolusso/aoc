#!/usr/bin/env ruby

example = File.read("example.txt")
  .split("\n")

def rates(collection)
  length = collection.first.length
  counts = length.times.map { |_| Hash.new(0) }
  
  collection.each do |l|
    (0...length).each do |i|
      counts[i][l[i]] += 1
    end
  end
  gamma_rate   = counts.map { |h| h.max_by { |k, v| v}[0] }.join.to_i(2)
  epsilon_rate = counts.map { |h| h.min_by { |k, v| v}[0] }.join.to_i(2)

  [gamma_rate, epsilon_rate]
end

example_rates = rates(example)

p "Part 1 Example: Gamma #{example_rates[0]} Epsilon #{example_rates[1]} Power #{example_rates.reduce(:*)}"

input = File.read("input.txt")
  .split("\n")

part1_rates = rates(input)

p "Part 1: Gamma #{part1_rates[0]} Epsilon #{part1_rates[1]} Power #{part1_rates.reduce(:*)}"

# Part 2

example = File.read("example.txt")
  .split("\n")
  .map(&:chars)
  .transpose

def oxy_rating(collection)
  length = collection.first.length

  # We're transposed now, so we don't need to track the i
  collection.each do |r|
    most_common = r.max_by { |v| r.count(v) }
  end

  # Honestly, this got too wordy so I got bored.
end