def main
  positions = $stdin.read.strip.split(",").map(&:to_i)
  puts "part 1: #{mincost(positions, median(positions), Part1.new)}"
  puts "part 2: #{mincost(positions, mean(positions), Part2.new)}"
end

def median(ns)
  ns.sort[ns.size/2]
end

def mean(ns)
  ns.sum / ns.size
end

def mincost(positions, start, calc)
  cost = cost(positions, start, calc)
  cost1 = cost(positions, start+1, calc)
  n, cost, step = cost1 < cost ? [start+1, cost1, 1] : [start, cost, -1]
  loop do
    nn = n + step
    ncost = cost(positions, nn, calc)
    if ncost > cost
      return cost
    end
    cost = ncost
    n = nn
  end
end

def cost(positions, pos, calc)
  positions.map { |x| calc.cost((x-pos).abs) }.sum
end

class Part1
  def cost(dist)
    dist
  end
end

class Part2
  def cost(dist)
    dist * (dist + 1) / 2
  end
end

main
