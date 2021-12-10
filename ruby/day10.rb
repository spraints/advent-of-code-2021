require "set"

def main
  score = 0
  $stdin.readlines.each do |line|
    score += analyze(line.strip)
  end
  puts "part 1: #{score}"
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
        return SCORE.fetch(c)
      end
    end
  end
  0
end

main
