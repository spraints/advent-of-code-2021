require "set"

def main
  graph = Hash.new { |h,k| h[k] = [] }
  $stdin.readlines.each do |line|
    a, b = line.strip.split("-", 2)
    graph[a] << b
    graph[b] << a
  end

  puts "part 1: #{count_paths(graph, visited: [], p: "start")}"
end

def count_paths(graph, visited:, p:)
  visited = visited + [p] if p =~ /[a-z]/
  count = 0
  (graph[p] - visited).each do |n|
    if n == "end"
      count += 1
    else
      count += count_paths(graph, visited: visited, p: n)
    end
  end
  count
end

main
