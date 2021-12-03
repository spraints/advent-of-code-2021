numbers = readlines.map(&:strip)
nchars = numbers.map(&:chars)
bit_counts = nchars.shift.zip(*nchars).map(&:tally).map { |c| [c["0"], c["1"]] }

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
