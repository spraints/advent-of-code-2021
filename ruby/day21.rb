require "set"

class Discard
  def puts(*) end
end

# $dbg = File.open("dbg", "w")
$dbg = Discard.new

def main
  one, two = $stdin.readlines.map { parse_position _1 }
  rolls, scores = play([one, two], InOrderDie.new(sides: 100))
  puts "part 1: #{rolls * scores.min}"
end

def parse_position(line)
  line =~ /Player (\d) starting position: (\d)/ or raise "could not parse #{line.inspect}"
  $2.to_i
end

def play(positions, die)
  scores = positions.map { 0 }
  rolls = 0
  i = 0
  loop do
    rolls += 3
    move = die.roll + die.roll + die.roll
    positions[i] = 1 + (positions[i] + move - 1) % 10
    scores[i] += positions[i]
    $dbg.puts "Player #{i+1} rolls #{move} and moves to space #{positions[i]} for a total score of #{scores[i]}"
    return [rolls, scores] if scores[i] >= 1000
    i = 1 - i
  end
end

class InOrderDie
  def initialize(sides:)
    @sides = sides
    @next = 1
  end

  def roll
    v = @next
    @next = (@next % @sides) + 1
    v
  end
end

main
