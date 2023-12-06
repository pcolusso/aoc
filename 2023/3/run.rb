s1 = <<-INPUT
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
INPUT

def is_num?(c)
  48 <= c.ord && c.ord <= 57
end

def is_period?(c)
  c == '.'
end

def is_sym?(c)
  return false if c.nil?
  !is_num?(c) && !is_period?(c)
end

# offsets for each direction
DIRECTIONS =  [
  [-1, 0], # left
  [1, 0], # right
  [0, -1], #above
  [0, 1], # below
  [-1, -1], # diag left above
  [-1, 1], # diag left below
  [1, -1], # diag right above
  [1, 1], # diag right below
]

def part1(input)
  lines = input.split("\n")
  # get char in x,y
  index = lambda { |x, y| lines&.[](y)&.[](x) }
  # apply offset to x,y
  index_with_direction = lambda { |offset, x, y| index.call(x + offset[0], y + offset[1] ) }
  # function to check all directions are not symbols.
  check = lambda do |x, y|
    DIRECTIONS.each do |d|
      c = index_with_direction.call(d, x, y)
      # puts "Checking @ #{x + d[0]}, #{y + d[1]} => '#{c}' is a symbol... #{is_sym?(c) ? '✅' : '🅱️}'} " 
      return true if is_sym?(c)
    end
    false 
  end

  values = []
  lines.each_with_index do |line, y|
    # arr to collect number literals
    number_literals = []
    # flag to indicate we're in a number
    parsing = false
    good_number = false
    line.chars.each_with_index do |c, x|
      if is_num?(c)
        # Start parsing, if not already.
        if !parsing
          parsing = true
        end

        # If we find a symbol, mark it as such
        if !good_number && check.call(x, y)
          good_number = true
        end

        number_literals << c
      else # We're at a period or symbol
        if parsing && good_number 
          values << number_literals.join.to_i
        end
        parsing = false
        good_number = false
        number_literals = []
      end
    end
    if parsing && good_number
      values << number_literals.join.to_i
    end
  end
  values.sum
end

p part1(File.read("input.txt"))


# For part two, find all * with two adjacent numbers, and return it's index.
# Find all numbers, pair with an index itself.
# For each gear in the first set, find the two numbers matching that index, and add to range.

def extract_with_index(pattern, str)
  results = []
  str.enum_for(:scan, pattern).each do |match|
    results << [match, Regexp.last_match.begin(0)]
  end
  results
end

# From a start index, explode into all possible indices that would match.
# Need to know width of grid.
def explode(pair, width)
  num, index = pair
  y = index / width
  x = index % width
  indices = (0..num.length-1).map do |i|
    [x + i, y]
  end
  [num, indices]
end

def part2(input)
  lines = input.split("\n")
  width = lines.first.length
  nums_and_indices = extract_with_index(/\d+/, input)
  exploded = nums_and_indices.map { |pair| explode(pair, width) }
  stars = extract_with_index(/\*/, input)
  stars_exploded = stars.map do |_, index|
    y = index / width
    x = index % width
    
    DIRECTIONS.map { |d| [x + d[0], y + d[1]] }
  end.flatten(1)


  exploded.filter do |num, indices|
    matches = stars_exploded & indices
    p matches
    matches.count == 2 
  end
end

p part2(s1) 

