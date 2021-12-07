def main
  positions = $stdin.read.strip.split(",").map(&:to_i)
  positions.sort!
  midpoint = positions[positions.size / 2]
  costs = [midpoint - 1, midpoint, midpoint + 1].map { |mp| cost(positions, mp) }
  puts "part 1: #{costs.min}"
end

def cost(positions, end_point)
  positions.inject(0) { |total, pos| total + (pos - end_point).abs }
end

main
