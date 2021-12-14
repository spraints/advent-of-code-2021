require "set"

def main
  init, rules = $stdin.read.split("\n\n")
  seq = ll(init.chars)
  rules = rules.lines.map { |line| parse_line(line.strip) }.to_h
  10.times do
    seq = update(seq, rules)
  end
  counts = seq.tally.values.sort
  puts "part 1: #{counts.last - counts.first}"
  30.times do |i|
    puts i
    seq = update(seq, rules)
  end
  counts= seq.tally.values.sort
  puts "part 2: #{counts.last - counts.first}"
end

def ll(a)
  a.reverse.inject(nil) { |next_item, c| [c, next_item] }
end

def update(seq, rules)
  n = 0
  p = seq
  while p
    q = p[1]
    if q && ins = rules[[p[0], q[0]]]
      n += 1
      p[1] = [ins, q]
    end
    p = q
  end
  puts "#{n} new items"
  seq
end

def parse_line(line)
  pair, add = line.split(" -> ")
  [pair.chars, add]
end

main
