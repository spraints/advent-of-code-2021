reports = readlines.map(&:strip).map { |s| s.split("").map(&:to_i) }
r2 = reports.dup

def dec(n)
  n.inject { |n, b| (n*2)+b }
end

a = reports.shift
mask = a.inject(0) { |m, _| m*2+1 }
gamma = a.zip(*reports).map { |d| d.count { |bit| bit == 1 } > d.size/2 ? 1 : 0 }
gamma = dec(gamma)
epsilon = gamma ^ mask

puts gamma * epsilon
# not 14375442

def apply_bit_criteria(ar, i)
  zeros, ones = ar.partition { |n| n[i] == 0 }
  if zeros.size > ones.size
    [zeros, ones]
  else
    [ones, zeros]
  end
end

o2 = r2
co2 = r2
r2.first.each_with_index do |_, i|
  o2, _ = apply_bit_criteria(o2, i) unless o2.size == 1
  _, co2 = apply_bit_criteria(co2, i) unless co2.size == 1
end

p o2: o2, co2: co2
o2 = dec(o2.first)
co2 = dec(co2.first)
p o2*co2
