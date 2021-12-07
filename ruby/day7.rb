def main
  positions = $stdin.read.strip.split(",").map(&:to_i)
  midpoint = positions.sort[positions.size / 2]

  costs = [midpoint - 1, midpoint, midpoint + 1].map { |mp| cost(positions, mp) }
  puts "part 1: #{costs.min}"

  costs = [midpoint - 1, midpoint].map { |mp| cost2(positions, mp) }
  if costs[0] > costs[1]
    last = midpoint
    last_cost = costs[1]
    loop do
      new_point = last + 1
      new_cost = cost2(positions, new_point)
      if new_cost > last_cost
        puts "part 2: #{last_cost}"
        break
      end
      last, last_cost = new_point, new_cost
    end
  else
    last = midpoint - 1
    last_cost = costs[0]
    loop do
      new_point = last - 1
      new_cost = cost2(positions, new_point)
      if new_cost > last_cost
        puts "part 2: #{last_cost}"
        break
      end
      last, last_cost = new_point, new_cost
    end
  end
end

def cost(positions, end_point)
  positions.inject(0) { |total, pos| total + (pos - end_point).abs }
end

def cost2(positions, end_point)
  #positions.each do |pos|
  #  p [pos, sumofn((pos - end_point).abs)]
  #end
  positions.inject(0) { |total, pos| total + sumofn((pos - end_point).abs) }
end

def sumofn(n)
  n * (n + 1) / 2
end

main
