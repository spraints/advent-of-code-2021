require "set"

def main
  insts = $stdin.readlines.map { parse_line _1 }
  w = Part1World.new
  insts.each do |cmd, xr, yr, zr|
    w.send(cmd, xr, yr, zr)
  end
  puts "part 1: #{w.count_on}"
end

def parse_line(line)
  cmd, pos = line.strip.split(" ", 2)
  xr, yr, zr = pos.split(",", 3).map { |s| s.split("=").last.split("..").map(&:to_i) }
  [cmd, xr, yr, zr]
end

class Part1World
  def initialize
    @grid = 101.times.map { 101.times.map { 101.times.map { false } } }
    @off = -50
  end

  def on(xr, yr, zr)
    set(true, xr, yr, zr)
  end

  def off(xr, yr, zr)
    set(false, xr, yr, zr)
  end

  def count_on
    count = 0
    @grid.each do |x|
      x.each do |y|
        y.each do |z|
          count += 1 if z
        end
      end
    end
    count
  end

  private

  def set(on, xr, yr, zr)
    return if xr[0] > 50
    return if xr[1] < -50
    return if yr[0] > 50
    return if yr[1] < -50
    return if zr[0] > 50
    return if zr[1] < -50

    xmin = [xr[0], -50].max
    xmax = [xr[1], 50].min
    ymin = [yr[0], -50].max
    ymax = [yr[1], 50].min
    zmin = [zr[0], -50].max
    zmax = [zr[1], 50].min

    xmin.upto(xmax) do |x|
      ymin.upto(ymax) do |y|
        zmin.upto(zmax) do |z|
          @grid[x + @off][y + @off][z + @off] = on
        end
      end
    end
  end
end

main
