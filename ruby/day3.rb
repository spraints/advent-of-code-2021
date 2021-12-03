bit_counts = []
numbers = []

readlines(chomp:true).each do |line|
  numbers << line
  line.each_char.with_index do |c, i|
    bit_counts[i] ||= [0,0]
    case c
    when "0"
      bit_counts[i][0] += 1
    when "1"
      bit_counts[i][1] += 1
    else
      raise "boom #{line.inspect} #{c.inspect}/#{i}"
    end
  end
end

gamma = bit_counts.map { |a,b| a > b ? 0 : 1 }.inject { |n, b| n*2 + b }
epsilon = bit_counts.map { |a,b| a > b ? 1 : 0 }.inject { |n, b| n*2 + b }
puts "part 1: #{gamma * epsilon}"

def search(numbers)
  i = 0
  while numbers.size > 1
    zeros, ones = numbers.partition { |n| n[i] == "0" }
    numbers = yield(zeros, ones)
    i += 1
  end
  numbers.first
end

o2  = search(numbers) { |z, o| z.size > o.size ? z : o }
co2 = search(numbers) { |z, o| z.size > o.size ? o : z }

puts "part 2: #{o2.to_i(2) * co2.to_i(2)}"
