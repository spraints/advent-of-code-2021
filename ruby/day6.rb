def main
  fishes = $stdin.read.strip.split(",").map(&:to_i)
  80.times do
    new_fishes = fishes.select { |f| f == 0 }.map { 8 }
    fishes = fishes.map { _1 == 0 ? 6 : _1 - 1}
    fishes += new_fishes
  end
  puts "part 1: #{fishes.size}"
  (256-80).times do
    new_fishes = fishes.select { |f| f == 0 }.map { 8 }
    fishes = fishes.map { _1 == 0 ? 6 : _1 - 1}
    fishes += new_fishes
  end
  puts "part 2: #{fishes.size}"
end

main
