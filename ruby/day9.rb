require "set"

def main
  input = $stdin.readlines.map { |line| line.strip.chars.map(&:to_i) }
  input.each do |l|
    #p l
  end
  lows = []
  0.upto(input.size - 1) do |x|
    0.upto(input[x].size - 1) do |y|
      if low?(input, x, y)
        #p [[x,y],input[x][y]]
        lows << [x, y]
      end
    end
  end
  puts "part 1: #{lows.map { |x, y| input[x][y] + 1 }.sum}"
  # 436 ^^
  basins = lows.map { |x,y| basin_size(input, x, y) }
  p basins: basins
  puts "part 2: #{basins.sort.reverse.take(3).inject { |a,b| a * b }}"
end

DIRECTIONS = [
  [-1,0],
  [1,0],
  [0,-1],
  [0,1],
]

def low?(input, x, y)
  val = input[x][y]
  DIRECTIONS.each do |(dx, dy)|
    cx = x + dx
    cy = y + dy
    #p [cx, cy]
    if cx >= 0 && cy >= 0
      xx = input[cx]
      next if xx.nil?
      yy = xx[cy]
      next if yy.nil?
      if yy <= val
        #p [:no, [x, y], [cx, cy], val, yy]
        return false
      end
      #p [:yes, [x, y], [cx, cy], val, yy]
    end
  end
  #p :yes!
  true
end

def basin_size(input, x, y)
  visited = Set.new
  to_visit = [ [input[x][y], [x,y]] ]
  #p [x,y]
  while c = to_visit.shift
    depth, c = *c
    next if visited.include?(c)
    #p visit: c
    visited << c
    x, y = *c
    DIRECTIONS.each do |dx, dy|
      vx = x + dx
      vy = y + dy
      next if vx < 0 || vy < 0
      row = input[vx]
      next if row.nil?
      vdepth = row[vy]
      next if vdepth.nil? || vdepth == 9
      if vdepth > depth
        #p up: [vx, vy]
        to_visit << [vdepth, [vx, vy]]
      end
    end
  end
  visited.size
end

main
