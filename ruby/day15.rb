require "set"

def main
  grid = $stdin.readlines.map { _1.strip.chars.map(&:to_i) }
  puts "part 1: #{least_cost(grid, [0,0], bottom_right_node(grid))}"
end

MAX = 999_999_999

def least_cost(grid, from, to)
  visited = Set.new
  cost = Hash.new(MAX)
  cost[from] = 0
  current = from
  visit_soon = Set.new
  until visited.include?(to)
    #puts "node: #{current.inspect} (cost = #{cost[current].inspect})"
    current_cost = cost[current]
    edges(grid, current).each do |node, edge_cost|
      next if visited.include?(node)
      cost[node] = [current_cost + edge_cost, cost[node]].min
      #puts "  - cost of #{node.inspect} is now #{cost[node]}"
      visit_soon << node
      p visit_soon: visit_soon
    end
    puts "after visiting #{current.inspect}"
    grid.each_with_index do |row, r|
      row.each_with_index do |_, c|
        if current == [r,c]
          printf "%3d* ", cost[[r,c]]
        elsif visited.include?([r,c])
          printf "%3dx ", cost[[r,c]]
        elsif cost.include?([r,c])
          printf "%3d  ", cost[[r,c]]
        elsif to == [r,c]
          printf " > < "
        else
          printf "     "
        end
      end
      printf "\n"
    end
    visited << current
    p visit_soon: visit_soon
    raise "boom" if visit_soon.empty?
    current = visit_soon.sort_by { |n| cost[n] }.first
    #p visit_soon: visit_soon
    visit_soon.delete(current)
  end
  cost[to]
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
