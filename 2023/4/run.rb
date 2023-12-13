s1 = <<-TEXT
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
TEXT


def part1(input)
  input.split("\n").map do |line|
    id =  line.match(/Card([0-9 ]*)\:/).captures.first
    drawn_numbers = line.match(/\:([0-9 ]*)\|/).captures.first.split(" ").map(&:to_i)
    winning_numbers = line.match(/\|([0-9 ]*)$/).captures.first.split(" ").map(&:to_i)
    matches = drawn_numbers & winning_numbers
    score = (2 ** (matches.count - 1)).floor
    score
  end.sum
end

def score_card(line)
  id =  line.match(/Card([0-9 ]*)\:/).captures.first
  drawn_numbers = line.match(/\:([0-9 ]*)\|/).captures.first.split(" ").map(&:to_i)
  winning_numbers = line.match(/\|([0-9 ]*)$/).captures.first.split(" ").map(&:to_i)
  matches = drawn_numbers & winning_numbers
  score = (2 ** (matches.count - 1)).floor
  
  score
end

def process_card(cursor, lines, counts)
  puts "Process card working on index #{cursor}"
  # Define the base case
  if cursor >= lines.count-1
    return 
  end
  
  line = lines[cursor]
  score = score_card(line)

  # If score is 0, the range will be empty, avoiding iteration.
  (1..score).each do |x|
    i = cursor + x
    next if i >= lines.count # Janky fix, this loop should be better...
    line = lines[i]
    id =  line.match(/Card([0-9 ]*)\:/).captures.first.to_i
    counts[id] += 1
    process_card(i, lines, counts)
  end
end

def part2(input)
  lines = input.split("\n")
  counts = (1..lines.count).map { |x| [x, 1] }.to_h

  (0..lines.count-1).each do |i|
    process_card(i, lines, counts)
  end

  counts.values.sum
end

p part2 File.read("input.txt") 
