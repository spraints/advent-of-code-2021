def main
  filled = Hash.new(0)
  $stdin.readlines.each do |line|
    from, _, to = line.split(" ")
    x1, y1 = from.split(",").map(&:to_i)
    x2, y2 = to.split(",").map(&:to_i)
    if x1 == x2
      low, high = [y1, y2].sort
      low.upto(high) do |y|
        filled[[x1,y]] += 1
      end
    elsif y1 == y2
      low, high = [x1, x2].sort
      low.upto(high) do |x|
        filled[[x,y1]] += 1
      end
    end
  end

  puts "part 1: #{filled.count { |_, n| n > 1 }}"

  #puts "part 1: #{score(*winners.first)}"
  #puts "part 2: #{score(*winners.last)}"
end

main
