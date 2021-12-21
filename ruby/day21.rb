require "set"

def main
  one, two = $stdin.readlines.map { parse_position _1 }

  #rolls, scores = play(positions: [one, two], die: InOrderDie.new(sides: 100), goal: 1000)
  #puts "part 1: #{rolls * scores.min}"

  wins_per_player = play2([one, two])
  puts "part 2: #{wins_per_player.max}"
end

def play2(positions)
  play_all_alternates \
    positions: positions,
    scores: positions.map { 0 },
    turn: 0,
    rolls: []
end

$x = 0
def play_all_alternates(positions:, scores:, turn:, rolls:)
  return [0,0] if $x > 100
  $x += 1

  positions = positions.dup
  wins = []
  (1..3).each do |n|
    rolls = rolls + [n]
    if rolls.len == 3
      todo!
      rolls = []
    end
    if scores.any? { |s| s >= 21 }
      todo!
    end
    wins << play_all_alternates(positions: positions, scores: scores, turn: turn, rolls: rolls)
  end
end

def parse_position(line)
  line =~ /Player (\d) starting position: (\d)/ or raise "could not parse #{line.inspect}"
  $2.to_i
end

def play(positions:, die:, goal:)
  scores = positions.map { 0 }
  rolls = 0
  i = 0
  loop do
    rolls += 3
    move = die.roll + die.roll + die.roll
    positions[i] = 1 + (positions[i] + move - 1) % 10
    scores[i] += positions[i]
    return [rolls, scores] if scores[i] >= goal
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
