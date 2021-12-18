require "set"

def main
  lines = $stdin.readlines.map { |l| eval(l) }

  res = lines.inject { |a, b| reduce([a,b]) }
  puts "part 1: #{mag(res)}"

  max = 0
  (0..lines.size-1).each do |i|
    (0..lines.size-1).each do |j|
      next if i == j
      m = mag(reduce([lines[i], lines[j]]))
      max = m if m > max
    end
  end
  puts "part 2: #{max}"
end

def mag(sn)
  case sn
  when Array
    a, b = sn
    3*mag(a) + 2*mag(b)
  else
    sn
  end
end

def reduce(sn)
  sn = convert(sn)
  loop do
    next if explode(sn)
    next if split(sn)
    return unconvert(sn)
  end
end

def convert(sn, depth: 0, res: [])
  case sn
  when Array
    convert(sn[0], depth: depth+1, res: res)
    convert(sn[1], depth: depth+1, res: res)
  else
    res << [sn, depth]
  end
  res
end

def unconvert(sn)
  res = [[]]
  while n = sn.shift
    while n[1] < res.size
      loop do
        a = res.pop
        res.last.push(a)
        break unless res.last.size == 2
      end
    end
    while n[1] > res.size
      res.push([])
    end
    res.last.push(n[0])
    while res.last.size == 2 && res.size > 1
      a = res.pop
      res.last.push(a)
    end
  end
  until res.size == 1
    a = res.pop
    res.last.push(a)
  end
  res.pop
end

def explode(sn)
  (0..sn.size-1).each do |i|
    if sn[i][1] > 4 && sn[i+1][1] == sn[i][1]
      p1 = sn[i]
      p2 = sn.delete_at(i+1)
      sn[i] = [0, p1[1]-1]
      if i > 0
        n1 = sn[i-1]
        n1[0] += p1[0]
      end
      if n2 = sn[i+1]
        n2[0] += p2[0]
      end
      return true
    end
  end
  false
end

def split(sn)
  i = sn.index { |x,d| x >= 10 }
  return false if i.nil?
  x, d = sn[i]
  xa = x/2
  xb = x-xa
  sn[i] = [xa, d+1]
  sn.insert(i+1, [xb, d+1])
  true
end

main
