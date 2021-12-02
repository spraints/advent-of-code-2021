script = $stdin.read

$dist = 0
$aim = 0
$depth = 0
def forward(n)
  $dist += n
  $depth += $aim*n
end

def down(n)
  $aim += n
end

def up(n)
  $aim -= n
end

eval script
puts "part 1: #{$aim * $dist}"
puts "part 2: #{$depth * $dist}"
