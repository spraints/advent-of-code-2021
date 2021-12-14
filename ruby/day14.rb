require "set"

def main
  init, rules = $stdin.read.split("\n\n")
  counts = init.chars.each_cons(2).map(&:join).tally
  ecounts = init.chars.tally
  rules = rules.lines.map { |line| parse_line(line.strip) }.to_h
  #puts init
  #p counts
  #p ecounts
  #p rules
  10.times do
    counts = update(counts, ecounts, rules)
    #p counts
  end
  #return
  res = ecounts.values.sort
  puts "part 1: #{res.last - res.first}"
  30.times do |i|
    puts i
    counts = update(counts, ecounts, rules)
  end
  res = ecounts.values.sort
  puts "part 2: #{res.last - res.first}"
end

def update(counts, ecounts, rules)
  newcounts = Hash.new(0)
  counts.each do |pair, count|
    if ins = rules[pair]
      newcounts[ins[0]] += count
      newcounts[ins[1]] += count
      ecounts[ins[2]] = (ecounts[ins[2]] || 0) + count
    else
      newcounts[pair] = count
    end
  end
  newcounts
end

def parse_line(line)
  pair, add = line.split(" -> ")
  a, b = pair.chars
  [pair, ["#{a}#{add}", "#{add}#{b}", add]]
end

main
