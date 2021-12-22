require "set"

def main
  one, two = $stdin.readlines.map { parse_position _1 }

  #rolls, scores = play(positions: [one, two], die: InOrderDie.new(sides: 100), goal: 1000)
  #puts "part 1: #{rolls * scores.min}"

  wins_per_player = play2([one, two])
  puts "part 2: #{wins_per_player.max}"
end

def play2(positions)
  roll_universes = []
  roll_turns 3, [[]]
  roll_turns2 3, {{positions: positions, scores: positions.map { 0 }, roll_sum: 0} => 1}

  positions # todo
end

def roll_turns2(max_rolls, universes)
  max_rolls.times do |i|
    universes = roll_turn2(universes)
    puts "After turn #{i+1}:", universes.map(&:inspect)
  end
end

def roll_turn2(universes)
  new_universes = Hash.new(0)
  universes.each do |u, c|
    (1..3).each do |i|
      new_universes[u.merge(roll_sum: u[:roll_sum] + i)] += c
    end
  end
  new_universes
end

def roll_turns(max_rolls, universes, roll: 0)
  return unless roll < max_rolls

  roll += 1
  new_universes = universes.flat_map { |u| (1..3).map { |r| u + [r] } }

  puts "After roll #{roll}:"
  #puts new_universes.map { |u| u.join(",") }
  puts new_universes.map { _1.sum }.tally.sort_by { _1 }.map { |score, count| sprintf "%4d: %3d times", score, count }

  roll_turns(max_rolls, new_universes, roll: roll)
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
