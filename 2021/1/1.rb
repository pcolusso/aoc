#!/usr/bin/env ruby

def solve_1(arr)
  states = [:no_change]
  prev = arr[0]

  arr[1..].each do |x|
    states.push( x > prev ? :increased : :decreased)
    prev = x
  end
  raise unless states.count == arr.count
  states
end

# Discovered each_cons, which will give us that moving window in a functional manner
def solve_1_alt(arr)
  arr.each_cons(2).map { |a, b| b > a ? :increased : :decreased }
end

puts ("Example: ")

example_answer = solve_1(File.read("example.txt").split("\n").map(&:to_i))

puts (example_answer.filter { |x| x == :increased }.count)

puts ("Problem 1: ")

problem_1 = solve_1(File.read("input.txt").split("\n").map(&:to_i))
problem_1_alt = solve_1_alt(File.read("input.txt").split("\n").map(&:to_i))

puts(problem_1.filter { |x| x == :increased }.count)
puts(problem_1_alt.filter { |x| x == :increased }.count)

# So, for a one-liner (ish)

p File.read("input.txt")
    .split("\n")
    .map(&:to_i)
    .each_cons(2)
    .map { |a, b| b > a ? :increased : :decreased }
    .filter { |x| x == :increased }
    .count

# Gets muuuch easier solving functionally...
puts("Problem 2")
p File.read("input.txt")
      .split("\n")
      .map(&:to_i)
      .each_cons(3)
      .map { |a, b, c| a + b + c }
      .each_cons(2)
      .map { |a, b| a == b ? :no_change : b > a ? :increased : :decreased }
      .filter { |x| x == :increased }
      .count