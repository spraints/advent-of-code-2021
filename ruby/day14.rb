require "set"

def main
  init, rules = $stdin.read.split("\n\n")
  seq = init.chars
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

def update(seq, rules)
  newseq = []
  last = ""
  seq.each do |c|
    if ins = rules[[last,c]]
      newseq << ins
    end
    newseq << c
    last = c
  end
  newseq
end

def parse_line(line)
  pair, add = line.split(" -> ")
  [pair.chars, add]
end

main
