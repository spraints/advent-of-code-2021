def main
  input = $stdin.readlines.map { |line| line.strip.chars.map(&:to_i) }
  input.each do |l|
    p l
  end
  lows = []
  0.upto(input.size - 1) do |x|
    0.upto(input[x].size - 1) do |y|
      if low?(input, x, y)
        #p [[x,y],input[x][y]]
        lows << input[x][y]
      end
    end
  end
  p lows
  puts "part 1: #{lows.map { |x| x + 1 }.sum}"
end

def low?(input, x, y)
  val = input[x][y]
  [
    [-1,0],
    [1,0],
    [0,-1],
    [0,1],
  ].each do |(dx, dy)|
    cx = x + dx
    cy = y + dy
    p [cx, cy]
    if cx >= 0 && cy >= 0
      xx = input[cx]
      next if xx.nil?
      yy = xx[cy]
      next if yy.nil?
      if yy <= val
        p [:no, [x, y], [cx, cy], val, yy]
        return false
      end
      p [:yes, [x, y], [cx, cy], val, yy]
    end
  end
  p :yes!
  true
end

main
