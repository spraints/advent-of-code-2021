require "set"

def main
  one, two = $stdin.readlines.map { parse_position _1 }

  rolls, scores = play(positions: [one, two], die: InOrderDie.new(sides: 100), goal: 1000)
  puts "part 1: #{rolls * scores.min}"

  wins_per_player = play2([one, two])
  puts "part 2: #{wins_per_player.max}"
end

def play2(positions)
  # p positions: positions
  universes = {
    {positions: positions, scores: positions.map { 0 }, roll_sum: 0} => 1,
  }
  winners = {0 => 0, 1 => 0}
  turn_no = 0
  player = 0
  until universes.empty?
    turn_no += 1
    #puts "TURN #{turn_no}"
    won, universes = play_turn(universes, player)
    winners[player] += won
    #p winners: winners, n_universes: universes.size
    #break if turn_no >= 10
    player = 1 - player
  end

  winners.values
end

def play_turn(universes, player)
  # Roll 3 times!
  3.times do
    new_universes = Hash.new(0)
    # Split the universe on each die roll!
    (1..3).each do |n|
      universes.each do |u, c|
        new_universes[u.merge(roll_sum: u[:roll_sum] + n)] += c
      end
    end
    universes = new_universes
  end
  # Move the pawn and score it!
  won = 0
  final_universes = Hash.new(0)
  universes.each do |u, c|
    new_u = score(u, player)
    if new_u[:scores][player] > 20
      #p winner: new_u, count: c
      won += c
    else
      final_universes[new_u] += c
    end
  end
  [won, final_universes]
end

def score(universe, player)
  roll_sum = universe[:roll_sum]
  positions = universe[:positions].dup
  positions[player] = update_position(positions[player], roll_sum)
  scores = universe[:scores].dup
  scores[player] += positions[player]
  universe.merge(roll_sum: 0, positions: positions, scores: scores)
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
    positions[i] = update_position(positions[i], move)
    scores[i] += positions[i]
    return [rolls, scores] if scores[i] >= goal
    i = 1 - i
  end
end

def update_position(pos, delta)
  1 + (pos + delta - 1) % 10
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
