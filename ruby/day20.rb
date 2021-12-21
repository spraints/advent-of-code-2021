require "set"

def main
  key, input = $stdin.read.split("\n\n")
  key = key.gsub(/\s+/, "").chars.map { |c| parse_c(c) }
  input = input.lines.map(&:strip).map { |line| line.chars.map { |c| parse_c(c) } }
  u = Universe.new(input, key: key)
  #u.draw
  u.step!
  #u.draw
  u.step!
  #u.draw
  puts "part 1: #{u.lit}"
  48.times { u.step! }
  puts "part 2: #{u.lit}"
end

def parse_c(c)
  c == "#" ? 1 : 0
end

class Universe
  def initialize(pixels, key:)
    @pixels = pixels
    @key = key
    @everything_else = 0
    @rows = @pixels.size
    @cols = @pixels[0].size
  end

  def draw
    @pixels.each do |row|
      puts row.map { _1 == 1 ? "#" : "." }.join
    end
      puts "-----"
  end

  def lit
    @pixels.inject(0) { |c, row| c + row.count { _1 == 1 } }
  end

  def step!
    res = (-1..@rows).map { |r| (-1..@cols).map { |c| @key[val(r, c)] } }
    @pixels = res
    @rows += 2
    @cols += 2
    @everything_else = @everything_else == 0 ? @key[0] : @key[0b111_111_111]
  end

  def val(row, col)
    res = 0
    (row-1..row+1).each do |r|
      (col-1..col+1).each do |c|
        x = bit(r, c)
        #p [r,c] => x
        res = res * 2 + x
      end
    end
    #if row == 1
    #  p [row, col] => [res, res.to_s(2)]
    #end
    res
  end

  def bit(r, c)
    if r < 0 || r >= @rows || c < 0 || c >= @cols
      #p :oof => [r,c]
      return @everything_else
    end
    @pixels[r][c]
  end
end

main
