#!/usr/bin/env ruby

example = File.read("example.txt")
  .split("\n")
  .map { |line| line.split(" ") }

input = File.read("input.txt")
  .split("\n")
  .map { |line| line.split(" ") }

def solve_1(input)
  x, y = 0, 0
  input.each do |command, magnitude|
    magnitude = magnitude.to_i
    case command
    when 'forward'
      x += magnitude
    when 'down'
      y += magnitude
    when 'up'
      y -= magnitude
    end
  end
  [x, y]
end

def solve_2(input)
  x, y, a = 0, 0, 0
  input.each do |command, magnitude|
    puts("x: #{x}, y: #{y}, a: #{a}")
    puts("#{command} #{magnitude}")
    magnitude = magnitude.to_i
    case command
    when 'forward'
      x += magnitude
      y += magnitude * a
    when 'down'
      # y += magnitude  # TODO: Learn to read the question!
      a += magnitude
    when 'up'
      # y -= magnitude
      a -= magnitude
    end
  end
  puts("x: #{x}, y: #{y}, a: #{a}")
  [x, y]
end

example_1_ans = solve_1(example)
problem_1_ans = solve_1(input)
example_2_ans = solve_2(example)
problem_2_ans = solve_2(input)

puts "Example 1: #{example_1_ans[0] * example_1_ans[1]}"

puts "Answer 1: #{problem_1_ans[0] * problem_1_ans[1]}"

puts "Example 2: #{example_2_ans[0] * example_2_ans[1]}"

puts "Answer 1: #{problem_2_ans[0] * problem_2_ans[1]}"