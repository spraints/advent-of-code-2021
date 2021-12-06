def main
  ft = $stdin.read.strip.split(",").map(&:to_i).tally
  #p ft
  80.times do
    newft = {}
    babies = ft.delete(0) and newft[8] = babies
    ft.each do |age, count|
      newft[age - 1] = count
    end
    newft[6] = (newft[6] || 0) + babies if babies
    ft = newft
    #p ft
  end
  puts "part 1: #{ft.values.sum}"
  (256-80).times do
    newft = {}
    babies = ft.delete(0) and newft[8] = babies
    ft.each do |age, count|
      newft[age - 1] = count
    end
    newft[6] = (newft[6] || 0) + babies if babies
    ft = newft
  end
  puts "part 2: #{ft.values.sum}"
end

main
