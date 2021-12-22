require "set"

def main
  grid = $stdin.readlines.map { _1.strip.chars.map(&:to_i) }
  br = bottom_right_node(grid)
  puts "part 1: #{least_cost(Part1Grid.new(grid), [0,0], br)}"
  puts "part 2: #{least_cost(Part2Grid.new(grid), [0,0], br.map { (_1 + 1) * 5 - 1 })}"
  #Part2Grid.new(grid).draw
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

  priquelol = Hash.new { |h,k| h[k] = [] }
  priquelol[:x] = 0
  priquelol[f_score[start]] << start

  until open_set.empty?
    current = getnext(priquelol, open_set)

    if current == goal
      return g_score[current]
    end

    open_set.delete(current)
    grid.edges(current).each do |neighbor, d|
      tentative_g_score = g_score[current] + d
      if tentative_g_score < g_score[neighbor]
        came_from[neighbor] = current
        g_score[neighbor] = tentative_g_score
        f_score[neighbor] = tentative_g_score + min_cost(neighbor, goal)
        priquelol[f_score[neighbor]] << neighbor
        open_set << neighbor
      end
    end
  end

  raise "no path found"
end

def getnext(pq, os)
  loop do
    x = pq[:x]
    raise "nooo #{pq.select { |k,v| k != :x && !v.empty? }}.inspect" if x > 10_000
    node = pq[x].shift
    if node.nil?
      pq[:x] += 1
      next
    end
    if node && os.include?(node)
      return node
    end
  end
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

class Part1Grid
  def initialize(grid)
    @grid = grid
  end

  def edges(node)
    [
      [0, 1],
      [0, -1],
      [1, 0],
      [-1, 0],
    ].map { |dr, dc|
      [dr + node[0], dc + node[1]]
    }.select { |r, c| r >= 0 && c >= 0 && r < @grid.size && c < @grid[0].size }.map { |r, c|
      [ [r, c], @grid[r][c] ]
    }
  end
end

class Part2Grid
  def initialize(grid)
    @grid = grid
    @tile_rows = @grid.size
    @tile_cols = @grid[0].size
    @total_rows = 5 * @tile_rows
    @total_cols = 5 * @tile_cols
    #p total_rows: @total_rows, total_cols: @total_cols
  end

  def edges(node)
    [
      [0, 1],
      [1, 0],
    ].map { |dr, dc|
      [dr + node[0], dc + node[1]]
    }.select { |r, c|
      r >= 0 && c >= 0 && r < @total_rows && c < @total_cols
    }.map { |r, c|
      [ [r, c], cost_to(r, c) ]
    }
  end

  def draw
    0.upto(@total_rows - 1) do |r|
      0.upto(@total_cols - 1) do |c|
        cost = cost_to(r, c)
        if cost > 9
          raise "unexpected cost #{r},#{c} => #{cost}"
        end
        print cost
      end
      print "\n"
    end
  end

  private

  def cost_to(r, c)
    base_cost = @grid[r % @tile_rows][c % @tile_cols] - 1
    off_r = r / @tile_rows
    off_c = c / @tile_cols
    (base_cost + off_r + off_c) % 9 + 1
  end
end

main
