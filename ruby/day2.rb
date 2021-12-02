script = $stdin.read

$depth = 0
$dist = 0
def forward(n)
  $dist += n
end

def down(n)
  $depth += n
end

def up(n)
  $depth -= n
end

eval script
puts $depth * $dist
