def main
  filled = Hash.new(0)
  skipped = []
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
    else
      skipped << [x1, y1, x2, y2]
    end
  end

  puts "part 1: #{filled.count { |_, n| n > 1 }}"

  skipped.each do |x1, y1, x2, y2|
    x1, y1, x2, y2 = x2, y2, x1, y1 if x1 > x2
    m = (y2 - y1) / (x2 - x1)
    y = y1
    x1.upto(x2) do |x|
      filled[[x,y]] += 1
      y += m
    end
  end

  puts "part 2: #{filled.count { |_, n| n > 1 }}"
end

main
