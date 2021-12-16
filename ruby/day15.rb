require "set"

def main
  grid = $stdin.readlines.map { _1.strip.chars.map(&:to_i) }
  puts "part 1: #{least_cost(grid, [0,0], bottom_right_node(grid))}"
end

MAX = 999_999_999

def least_cost(grid, start, goal)
  open_set = Set.new
  open_set << start

  came_from = Hash.new

  g_score = Hash.new(MAX)
  g_score[start] = 0

  f_score = Hash.new(MAX)
  f_score[start] = min_cost(start, goal)

  until open_set.empty?
    current = open_set.sort_by { |node| f_score[node] }.first
    if current == goal
      return g_score[current]
      #return reconstruct_path(came_from, current)
    end

    open_set.delete(current)
    edges(grid, current).each do |neighbor, d|
      tentative_g_score = g_score[current] + d
      if tentative_g_score < g_score[neighbor]
        came_from[neighbor] = current
        g_score[neighbor] = tentative_g_score
        f_score[neighbor] = tentative_g_score + min_cost(neighbor, goal)
        open_set << neighbor
      end
    end
  end

  raise "no path found"
end

def min_cost(from, to)
  dc = (to[1] - from[1]).abs
  dr = (to[0] - from[0]).abs
  dc + dr
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
