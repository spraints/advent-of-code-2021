require "set"

def main
  insts = $stdin.readlines.map { parse_line _1 }

  w = Part1World.new
  insts.each do |cmd, xr, yr, zr|
    w.send(cmd, xr, yr, zr)
  end
  puts "part 1: #{w.count_on}"

  w = Part2World.new
  insts.each do |cmd, xr, yr, zr|
    w.send(cmd, xr, yr, zr)
  end
  puts "part 2: #{w.count_on}"
end

def parse_line(line)
  cmd, pos = line.strip.split(" ", 2)
  xr, yr, zr = pos.split(",", 3).map { |s| s.split("=").last.split("..").map(&:to_i) }
  [cmd, xr, yr, zr]
end

class Part2World
  def initialize
    @on_cubes = []
  end

  def on(xr, yr, zr)
    off(xr, yr, zr)
    @on_cubes = @on_cubes + [Cube.new(xr, yr, zr)]
  end

  def off(xr, yr, zr)
    @on_cubes = @on_cubes.flat_map { |cube| cube.delete(xr, yr, zr) }
  end

  def count_on
    @on_cubes.inject(0) { |count, cube| count + cube.volume }
  end
end

class Cube
  def initialize(xr, yr, zr)
    @xmin, @xmax = xr
    @ymin, @ymax = yr
    @zmin, @zmax = zr
  end

  def volume
    (@xmax - @xmin + 1) * (@ymax - @ymin + 1) * (@zmax - @zmin + 1)
  end

  def delete(xr, yr, zr)
    xmin, xmax = xr
    ymin, ymax = yr
    zmin, zmax = zr

    return [self] if xmin > @xmax || xmax < @xmin || ymin > @ymax || ymax < @ymin || zmin > @zmax || zmax < @zmin

    before = self.inspect
    before_volume = volume

    xcmin = [xmin, @xmin].max
    xcmax = [xmax, @xmax].min
    ycmin = [ymin, @ymin].max
    ycmax = [ymax, @ymax].min
    zcmin = [zmin, @zmin].max
    zcmax = [zmax, @zmax].min
    clamped = Cube.new([xcmin, xcmax], [ycmin, ycmax], [zcmin, zcmax])

    remaining = []
    if xmin > @xmin
      remaining << Cube.new([@xmin, xmin-1], [@ymin, @ymax], [@zmin, @zmax])
      @xmin = xmin
    end

    if ymin > @ymin
      remaining << Cube.new([@xmin, @xmax], [@ymin, ymin-1], [@zmin, @zmax])
      @ymin = ymin
    end

    if zmin > @zmin
      remaining << Cube.new([@xmin, @xmax], [@ymin, @ymax], [@zmin, zmin-1])
      @zmin = zmin
    end

    if zmax < @zmax
      remaining << Cube.new([@xmin, @xmax], [@ymin, @ymax], [zmax+1, @zmax])
      @zmax = zmax
    end

    if ymax < @ymax
      remaining << Cube.new([@xmin, @xmax], [ymax+1, @ymax], [@zmin, @zmax])
      @ymax = ymax
    end

    if xmax < @xmax
      remaining << Cube.new([xmax+1, @xmax], [@ymin, @ymax], [@zmin, @zmax])
    end


    #if xmax < @xmax
    #  remaining

    remaining_volume = remaining.map(&:volume).sum
    puts "FROM #{before}",
      "DELETE #{[xr, yr, zr].inspect} (#{clamped.inspect})",
      remaining.map { |c| " >> #{c.inspect}" },
      "  EXPECTED VOLUME #{before_volume - clamped.volume}",
      "  GOT VOLUME      #{remaining_volume}"
    exit 1 if before_volume - clamped.volume != remaining_volume

    remaining
  end

  def inspect
    "#<Cube #{[[@xmin,@xmax], [@ymin, @ymax], [@zmin, @zmax]].inspect} vol=#{volume}>"
  end
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
