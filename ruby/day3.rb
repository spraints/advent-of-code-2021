reports = readlines.map(&:strip).map { |s| s.split("").map(&:to_i) }

a = reports.shift
mask = a.inject(0) { |m, _| m*2+1 }
gamma = a.zip(*reports).map { |d| d.count { |bit| bit == 1 } > d.size/2 ? 1 : 0 }
gamma = gamma.inject { |n, b| (n*2)+b }
epsilon = gamma ^ mask

puts gamma * epsilon
# not 14375442
