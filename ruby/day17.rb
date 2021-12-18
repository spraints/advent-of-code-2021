require "set"

def main
  $stdin.readline =~ /target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)/ or
    raise "bad input"
  xr = ($1.to_i..$2.to_i)
  yr = ($3.to_i..$4.to_i)
  dys_memo = {}
  vs = Set.new
  #s = sample_ok.to_set
  (1..xr.max).each do |dx|
    steps = steps_to_hit_x(x: 0, dx: dx, xr: xr)
    has_inf = steps.delete(:inf)
    steps.each do |n|
      #p dx: dx, n: n
      dys = (dys_memo[n] ||= dys_for(steps: n, y: 0, yr: yr))
      dys.each do |dy|
        #if !s.include?([dx, dy])
        #  p err: true, dx: dx, dy: dy, n: n
        #end
        vs << [dx, dy]
      end
    end
    if has_inf
      (steps.max..steps.max+1000).each do |n|
        dys = (dys_memo[n] ||= dys_for(steps: n, y: 0, yr: yr))
        dys.each do |dy|
          #if !s.include?([dx, dy])
          #  p err: true, dx: dx, dy: dy, n: n
          #end
          vs << [dx, dy]
        end
      end
    end
  end

  max_dy = vs.map { |v| v[1] }.max
  puts "part 1: #{max_y(y: 0, dy: max_dy)}" # 3916
  puts "part 2: #{vs.size}" # 3096 is too high.
end

def sample_ok
  s = <<S
23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
8,-2    27,-8   30,-5   24,-7
S
  s.scan(/(-?\d+),(-?\d+)/).map { _1.map(&:to_i) }
end

def max_y(y:, dy:)
  y1 = y + dy
  return y if y > y1
  max_y(y: y1, dy: dy - 1)
end

# y0 = Y
# dy0 = DY
# y[n+1] = y[n] + dy[n]
# dy[n+1] = dy[n] - 1
#
# y1 = Y + DY
# dy1 = DY - 1
# y2 = (Y + DY) + (DY - 1) = Y + 2*DY - 1
# dy2 = DY - 2
# y3 = Y + 2*DY - 1 + (DY - 2) = Y + 3*DY - 3
# y[n] = Y + n*DY - sum(1..(n-1))

def dys_for(steps:, y:, yr:)
  dys = []

  final_y0 = y_after_steps(steps, y: y, dy: 0)
  dys << 0 if yr.include?(final_y0)
  #p dy: 0, fin: final_y0, yr: yr

  final_y = final_y0
  dy = 0
  until final_y > yr.max
    dy += 1
    final_y = y_after_steps(steps, y: y, dy: dy)
    #p dy: dy, fin: final_y, yr: yr, yr_max: yr.max
    dys << dy if yr.include?(final_y)
    #raise "boom" if dy > 10
  end

  final_y = final_y0
  dy = 0
  while final_y > yr.min
    dy -= 1
    final_y = y_after_steps(steps, y: y, dy: dy)
    #p dy: dy, fin: final_y, yr: yr, yr_min: yr.min
    dys << dy if yr.include?(final_y)
    #raise "boom" if dy < -10
  end

  dys
end

def y_after_steps(n, y:, dy:)
  y + dy*n - (1..(n-1)).sum
end

def steps_to_hit_x(x:, dx:, xr:)
  steps = []
  i = 0
  while x <= xr.max && dx > 0
    steps << i if xr.include?(x)
    i += 1
    x += dx
    dx -= 1
  end
  steps << :inf if xr.include?(x)
  steps
end

# start
# 0,0
# target area is given

def step(location:, velocity:)
  x, y = location
  dx, dy = velocity
  ddx = (dx == 0 ? 0 : (dx < 0 ? 1 : -1))
  {location: [x+dx, y+dy], velocity: [dx+ddx, dy-1]}
end

main
