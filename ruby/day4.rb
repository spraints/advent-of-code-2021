def main
  input = $stdin.read
  draws, *boards = input.split("\n\n")

  boards = boards.map { |raw| make_board(raw) }

  winners = []
  draws.split(",").each do |n|
    n = n.to_i
    call(boards, n)
    w, boards = boards.partition { |board| winner?(board) }
    w.each do |board|
      winners << [board, n]
    end
  end

  puts "part 1: #{score(*winners.first)}"
  puts "part 2: #{score(*winners.last)}"
end

def score(board, n)
  unused = board[:tiles].reject { |n, tile| tile[:flipped] }.inject(0) { |sum, (n, _)| sum + n }
  unused * n
end

def call(boards, n)
  boards.each do |board|
    if tile = board[:tiles][n]
      tile[:flipped] = true
    end
  end
end

def pretty_board(board)
  board[:board].each do |row|
    puts row.map { |n| board[:tiles][n][:flipped] ? "X" : "O" }.join(" ")
  end
  puts "--"
end

def winner?(board)
  board[:board].each do |row|
    return true if row.all? { |n| board[:tiles][n][:flipped] }
  end
  (0..(board[:board][0].size-1)).each do |j|
    return true if board[:board].all? { |row| board[:tiles][row[j]][:flipped] }
  end
  false
end

def make_board(raw)
  tiles = {}
  board = []
  raw.lines.each_with_index do |line, i|
    row = []
    line.strip.split(/\s+/).each_with_index do |n, j|
      n = n.to_i
      row << n
      tiles[n] = {row: i, col: j}
    end
    board << row
  end
  {tiles: tiles, board: board}
end

main
