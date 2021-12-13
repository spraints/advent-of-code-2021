require "set"

def main
  graph = Hash.new { |h,k| h[k] = [] }
  $stdin.readlines.each do |line|
    a, b = line.strip.split("-", 2)
    graph[a] << b
    graph[b] << a
  end

  puts "part 1: #{count_paths(graph, p: "start")}"
  puts "part 2: #{count_paths(graph, p: "start", double: true)}"
end

def count_paths(graph, p:, visited: Set.new, path: [], double: false)
  path = path + [p]
  visited = visited + [p] if p =~ /[a-z]/

  count = 0
  graph[p].each do |n|
    case
    when n == "end"
      #puts path.join(",") + ",end"
      count += 1
    when n == "start"
      # nothing
    when visited.include?(n)
      if double
        count += count_paths(graph, p: n, visited: visited, path: path)
      end
    else
      count += count_paths(graph, p: n, visited: visited, path: path, double: double)
    end
  end

  count
end

main
