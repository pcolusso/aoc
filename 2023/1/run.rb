def part1(input)
  input.split("\n").map do |line|
    digits = line.chars.filter { |c| 48 <= c.ord && c.ord <= 57 }
    fl = [digits.first, digits.last].join.to_i
    fl
  end.sum
end

s1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""

p part1 File.read("input.txt")

s2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

def parse_words_or_letters(input)
  words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
  words.find_index(input) || input.to_i 
end

def part2(input)
  input.split("\n").map do |line|
    digits = line.scan(/(?=(\d|one|two|three|four|five|six|seven|eight|nine))/).flatten.map { |w| parse_words_or_letters(w) }
    fl = [digits.first, digits.last].join.to_i
    fl
  end.sum
end

p part2 File.read("input.txt") 
