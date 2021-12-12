require "set"

def main
  octopi = $stdin.readlines.map { |line| line.strip.chars.map(&:to_i) }
  flashes = 0
      #puts "Before any steps:"
      #puts octopi.map(&:join)
  100.times do |i|
    octopi, inc = step(octopi)
    flashes += inc
    if i < 10
      #puts "After step #{i+1}:"
      #puts octopi.map(&:join)
    end
  end
  puts "part 1: #{flashes}"

  i = 100
  loop do
    i += 1
    octopi, inc = step(octopi)
    if i > 190 && i < 200
      puts "After step #{i}:"
      puts octopi.map(&:join)
    end
    if inc == 100
      puts "part 2: #{i}"
      break
    end
  end
end

def step(octopi)
  flashes = []
  (0..9).each do |i|
    (0..9).each do |j|
      if (octopi[i][j] += 1) == 10
        flashes << [i,j]
      end
    end
  end
  flashed = []
  while c = flashes.shift
    #p flash: c
    neighbors(*c).each do |i, j|
      if (octopi[i][j] += 1) == 10
        flashes << [i, j]
      end
    end
    flashed.push c
  end
  #puts "flashed:"
  #puts flashed.sort.map { |c| " - #{c.inspect}" }
  flashed.each do |i,j|
    octopi[i][j] = 0
  end
  [octopi, flashed.size]
end

def neighbors(x,y)
  [
    [x+1, y],
    [x-1, y],
    [x, y+1],
    [x, y-1],
    [x+1, y+1],
    [x-1, y+1],
    [x+1, y-1],
    [x-1, y-1],
  ].select { |i,j| j < 10 && i < 10 && i >= 0 && j >= 0 }
end

main
