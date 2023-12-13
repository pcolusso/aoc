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

Card = Struct.new(:id, :drawn, :winners, :score)

def parse_card(line)
  id =  line.match(/Card([0-9 ]*)\:/).captures.first.to_i
  drawn_numbers = line.match(/\:([0-9 ]*)\|/).captures.first.split(" ").map(&:to_i)
  winning_numbers = line.match(/\|([0-9 ]*)$/).captures.first.split(" ").map(&:to_i)
  matches = drawn_numbers & winning_numbers
  score = (2 ** (matches.count - 1)).floor
  Card.new(id, drawn_numbers, winning_numbers, score)
end

def part2(input)
  initial_cards = input.split("\n").map { |l| parse_card(l) }
  count = 0
  queue = []

  initial_cards.each { |c| queue << c  }

  while !queue.empty? do
    card = queue.shift
    index = card.id - 1
    count += 1

    if card.score > 0
      (1..card.score).each do |x|
        new_index = x + index
        if (new_index < initial_cards.count)
          queue.unshift(initial_cards[new_index])
        end
      end
    end
  end

  count
end

p part2 File.read("input.txt") 
