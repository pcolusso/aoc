s1 = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

def is_num?(c)
  48 <= c.ord && c.ord <= 57
end

def is_period?(c)
  c == '.'
end

def accept?(c)
  c.nil? || is_num?(c) || is_period?(c)
end

def reject?(c)
  !accept?(c)
end


def part1(input)
  # offsets for each direction
  directions =  [
    [-1, 0], # left
    [1, 0], # right
    [0, -1], #above
    [0, 1], # below
    [-1, -1], # diag left above
    [-1, 1], # diag left below
    [1, -1], # diag right above
    [1, 1], # diag right below
  ]
  lines = input.split("\n")
  # get char in x,y
  index = lambda { |x, y| lines&.[](y)&.[](x) }
  # apply offset to x,y
  index_with_direction = lambda { |offset, x, y| index.call(x + offset[0], y + offset[1] ) }
  # function to check all directions are not symbols.
  check = lambda do |x, y|
    directions.each do |d|
      c = index_with_direction.call(d, x, y)
      return false if reject?(c)
    end
    true
  end

  values = []
  lines.each_with_index do |line, y|
    # arr to collect number literals
    number_literals = []
    # flag to indicate we're in a number
    parsing = false
    bad_number = false
    line.chars.each_with_index do |c, x|
      if is_num?(c)
        if check.call(x, y) # If we're still good, or starting a clean number, append the literal.
          parsing = true
          number_literals << c
        else
          parsing = false
          bad_number = true
          number_literals = []
        end
      else # We're at a period or symbol
        if parsing && !bad_number
          values << number_literals.join.to_i
          number_literals = []
        end
        parsing = false
        bad_number = false
      end
    end
  end
  values
end

# Y, then X, origin top right.
p part1(s1)
