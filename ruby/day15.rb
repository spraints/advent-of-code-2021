require "set"

def main
  grid = $stdin.readlines.map { _1.strip.chars.map(&:to_i) }
  puts "part 1: #{least_cost(grid, [0,0], bottom_right_node(grid))}"
end

MAX = 999_999_999

def least_cost(grid, from, to)
  costs = {from => 0}
  visited = Set.new
  cost = 0
  until from == to
    puts "from #{from}, cost = #{cost}..."
    visited << from
    min_cost = MAX
    edges(grid, from).each do |node, edge_cost|
      puts "  - consider #{node} (+#{edge_cost}, previously #{costs[node].inspect})"
      next if visited.include?(node)
      puts "    + not visited yet"
      new_cost = cost + edge_cost
      node_cost = costs[node]
      if node_cost.nil? || node_cost > new_cost
        puts "    + update cost to #{new_cost}"
        node_cost = costs[node] = new_cost
      end
      if node_cost < min_cost
        puts "    ++ use it!"
        min_cost = node_cost
        from = node
      end
    end
    raise :boom if min_cost == MAX
    cost = min_cost
  end
  cost
end

def bottom_right_node(grid)
  r = grid.size - 1
  c = grid[r].size - 1
  [r,c]
end

def edges(grid, node)
  [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
  ].map { |dr, dc|
    [dr + node[0], dc + node[1]]
  }.select { |r, c| r >= 0 && c >= 0 && r < grid.size && c < grid[0].size }.map { |r, c|
    [ [r, c], grid[r][c] ]
  }
end

main
