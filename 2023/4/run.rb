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
  
  puts "Card #{id} scored #{score}"
  score
end

def value_card(index, score, count, lines)
  (index..score).map do |i|
    new_count = count + 1
    new_index = i + 1
    line = lines[new_index]
    score = score_card(line)
    
    if score.zero?
      return count
    else
      return value_card(new_index, score, new_count, lines)
    end
  end.sum
end

def part2(input)
  lines = input.split("\n")

  lines.map.with_index do |line, index|
    score = score_card(line)
    value_card(index, score, 0, lines)
  end
end

p part2 s1#File.read("input.txt") 
