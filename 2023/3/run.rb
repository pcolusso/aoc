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
