require "set"

def main
  dots, instructions = $stdin.read.split("\n\n")
  grid = []
  dots.lines.each do |line|
    x, y = line.strip.split(",").map(&:to_i)
    grid[y] ||= []
    grid[y][x] = true
  end

  instructions = instructions.lines.map { |line| line =~ /fold along (.)=(\d+)/; [$1, $2.to_i] }
  grid = fold(grid, instructions.shift)
  puts "part 1: #{grid.compact.map { |r| r.select { _1 }.size }.sum}"
  instructions.each do |i|
    grid = fold(grid, i)
  end
  puts "part 2:"
  draw(grid)
end

def draw(grid)
  puts grid.map { |row| (row||[]).map { |x| x ? "#" : " " }.join.rstrip }
end

def fold(grid, inst)
  axis, pos = inst
  case axis
  when "y"
    fold_vert(grid, pos)
  when "x"
    fold_hor(grid, pos)
  else
    raise "no #{inst.inspect}"
  end
end

def fold_vert(grid, y_split)
  newgrid = []
  grid.take(y_split).each_with_index do |row, y|
    a = row || []
    b = grid[2*y_split - y] || []
    l = [a.size, b.size].max
    newgrid << (0..l).map { |x| a[x] || b[x] }
  end
  newgrid
end

def fold_hor(grid, x_split)
  grid.map { |row| row && (0..x_split).map { |x| x2 = 2*x_split - x; row[x] || row[x2] } }
end

main
