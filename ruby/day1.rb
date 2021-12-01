nums = readlines.map(&:to_i)
puts "part 1: #{nums.each_cons(2).count { |a| a.last > a.first }}"
puts "part 2: #{nums.each_cons(4).count { |a| a.last > a.first }}"
