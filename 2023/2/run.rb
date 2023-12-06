s1 = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""

def parse(input)
  colours = ["red", "green", "blue"]
  
  input.split(",").map do |x|
    count = x.scan(/\d+/).first.to_i
    colour = colours.find { |c| x.include?(c) }
    [count, colour] 
  end
end

def valid_game?(games)
  games.each do |pair|
    count, colour = pair
    valid_nums = {
      "red" =>  12,
      "green" => 13,
      "blue" => 14
    }
    if count > valid_nums[colour]
      p "Found #{colour} => #{count} > #{valid_nums[colour]}"
      return false
    end
  end
  true
end

def part1(input)

  input.split("\n").map do |line|
    ident = line.split(":")
    id = ident.first.gsub(/Game /, '').to_i
    games = ident.last.split(";").map { |x| parse(x) }
    checks = games.map { |g| valid_game?(g) } 
    if checks.all? { |x| x == true }
      id.to_i
    else
      0
    end
  end.sum
end

p part1 File.read("input.txt")
