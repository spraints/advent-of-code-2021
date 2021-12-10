require "set"

def main
  incomplete = []
  score = 0
  $stdin.readlines.each do |line|
    stack, n = analyze(line.strip)
    score += n
    incomplete << stack if n == 0
  end
  puts "part 1: #{score}"
  incomplete_scores = incomplete.map { |stack| finish_and_score(stack) }.sort
  puts "part 2: #{incomplete_scores[incomplete_scores.size / 2]}"
end

CLOSE_OPEN = {
  "}" => "{",
  "]" => "[",
  ")" => "(",
  ">" => "<",
}

SCORE = {
  "}" => 1197,
  "]" => 57,
  ")" => 3,
  ">" => 25137,
}

def analyze(line)
  stack = []
  line.chars.each do |c|
    case c
    when '{', '(', '[', '<'
      stack.push c
    else
      if stack.empty?
        raise "can't close with #{c} (#{line.inspect})"
      end
      if CLOSE_OPEN.fetch(c) != stack.pop
        return [stack, SCORE.fetch(c)]
      end
    end
  end
  [stack, 0]
end

AC_SCORE = {
  "(" => 1,
  "[" => 2,
  "{" => 3,
  "<" => 4,
}

def finish_and_score(stack)
  score = 0
  until stack.empty?
    score = score*5 + AC_SCORE.fetch(stack.pop)
  end
  score
end

main
