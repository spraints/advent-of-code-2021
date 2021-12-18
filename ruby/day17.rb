require "set"

def main
  $stdin.readline =~ /target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)/ or
    raise "bad input"
  xr = ($1.to_i..$2.to_i)
  yr = ($3.to_i..$4.to_i)
  max_steps = 1 + -2*yr.min
  dys_memo = {}
  vs = Set.new
  (1..xr.max).each do |dx|
    steps = steps_to_hit_x(x: 0, dx: dx, xr: xr)
    has_inf = steps.delete(:inf)
    steps.each do |n|
      dys = (dys_memo[n] ||= dys_for(steps: n, y: 0, yr: yr))
      dys.each do |dy|
        vs << [dx, dy]
      end
    end
    if has_inf
      (steps.max..max_steps).each do |n|
        dys = (dys_memo[n] ||= dys_for(steps: n, y: 0, yr: yr))
        dys.each do |dy|
          vs << [dx, dy]
        end
      end
    end
  end

  max_dy = vs.map { |v| v[1] }.max
  puts "part 1: #{max_y(y: 0, dy: max_dy)}"
  puts "part 2: #{vs.size}"
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
